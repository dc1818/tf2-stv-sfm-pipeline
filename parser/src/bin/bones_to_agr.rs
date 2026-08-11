use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

#[derive(Default)]
struct Group {
    frame: u64,
    demo_tick: u64,
    time: f64,
    visible: BTreeSet<u64>,
    models: BTreeMap<u64, Value>,
}

struct AgrWriter<W: Write> {
    out: W,
    dictionary: HashMap<String, i32>,
    previous_visible: BTreeSet<u64>,
}

impl<W: Write> AgrWriter<W> {
    fn new(mut out: W) -> io::Result<Self> {
        out.write_all(b"afxGameRecord\0")?;
        out.write_all(&6_i32.to_le_bytes())?;
        Ok(Self {
            out,
            dictionary: HashMap::new(),
            previous_visible: BTreeSet::new(),
        })
    }

    fn token(&mut self, value: &str) -> io::Result<()> {
        if let Some(index) = self.dictionary.get(value) {
            self.out.write_all(&index.to_le_bytes())
        } else {
            self.out.write_all(&(-1_i32).to_le_bytes())?;
            self.out.write_all(value.as_bytes())?;
            self.out.write_all(&[0])?;
            let index = self.dictionary.len() as i32;
            self.dictionary.insert(value.to_owned(), index);
            Ok(())
        }
    }

    fn int(&mut self, value: i32) -> io::Result<()> {
        self.out.write_all(&value.to_le_bytes())
    }

    fn float(&mut self, value: f32) -> io::Result<()> {
        self.out.write_all(&value.to_le_bytes())
    }

    fn boolean(&mut self, value: bool) -> io::Result<()> {
        self.out.write_all(&[u8::from(value)])
    }

    fn matrix(&mut self, value: &Value, context: &str) -> io::Result<()> {
        let array = value
            .as_array()
            .ok_or_else(|| invalid(format!("{context} is not an array")))?;
        if array.len() != 12 {
            return Err(invalid(format!(
                "{context} has {} values instead of 12",
                array.len()
            )));
        }
        for (index, number) in array.iter().enumerate() {
            let number = number
                .as_f64()
                .filter(|value| value.is_finite())
                .ok_or_else(|| invalid(format!("{context}[{index}] is not finite")))?;
            self.float(number as f32)?;
        }
        Ok(())
    }

    fn write_group(&mut self, group: &Group, duration: f32) -> io::Result<()> {
        self.token("afxFrame")?;
        self.float(duration.max(0.000_001))?;
        self.int(0)?; // No forward hidden-list offset; deleted packets below handle hiding.

        let hidden: Vec<u64> = self
            .previous_visible
            .difference(&group.visible)
            .copied()
            .collect();
        for key in hidden {
            self.token("deleted")?;
            self.int(agr_handle(key))?;
        }

        for (key, frame) in &group.models {
            if !group.visible.contains(key) {
                continue;
            }
            self.token("entity_state")?;
            self.int(agr_handle(*key))?;
            self.token("baseentity")?;
            let model = frame
                .get("model")
                .and_then(Value::as_str)
                .ok_or_else(|| invalid("frame is missing model"))?;
            self.token(model)?;
            self.boolean(true)?;
            self.matrix(
                frame
                    .get("render_matrix")
                    .ok_or_else(|| invalid("frame is missing render_matrix"))?,
                "render_matrix",
            )?;

            self.token("baseanimating")?;
            self.boolean(true)?;
            let bones = frame
                .get("bones")
                .and_then(Value::as_array)
                .ok_or_else(|| invalid("frame is missing bones"))?;
            self.int(i32::try_from(bones.len()).map_err(|_| invalid("too many bones"))?)?;
            for bone in bones {
                self.matrix(
                    bone.get("l")
                        .ok_or_else(|| invalid("bone is missing local matrix l"))?,
                    "bone local matrix",
                )?;
            }
            self.token("/")?;
            self.boolean(false)?; // Not a view-model.
        }

        self.token("afxFrameEnd")?;
        self.previous_visible = group.visible.clone();
        Ok(())
    }

    fn finish(mut self) -> io::Result<()> {
        self.out.flush()
    }
}

fn agr_handle(key: u64) -> i32 {
    // AGR treats this only as an identity key; signed wrapping is safe and stable.
    key as u32 as i32
}

