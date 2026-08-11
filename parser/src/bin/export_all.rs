use bitbuffer::BitRead;
use main_error::MainError;
use serde_json::json;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::parser::{DemoHandler, RawPacketStream};
use tf_demo_parser::Demo;

mod animation_export;
use animation_export::AnimationExporter;

fn usage(program: &str) {
    eprintln!(
        "Usage:\n  {program} <input.dem> <output_directory>\n\n\
         Example:\n  {program} \"C:\\Demos\\match.dem\" \"C:\\Demos\\match_export\""
    );
}

fn main() -> Result<(), MainError> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "export_all".to_string());

    let Some(input_arg) = args.next() else {
        usage(&program);
        std::process::exit(2);
    };
    let Some(output_arg) = args.next() else {
        usage(&program);
        std::process::exit(2);
    };
    if args.next().is_some() {
        usage(&program);
        std::process::exit(2);
    }

    let input_path = PathBuf::from(input_arg);
    let output_dir = PathBuf::from(output_arg);

    if !input_path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Demo file not found: {}", input_path.display()),
        )
        .into());
    }

    fs::create_dir_all(&output_dir)?;

    let bytes = fs::read(&input_path)?;
    let demo = Demo::new(&bytes);
    let mut stream = demo.get_stream();

    // The header is outside the normal packet stream.
    let header = Header::read(&mut stream)?;

    let header_file = File::create(output_dir.join("header.json"))?;
    serde_json::to_writer_pretty(BufWriter::new(header_file), &header)?;

    // DemoHandler::default() configures the parser to decode all supported
    // message types while maintaining send-table, baseline, string-table,
    // event-definition, and entity-class state for subsequent packets.
    let mut handler = DemoHandler::default();
    handler.handle_header(&header);

    let mut packet_stream = RawPacketStream::new(stream);
    let mut packets_out = BufWriter::new(File::create(output_dir.join("packets.ndjson"))?);
    let mut index_out = BufWriter::new(File::create(output_dir.join("packet_index.ndjson"))?);
    let mut animation_out = AnimationExporter::new(&output_dir)?;

    let mut packet_count: u64 = 0;

    loop {
        // RawPacketStream positions are bit positions in the original demo stream.
        let start_bit = packet_stream.pos();

        let Some(packet) = packet_stream.next(&handler.state_handler)? else {
            break;
        };

        let end_bit = packet_stream.pos();
        let tick = packet.tick();
        let packet_type = packet.packet_type().as_lowercase_str();

        // Write the complete decoded packet as one independent JSON line.
        // This streams to disk and does not keep the full match in memory.
        serde_json::to_writer(&mut packets_out, &packet)?;
        packets_out.write_all(b"\n")?;

        // Write a small seek/index record separately.
        serde_json::to_writer(
            &mut index_out,
            &json!({
                "sequence": packet_count,
                "tick": tick,
                "packet_type": packet_type,
                "start_bit": start_bit,
                "end_bit": end_bit,
                "encoded_bit_length": end_bit.saturating_sub(start_bit)
            }),
        )?;
        index_out.write_all(b"\n")?;

        // Resolve the player state and animation events against the state that
        // existed immediately before this packet. Enter updates use the parser's
        // static/instance baselines; delta updates are merged by entity lifetime.
        animation_out.observe(&packet, &handler.state_handler)?;

        // Apply this packet to parser state so later delta-compressed packets,
        // baselines, event definitions, and string-table updates decode correctly.
        handler.handle_packet(packet)?;

        packet_count += 1;
    }

    packets_out.flush()?;
    index_out.flush()?;
    let animation_stats = animation_out.finish()?;

    let manifest = json!({
        "format": "tf-demo-parser-decoded-packet-stream",
        "format_version": 1,
        "source_demo": input_path.to_string_lossy(),
        "packet_count": packet_count,
        "parser_reported_incomplete": packet_stream.incomplete,
        "animation_export": {
            "logical_frames": animation_stats.logical_frames,
            "player_samples": animation_stats.player_samples,
            "animation_events": animation_stats.animation_events,
            "interval_per_tick": animation_stats.interval_per_tick
        },
        "files": {
            "header": "header.json",
            "packets": "packets.ndjson",
            "packet_index": "packet_index.ndjson",
            "animation_inputs": "animation_inputs.ndjson"
        },
        "notes": [
            "packets.ndjson contains one complete parser-decoded top-level demo packet per line",
            "packet_index.ndjson contains packet order, tick, type, and original stream bit ranges",
            "animation_inputs.ndjson contains resolved per-player state and decoded player animation events",
            "keep the original .dem file as the bit-exact source archive"
        ]
    });

    let manifest_file = File::create(output_dir.join("manifest.json"))?;
    serde_json::to_writer_pretty(BufWriter::new(manifest_file), &manifest)?;

    if packet_stream.incomplete {
        eprintln!(
            "Export finished, but the parser reported that the demo ended with incomplete data."
        );
    }

    println!(
        "Exported {packet_count} packets to {}",
        output_dir.display()
    );

    Ok(())
}
