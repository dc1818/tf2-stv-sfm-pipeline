use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tf_demo_parser::demo::data::{DemoTick, ServerTick};
use tf_demo_parser::demo::message::{Message, PacketEntity, UpdateType};
use tf_demo_parser::demo::packet::datatable::ClassId;
use tf_demo_parser::demo::packet::Packet;
use tf_demo_parser::demo::sendprop::{SendProp, SendPropIdentifier, SendPropValue};
use tf_demo_parser::ParserState;

const ENTITY_INDEX_MASK: u64 = (1 << 11) - 1;

#[derive(Clone)]
struct ResolvedEntity {
    class_id: ClassId,
    class_name: String,
    serial: u32,
    in_pvs: bool,
    props: HashMap<SendPropIdentifier, SendProp>,
}

impl ResolvedEntity {
    fn apply(&mut self, props: impl IntoIterator<Item = SendProp>) {
        for prop in props {
            self.props.insert(prop.identifier, prop);
        }
    }

    fn value(&self, table: &str, name: &str) -> Option<&SendPropValue> {
        self.props
            .get(&SendPropIdentifier::new(table, name))
            .map(|prop| &prop.value)
    }

    fn number(&self, table: &str, name: &str) -> Option<f32> {
        match self.value(table, name) {
            Some(SendPropValue::Float(value)) => Some(*value),
            Some(SendPropValue::Integer(value)) => Some(*value as f32),
            _ => None,
        }
    }

    fn integer(&self, table: &str, name: &str) -> Option<i64> {
        match self.value(table, name) {
            Some(SendPropValue::Integer(value)) => Some(*value),
            Some(SendPropValue::Float(value)) => Some(*value as i64),
            _ => None,
        }
    }

    fn vector(&self, table: &str, name: &str) -> Option<[f32; 3]> {
        match self.value(table, name) {
            Some(SendPropValue::Vector(value)) => Some([value.x, value.y, value.z]),
            Some(SendPropValue::VectorXY(value)) => Some([value.x, value.y, 0.0]),
            Some(SendPropValue::Array(values)) if values.len() >= 3 => Some([
                value_number(&values[0])?,
                value_number(&values[1])?,
                value_number(&values[2])?,
            ]),
            _ => None,
        }
    }
}

fn value_number(value: &SendPropValue) -> Option<f32> {
    match value {
        SendPropValue::Float(value) => Some(*value),
        SendPropValue::Integer(value) => Some(*value as f32),
        _ => None,
    }
}

#[derive(Clone)]
struct PendingEvent {
    entity_index: u32,
    entity_serial: u32,
    event: i32,
    data: i32,
    fire_delay: f32,
}

pub struct AnimationExportStats {
    pub logical_frames: u64,
    pub player_samples: u64,
    pub animation_events: u64,
    pub interval_per_tick: f32,
}

pub struct AnimationExporter {
    readable: BufWriter<File>,
    class_names: HashMap<ClassId, String>,
    entities: BTreeMap<u32, ResolvedEntity>,
    last_positions: HashMap<(u32, u32), (f32, [f32; 3])>,
    server_tick: ServerTick,
    interval_per_tick: f32,
    frame_index: u64,
    player_samples: u64,
    animation_events: u64,
}

impl AnimationExporter {
    pub fn new(output_dir: &Path) -> std::io::Result<Self> {
        Ok(Self {
            readable: BufWriter::new(File::create(output_dir.join("animation_inputs.ndjson"))?),
            class_names: HashMap::new(),
            entities: BTreeMap::new(),
            last_positions: HashMap::new(),
            server_tick: ServerTick::default(),
            interval_per_tick: 1.0 / 66.0,
            frame_index: 0,
            player_samples: 0,
            animation_events: 0,
        })
    }