fn entity_key(record: &Value) -> io::Result<u64> {
    let entity = record
        .get("entity_index")
        .and_then(Value::as_u64)
        .ok_or_else(|| invalid("record is missing entity_index"))?;
    let serial = record
        .get("serial")
        .and_then(Value::as_u64)
        .ok_or_else(|| invalid("record is missing serial"))?;
    Ok((serial << 11) | (entity & 0x7ff))
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn usage(program: &str) {
    eprintln!(
        "Usage: {program} <bones.ndjson> <output.agr> [--fps 30] [--start seconds --end seconds] [--start-demo-tick tick --end-demo-tick tick] [--trusted-complete]\n\
         Seconds are relative to the first captured timeline frame. Use --fps 0 to keep every worker frame.\n\
         --trusted-complete stops after the requested range; use it only after validating bones.ndjson."
    );
}

fn main() -> io::Result<()> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "bones_to_agr".to_owned());
    let Some(input) = args.next() else {
        usage(&program);
        std::process::exit(2)
    };
    let Some(output) = args.next() else {
        usage(&program);
        std::process::exit(2)
    };
    let mut fps = 30.0_f64;
    let mut clip_start = 0.0_f64;
    let mut clip_duration: Option<f64> = None;
    let mut clip_end: Option<f64> = None;
    let mut start_demo_tick: Option<u64> = None;
    let mut end_demo_tick: Option<u64> = None;
    let mut trusted_complete = false;
    while let Some(option) = args.next() {
        if option == "--trusted-complete" {
            trusted_complete = true;
            continue;
        }
        if option == "--start-demo-tick" || option == "--end-demo-tick" {
            let value = args
                .next()
                .ok_or_else(|| invalid(format!("{option} requires a whole-number tick")))?
                .parse::<u64>()
                .map_err(|_| invalid(format!("invalid tick value for {option}")))?;
            if option == "--start-demo-tick" {
                start_demo_tick = Some(value);
            } else {
                end_demo_tick = Some(value);
            }
            continue;
        }
        let value = args
            .next()
            .ok_or_else(|| invalid(format!("{option} requires a number")))?
            .parse::<f64>()
            .map_err(|_| invalid(format!("invalid value for {option}")))?;
        match option.as_str() {
            "--fps" => {
                if value < 0.0 || !value.is_finite() {
                    return Err(invalid("--fps must be zero or a positive finite number"));
                }
                fps = value;
            }
            "--start" => {
                if value < 0.0 || !value.is_finite() {
                    return Err(invalid("--start must be a zero or positive finite number"));
                }
                clip_start = value;
            }
            "--duration" => {
                if value <= 0.0 || !value.is_finite() {
                    return Err(invalid("--duration must be a positive finite number"));
                }
                clip_duration = Some(value);
            }
            "--end" => {
                if value < 0.0 || !value.is_finite() {
                    return Err(invalid("--end must be a zero or positive finite number"));
                }
                clip_end = Some(value);
            }
            _ => {
                usage(&program);
                std::process::exit(2);
            }
        }
    }
    if clip_duration.is_some() && clip_end.is_some() {
        return Err(invalid("use either --end or --duration, not both"));
    }
    if let Some(duration) = clip_duration {
        clip_end = Some(clip_start + duration);
    }
    if let Some(end) = clip_end {
        if end <= clip_start {
            return Err(invalid("--end must be later than --start"));
        }
    }
    let use_tick_range = start_demo_tick.is_some() || end_demo_tick.is_some();
    if use_tick_range && (start_demo_tick.is_none() || end_demo_tick.is_none()) {
        return Err(invalid("demo-tick clips require both --start-demo-tick and --end-demo-tick"));
    }
    if use_tick_range && start_demo_tick >= end_demo_tick {
        return Err(invalid("--end-demo-tick must be greater than --start-demo-tick"));
    }

    let input_path = PathBuf::from(input);
    let output_path = PathBuf::from(output);
    let reader = BufReader::new(File::open(&input_path)?);
    let mut agr = AgrWriter::new(BufWriter::new(File::create(&output_path)?))?;

    let mut current: Option<Group> = None;
    let mut pending: Option<Group> = None;
    let mut next_sample_time = if fps > 0.0 && !use_tick_range {
        Some(clip_start)
    } else {
        None
    };
    let mut last_duration = if fps > 0.0 { 1.0 / fps } else { 1.0 / 66.0 };
    let mut saw_metadata = false;
    let mut saw_complete = false;
    let mut written = 0_u64;

    let select_group = |group: Group,
                        pending: &mut Option<Group>,
                        next_sample_time: &mut Option<f64>,
                        agr: &mut AgrWriter<BufWriter<File>>,
                        last_duration: &mut f64,
                        written: &mut u64,
                        clip_start: f64,
                        clip_end: Option<f64>,
                        start_demo_tick: Option<u64>,
                        end_demo_tick: Option<u64>,
                        timeline_origin: f64|
     -> io::Result<()> {
        if let (Some(start), Some(end)) = (start_demo_tick, end_demo_tick) {
            if group.demo_tick < start || group.demo_tick >= end {
                return Ok(());
            }
        } else {
            let relative_time = group.time - timeline_origin;
            if relative_time + 0.000_001 < clip_start {
                return Ok(());
            }
            if let Some(end) = clip_end {
                if relative_time >= end - 0.000_001 {
                    return Ok(());
                }
            }
        }
        if fps > 0.0 {
            let sample_time = if start_demo_tick.is_some() {
                group.time
            } else {
                group.time - timeline_origin
            };
            let target = next_sample_time.get_or_insert(group.time);
            if sample_time + 0.000_001 < *target {
                return Ok(());
            }
            while *target <= sample_time + 0.000_001 {
                *target += 1.0 / fps;
            }
        }
        if let Some(previous) = pending.replace(group) {
            let duration = pending.as_ref().unwrap().time - previous.time;
            if duration > 0.0 && duration.is_finite() {
                *last_duration = duration;
            }
            agr.write_group(&previous, *last_duration as f32)?;
            *written += 1;
        }
        Ok(())
    };

    let mut timeline_origin: Option<f64> = None;

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let record: Value = serde_json::from_str(&line)
            .map_err(|error| invalid(format!("line {}: {error}", line_number + 1)))?;
        match record.get("type").and_then(Value::as_str).unwrap_or("") {
            "metadata" => {
                saw_metadata = record.get("format").and_then(Value::as_str)
                    == Some("tf2-final-bones")
                    && record
                        .get("format_version")
                        .and_then(Value::as_u64)
                        .unwrap_or(0)
                        >= 3;
            }
            "timeline" => {
                if let Some(group) = current.take() {
                    let fallback_time = group.time;
                    select_group(
                        group,
                        &mut pending,
                        &mut next_sample_time,
                        &mut agr,
                        &mut last_duration,
                        &mut written,
                        clip_start,
                        clip_end,
                        start_demo_tick,
                        end_demo_tick,
                        timeline_origin.unwrap_or(fallback_time),
                    )?;
                }
                let time = record
                    .get("time")
                    .and_then(Value::as_f64)
                    .ok_or_else(|| invalid("timeline is missing time"))?;
                let origin = *timeline_origin.get_or_insert(time);
                let demo_tick = record.get("demo_tick").and_then(Value::as_u64).unwrap_or(0);
                if trusted_complete
                    && ((use_tick_range && end_demo_tick.is_some_and(|end| demo_tick >= end))
                        || (!use_tick_range
                            && clip_end.is_some_and(|end| time - origin >= end)))
                {
                    break;
                }
                current = Some(Group {
                    frame: record.get("frame").and_then(Value::as_u64).unwrap_or(0),
                    demo_tick,
                    time,
                    ..Group::default()
                });
            }
            "visibility" => {
                let group = current
                    .as_mut()
                    .ok_or_else(|| invalid("visibility before timeline"))?;
                if record.get("frame").and_then(Value::as_u64) != Some(group.frame) {
                    return Err(invalid("visibility frame does not match timeline"));
                }
                if record
                    .get("visible")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                {
                    group.visible.insert(entity_key(&record)?);
                }
            }
            "frame" => {
                let group = current
                    .as_mut()
                    .ok_or_else(|| invalid("bone frame before timeline"))?;
                if record.get("frame").and_then(Value::as_u64) != Some(group.frame) {
                    return Err(invalid("bone frame does not match timeline"));
                }
                group.models.insert(entity_key(&record)?, record);
            }
            "complete" => {
                saw_complete = record.get("failures").and_then(Value::as_u64) == Some(0);
            }
            "skeleton" => {}
            other => return Err(invalid(format!("unknown bones record type {other:?}"))),
        }
    }

    if let Some(group) = current.take() {
        let fallback_time = group.time;
        select_group(
            group,
            &mut pending,
            &mut next_sample_time,
            &mut agr,
            &mut last_duration,
            &mut written,
            clip_start,
            clip_end,
            start_demo_tick,
            end_demo_tick,
            timeline_origin.unwrap_or(fallback_time),
        )?;
    }
    if let Some(group) = pending.take() {
        agr.write_group(&group, last_duration as f32)?;
        written += 1;
    }

    if !saw_metadata {
        return Err(invalid(
            "bones file metadata is missing or older than format version 3",
        ));
    }
    if !saw_complete && !trusted_complete {
        return Err(invalid("bones file has no successful complete record"));
    }
    if written == 0 {
        if clip_end.is_some() || use_tick_range {
            return Err(invalid("the requested clip time range contained no timeline frames"));
        }
        return Err(invalid("bones file contained no timeline frames"));
    }
    agr.finish()?;
    if let (Some(start), Some(end)) = (start_demo_tick, end_demo_tick) {
        println!(
            "Wrote {written} SFM frames for demo ticks {start} through {end} to {}",
            output_path.display()
        );
    } else if let Some(end) = clip_end {
        println!(
            "Wrote {written} SFM frames for relative {:.3}s through {:.3}s to {}",
            clip_start,
            end,
            output_path.display()
        );
    } else {
        println!("Wrote {written} SFM frames to {}", output_path.display());
    }
    Ok(())
}