    pub fn observe(
        &mut self,
        packet: &Packet<'_>,
        parser_state: &ParserState,
    ) -> std::io::Result<()> {
        if let Packet::DataTables(data) = packet {
            for class in &data.server_classes {
                self.class_names
                    .insert(class.id, class.name.as_str().to_owned());
            }
            return Ok(());
        }

        let message_packet = match packet {
            Packet::Message(packet) | Packet::Signon(packet) => packet,
            _ => return Ok(()),
        };

        let mut events = Vec::new();
        let mut has_timeline_data = false;

        for message in &message_packet.messages {
            match message {
                Message::NetTick(message) => {
                    self.server_tick = message.tick;
                    has_timeline_data = true;
                }
                Message::ServerInfo(message) => {
                    if message.interval_per_tick.is_finite() && message.interval_per_tick > 0.0 {
                        self.interval_per_tick = message.interval_per_tick;
                    }
                }
                Message::PacketEntities(message) => {
                    self.apply_entities(&message.entities, &message.removed_entities, parser_state);
                    has_timeline_data = true;
                }
                Message::TempEntities(message) => {
                    for event in &message.events {
                        if self.class_name(event.class_id) != "CTEPlayerAnimEvent" {
                            continue;
                        }
                        let handle = prop_integer(&event.props, "DT_TEPlayerAnimEvent", "m_hPlayer")
                            .unwrap_or(-1) as u64;
                        if handle == u64::MAX {
                            continue;
                        }
                        events.push(PendingEvent {
                            entity_index: (handle & ENTITY_INDEX_MASK) as u32,
                            entity_serial: (handle >> 11) as u32,
                            event: prop_integer(&event.props, "DT_TEPlayerAnimEvent", "m_iEvent")
                                .unwrap_or(0) as i32,
                            data: prop_integer(&event.props, "DT_TEPlayerAnimEvent", "m_nData")
                                .unwrap_or(0) as i32,
                            fire_delay: event.fire_delay,
                        });
                    }
                    has_timeline_data = true;
                }
                _ => {}
            }
        }

        if has_timeline_data && !self.entities.is_empty() {
            self.write_frame(message_packet.tick, &events)?;
        }
        Ok(())
    }

    fn class_name(&self, id: ClassId) -> &str {
        self.class_names.get(&id).map(String::as_str).unwrap_or("")
    }

    fn apply_entities(
        &mut self,
        updates: &[PacketEntity],
        removed_entities: &[tf_demo_parser::demo::message::EntityId],
        parser_state: &ParserState,
    ) {
        for update in updates {
            let index = u32::from(update.entity_index);
            match update.update_type {
                UpdateType::Enter => {
                    let class_name = self.class_name(update.server_class).to_owned();
                    let mut entity = ResolvedEntity {
                        class_id: update.server_class,
                        class_name,
                        serial: update.serial_number,
                        in_pvs: update.in_pvs,
                        props: HashMap::new(),
                    };
                    entity.apply(update.props(parser_state));
                    self.entities.insert(index, entity);
                }
                UpdateType::Delta => {
                    if let Some(entity) = self.entities.get_mut(&index) {
                        entity.in_pvs = update.in_pvs || entity.in_pvs;
                        entity.apply(update.props.iter().cloned());
                    }
                }
                UpdateType::Leave | UpdateType::Delete => {
                    self.entities.remove(&index);
                }
            }
        }

        for id in removed_entities {
            self.entities.remove(&u32::from(*id));
        }
    }

    fn write_frame(&mut self, demo_tick: DemoTick, events: &[PendingEvent]) -> std::io::Result<()> {
        let demo_tick_u32 = u32::from(demo_tick);
        let server_tick_u32 = u32::from(self.server_tick);
        let time = server_tick_u32 as f32 * self.interval_per_tick;

        for event in events {
            serde_json::to_writer(
                &mut self.readable,
                &json!({
                    "type": "animation_event",
                    "frame": self.frame_index,
                    "demo_tick": demo_tick_u32,
                    "server_tick": server_tick_u32,
                    "time": time,
                    "entity_index": event.entity_index,
                    "serial": event.entity_serial,
                    "event": event.event,
                    "data": event.data,
                    "fire_delay": event.fire_delay
                }),
            )
            .map_err(std::io::Error::other)?;
            self.readable.write_all(b"\n")?;
            self.animation_events += 1;
        }

        for (&entity_index, entity) in &self.entities {
            if entity.class_name != "CTFPlayer" {
                continue;
            }
            let Some(origin) = player_origin(entity) else {
                continue;
            };
            let velocity = match self.last_positions.get(&(entity_index, entity.serial)) {
                Some((old_time, old_origin)) if time > *old_time && time - *old_time < 1.0 => {
                    let dt = time - *old_time;
                    [
                        (origin[0] - old_origin[0]) / dt,
                        (origin[1] - old_origin[1]) / dt,
                        (origin[2] - old_origin[2]) / dt,
                    ]
                }
                _ => [0.0; 3],
            };
            self.last_positions
                .insert((entity_index, entity.serial), (time, origin));

            let class_index = entity
                .integer("DT_TFPlayerClassShared", "m_iClass")
                .unwrap_or(0) as i32;
            if !(1..=9).contains(&class_index) {
                continue;
            }
            let team = entity.integer("DT_BaseEntity", "m_iTeamNum").unwrap_or(0) as i32;
            let health = entity.integer("DT_BasePlayer", "m_iHealth").unwrap_or(1) as i32;
            let life_state = entity.integer("DT_BasePlayer", "m_lifeState").unwrap_or(0) as i32;
            let alive = life_state == 0 && health > 0;
            let flags = entity.integer("DT_BasePlayer", "m_fFlags").unwrap_or(1) as i32;
            let water = entity
                .integer("DT_TFPlayer", "m_nWaterLevel")
                .or_else(|| entity.integer("DT_LocalPlayerExclusive", "m_nWaterLevel"))
                .unwrap_or(0) as i32;
            let pitch = player_number(entity, "m_angEyeAngles[0]").unwrap_or(0.0);
            let yaw = player_number(entity, "m_angEyeAngles[1]").unwrap_or(0.0);
            let cycle = entity
                .number("DT_ServerAnimationData", "m_flCycle")
                .or_else(|| entity.number("DT_TFPlayer", "m_flCycle"))
                .unwrap_or(0.0);
            let playback_rate = entity
                .number("DT_BaseAnimating", "m_flPlaybackRate")
                .unwrap_or(1.0);
            let source_sequence = entity
                .integer("DT_BaseAnimating", "m_nSequence")
                .unwrap_or(0) as i32;
            let conditions = [
                entity
                    .integer("DT_TFPlayerShared", "m_nPlayerCond")
                    .unwrap_or(0) as u32,
                entity
                    .integer("DT_TFPlayerShared", "m_nPlayerCondEx")
                    .unwrap_or(0) as u32,
                entity
                    .integer("DT_TFPlayerShared", "m_nPlayerCondEx2")
                    .unwrap_or(0) as u32,
                entity
                    .integer("DT_TFPlayerShared", "m_nPlayerCondEx3")
                    .unwrap_or(0) as u32,
                entity
                    .integer("DT_TFPlayerShared", "m_nPlayerCondEx4")
                    .unwrap_or(0) as u32,
            ];

            let active_handle = entity
                .integer("DT_BaseCombatCharacter", "m_hActiveWeapon")
                .unwrap_or(-1) as u64;
            let weapon_entity = (active_handle & ENTITY_INDEX_MASK) as u32;
            let weapon_serial = (active_handle >> 11) as u32;
            let weapon = self.entities.get(&weapon_entity).filter(|weapon| {
                active_handle != u64::MAX && (weapon.serial == weapon_serial || weapon_serial == 0)
            });
            let weapon_class = weapon
                .map(|weapon| weapon.class_name.as_str())
                .unwrap_or("");
            let weapon_role = animation_role(weapon_class, class_index);
            let model = class_model(class_index);

            serde_json::to_writer(
                &mut self.readable,
                &json!({
                    "type": "player_animation_input",
                    "frame": self.frame_index,
                    "demo_tick": demo_tick_u32,
                    "server_tick": server_tick_u32,
                    "time": time,
                    "entity_index": entity_index,
                    "serial": entity.serial,
                    "in_pvs": entity.in_pvs,
                    "class": class_index,
                    "team": team,
                    "alive": alive,
                    "health": health,
                    "flags": flags,
                    "water_level": water,
                    "origin": origin,
                    "velocity": velocity,
                    "eye_angles": [pitch, yaw, 0.0],
                    "server_animation": {
                        "cycle": cycle,
                        "playback_rate": playback_rate,
                        "source_sequence": source_sequence
                    },
                    "conditions": conditions,
                    "active_weapon": {
                        "entity_index": weapon_entity,
                        "serial": weapon_serial,
                        "server_class": weapon_class,
                        "animation_role": weapon_role
                    },
                    "model": model,
                    "source_server_class_id": u16::from(entity.class_id)
                }),
            )
            .map_err(std::io::Error::other)?;
            self.readable.write_all(b"\n")?;
            self.player_samples += 1;
        }

        self.frame_index += 1;
        Ok(())
    }

    pub fn finish(mut self) -> std::io::Result<AnimationExportStats> {
        self.readable.flush()?;
        Ok(AnimationExportStats {
            logical_frames: self.frame_index,
            player_samples: self.player_samples,
            animation_events: self.animation_events,
            interval_per_tick: self.interval_per_tick,
        })
    }
}

fn prop_integer(props: &[SendProp], table: &str, name: &str) -> Option<i64> {
    let identifier = SendPropIdentifier::new(table, name);
    props.iter().find_map(|prop| {
        if prop.identifier != identifier {
            return None;
        }
        match &prop.value {
            SendPropValue::Integer(value) => Some(*value),
            SendPropValue::Float(value) => Some(*value as i64),
            _ => None,
        }
    })
}

fn player_number(entity: &ResolvedEntity, name: &str) -> Option<f32> {
    entity
        .number("DT_TFNonLocalPlayerExclusive", name)
        .or_else(|| entity.number("DT_TFLocalPlayerExclusive", name))
        .or_else(|| entity.number("DT_TFPlayer", name))
}

fn player_origin(entity: &ResolvedEntity) -> Option<[f32; 3]> {
    let tables = [
        "DT_TFNonLocalPlayerExclusive",
        "DT_TFLocalPlayerExclusive",
        "DT_TFPlayer",
        "DT_BaseEntity",
    ];
    for table in tables {
        if let Some(mut origin) = entity.vector(table, "m_vecOrigin") {
            if let Some(z) = entity.number(table, "m_vecOrigin[2]") {
                origin[2] = z;
            }
            return Some(origin);
        }
    }
    None
}

fn sanitize_field(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '\t' | '\r' | '\n' => '_',
            other => other,
        })
        .collect()
}

fn class_model(class_index: i32) -> &'static str {
    match class_index {
        1 => "models/player/scout.mdl",
        2 => "models/player/sniper.mdl",
        3 => "models/player/soldier.mdl",
        4 => "models/player/demo.mdl",
        5 => "models/player/medic.mdl",
        6 => "models/player/heavy.mdl",
        7 => "models/player/pyro.mdl",
        8 => "models/player/spy.mdl",
        9 => "models/player/engineer.mdl",
        _ => "models/player/scout.mdl",
    }
}

fn animation_role(server_class: &str, player_class: i32) -> i32 {
    let name = server_class.to_ascii_lowercase();
    if name.contains("knife")
        || name.contains("wrench")
        || name.contains("bat")
        || name.contains("fists")
        || name.contains("bonesaw")
        || name.contains("bottle")
        || name.contains("shovel")
        || name.contains("fireaxe")
        || name.contains("sword")
        || name.contains("club")
        || name.contains("breakable_sign")
    {
        return 2; // TF_WPN_TYPE_MELEE
    }
    if name.contains("pda") || name.contains("invis") {
        return 5; // TF_WPN_TYPE_PDA
    }
    if name.contains("builder") {
        return 4; // TF_WPN_TYPE_BUILDING
    }
    if name.contains("shotgun")
        || name.contains("pistol")
        || name.contains("smg")
        || name.contains("medigun")
        || name.contains("pipebomblauncher")
        || name.contains("flaregun")
        || name.contains("lunchbox")
        || name.contains("jar")
        || name.contains("buff_item")
        || name.contains("laser_pointer")
        || name.contains("cleaver")
        || (player_class == 8 && name.contains("revolver"))
    {
        return 1; // TF_WPN_TYPE_SECONDARY
    }
    0 // TF_WPN_TYPE_PRIMARY
}
