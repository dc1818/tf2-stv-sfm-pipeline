use super::gamevent::{
    EventValue, GameEventDefinition, GameEventEntry, GameEventValue, RawGameEvent,
};
use crate::demo::data::MaybeUtf8String;
use crate::demo::Stream;
use crate::{ParseError, Result};
use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};
use std::mem::size_of;
fn read_value<'a, T: EventValue + BitRead<'a, LittleEndian> + Default>(
    stream: &mut Stream<'a>,
    entry: Option<&GameEventEntry>,
    name: &'static str,
) -> Result<T> {
    let entry = match entry {
        Some(entry) => entry,
        None => {
            return Ok(T::default());
        }
    };
    if T::value_type() != entry.kind {
        return Err(ParseError::InvalidGameEvent {
            expected_type: T::value_type(),
            name,
            found_type: entry.kind,
        });
    }
    Ok(T::read(stream)?)
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerSpawnEvent {
    pub hostname: MaybeUtf8String,
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
    pub game: MaybeUtf8String,
    pub map_name: MaybeUtf8String,
    pub max_players: u32,
    pub os: MaybeUtf8String,
    pub dedicated: bool,
    pub password: bool,
}
impl ServerSpawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerSpawnEvent {
            hostname: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(7992289610851289516u64),
                "hostname",
            )?,
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(1673076945917317811u64),
                "address",
            )?,
            ip: read_value::<u32>(stream, definition.get_entry(628043273916406972u64), "ip")?,
            port: read_value::<u16>(
                stream,
                definition.get_entry(10100688915994460070u64),
                "port",
            )?,
            game: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(10005491431272162599u64),
                "game",
            )?,
            map_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(18037678950216614794u64),
                "map_name",
            )?,
            max_players: read_value::<u32>(
                stream,
                definition.get_entry(6820574247554962453u64),
                "max_players",
            )?,
            os: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(626093839799967319u64),
                "os",
            )?,
            dedicated: read_value::<bool>(
                stream,
                definition.get_entry(17181338330120084322u64),
                "dedicated",
            )?,
            password: read_value::<bool>(
                stream,
                definition.get_entry(5411718394350379800u64),
                "password",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            7992289610851289516u64 => Ok(self.hostname.clone().into()),
            1673076945917317811u64 => Ok(self.address.clone().into()),
            628043273916406972u64 => Ok(self.ip.clone().into()),
            10100688915994460070u64 => Ok(self.port.clone().into()),
            10005491431272162599u64 => Ok(self.game.clone().into()),
            18037678950216614794u64 => Ok(self.map_name.clone().into()),
            6820574247554962453u64 => Ok(self.max_players.clone().into()),
            626093839799967319u64 => Ok(self.os.clone().into()),
            17181338330120084322u64 => Ok(self.dedicated.clone().into()),
            5411718394350379800u64 => Ok(self.password.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerSpawn",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerChangeLevelFailedEvent {
    pub level_name: MaybeUtf8String,
}
impl ServerChangeLevelFailedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerChangeLevelFailedEvent {
            level_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(8103714013497669086u64),
                "level_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            8103714013497669086u64 => Ok(self.level_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerChangeLevelFailed",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerShutdownEvent {
    pub reason: MaybeUtf8String,
}
impl ServerShutdownEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerShutdownEvent {
            reason: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(7343356632300987961u64),
                "reason",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            7343356632300987961u64 => Ok(self.reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerShutdown",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerCvarEvent {
    pub cvar_name: MaybeUtf8String,
    pub cvar_value: MaybeUtf8String,
}
impl ServerCvarEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerCvarEvent {
            cvar_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(8822721269188576188u64),
                "cvar_name",
            )?,
            cvar_value: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(9254320334284503348u64),
                "cvar_value",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            8822721269188576188u64 => Ok(self.cvar_name.clone().into()),
            9254320334284503348u64 => Ok(self.cvar_value.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerCvar",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerMessageEvent {
    pub text: MaybeUtf8String,
}
impl ServerMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerMessageEvent {
            text: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(18015793717152399486u64),
                "text",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18015793717152399486u64 => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerMessage",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerAddBanEvent {
    pub name: MaybeUtf8String,
    pub user_id: u16,
    pub network_id: MaybeUtf8String,
    pub ip: MaybeUtf8String,
    pub duration: MaybeUtf8String,
    pub by: MaybeUtf8String,
    pub kicked: bool,
}
impl ServerAddBanEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerAddBanEvent {
            name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14176396743819860870u64),
                "name",
            )?,
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(2293149186490744864u64),
                "network_id",
            )?,
            ip: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(628043273916406972u64),
                "ip",
            )?,
            duration: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(10012068961515151501u64),
                "duration",
            )?,
            by: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(623268094916032724u64),
                "by",
            )?,
            kicked: read_value::<bool>(
                stream,
                definition.get_entry(11906843383782741950u64),
                "kicked",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            14176396743819860870u64 => Ok(self.name.clone().into()),
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            2293149186490744864u64 => Ok(self.network_id.clone().into()),
            628043273916406972u64 => Ok(self.ip.clone().into()),
            10012068961515151501u64 => Ok(self.duration.clone().into()),
            623268094916032724u64 => Ok(self.by.clone().into()),
            11906843383782741950u64 => Ok(self.kicked.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerAddBan",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerRemoveBanEvent {
    pub network_id: MaybeUtf8String,
    pub ip: MaybeUtf8String,
    pub by: MaybeUtf8String,
}
impl ServerRemoveBanEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerRemoveBanEvent {
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(2293149186490744864u64),
                "network_id",
            )?,
            ip: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(628043273916406972u64),
                "ip",
            )?,
            by: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(623268094916032724u64),
                "by",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            2293149186490744864u64 => Ok(self.network_id.clone().into()),
            628043273916406972u64 => Ok(self.ip.clone().into()),
            623268094916032724u64 => Ok(self.by.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerRemoveBan",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerConnectEvent {
    pub name: MaybeUtf8String,
    pub index: u8,
    pub user_id: u16,
    pub network_id: MaybeUtf8String,
    pub address: MaybeUtf8String,
    pub bot: u16,
}
impl PlayerConnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerConnectEvent {
            name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14176396743819860870u64),
                "name",
            )?,
            index: read_value::<u8>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(2293149186490744864u64),
                "network_id",
            )?,
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(1673076945917317811u64),
                "address",
            )?,
            bot: read_value::<u16>(stream, definition.get_entry(21728656485903294u64), "bot")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            14176396743819860870u64 => Ok(self.name.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            2293149186490744864u64 => Ok(self.network_id.clone().into()),
            1673076945917317811u64 => Ok(self.address.clone().into()),
            21728656485903294u64 => Ok(self.bot.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerConnect",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerConnectClientEvent {
    pub name: MaybeUtf8String,
    pub index: u8,
    pub user_id: u16,
    pub network_id: MaybeUtf8String,
    pub bot: u16,
}
impl PlayerConnectClientEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerConnectClientEvent {
            name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14176396743819860870u64),
                "name",
            )?,
            index: read_value::<u8>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(2293149186490744864u64),
                "network_id",
            )?,
            bot: read_value::<u16>(stream, definition.get_entry(21728656485903294u64), "bot")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            14176396743819860870u64 => Ok(self.name.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            2293149186490744864u64 => Ok(self.network_id.clone().into()),
            21728656485903294u64 => Ok(self.bot.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerConnectClient",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerInfoEvent {
    pub name: MaybeUtf8String,
    pub index: u8,
    pub user_id: u16,
    pub network_id: MaybeUtf8String,
    pub bot: bool,
}
impl PlayerInfoEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerInfoEvent {
            name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14176396743819860870u64),
                "name",
            )?,
            index: read_value::<u8>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(2293149186490744864u64),
                "network_id",
            )?,
            bot: read_value::<bool>(stream, definition.get_entry(21728656485903294u64), "bot")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            14176396743819860870u64 => Ok(self.name.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            2293149186490744864u64 => Ok(self.network_id.clone().into()),
            21728656485903294u64 => Ok(self.bot.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerInfo",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDisconnectEvent {
    pub user_id: u16,
    pub reason: MaybeUtf8String,
    pub name: MaybeUtf8String,
    pub network_id: MaybeUtf8String,
    pub bot: u16,
}
impl PlayerDisconnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDisconnectEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            reason: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(7343356632300987961u64),
                "reason",
            )?,
            name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14176396743819860870u64),
                "name",
            )?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(2293149186490744864u64),
                "network_id",
            )?,
            bot: read_value::<u16>(stream, definition.get_entry(21728656485903294u64), "bot")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            7343356632300987961u64 => Ok(self.reason.clone().into()),
            14176396743819860870u64 => Ok(self.name.clone().into()),
            2293149186490744864u64 => Ok(self.network_id.clone().into()),
            21728656485903294u64 => Ok(self.bot.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDisconnect",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerActivateEvent {
    pub user_id: u16,
}
impl PlayerActivateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerActivateEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerActivate",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerSayEvent {
    pub user_id: u16,
    pub text: MaybeUtf8String,
}
impl PlayerSayEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerSayEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            text: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(18015793717152399486u64),
                "text",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            18015793717152399486u64 => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerSay",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientDisconnectEvent {
    pub message: MaybeUtf8String,
}
impl ClientDisconnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ClientDisconnectEvent {
            message: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(6080987277291999908u64),
                "message",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            6080987277291999908u64 => Ok(self.message.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ClientDisconnect",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientBeginConnectEvent {
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
    pub source: MaybeUtf8String,
}
impl ClientBeginConnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ClientBeginConnectEvent {
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(1673076945917317811u64),
                "address",
            )?,
            ip: read_value::<u32>(stream, definition.get_entry(628043273916406972u64), "ip")?,
            port: read_value::<u16>(
                stream,
                definition.get_entry(10100688915994460070u64),
                "port",
            )?,
            source: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(8564681157369146808u64),
                "source",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            1673076945917317811u64 => Ok(self.address.clone().into()),
            628043273916406972u64 => Ok(self.ip.clone().into()),
            10100688915994460070u64 => Ok(self.port.clone().into()),
            8564681157369146808u64 => Ok(self.source.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ClientBeginConnect",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientConnectedEvent {
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
}
impl ClientConnectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ClientConnectedEvent {
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(1673076945917317811u64),
                "address",
            )?,
            ip: read_value::<u32>(stream, definition.get_entry(628043273916406972u64), "ip")?,
            port: read_value::<u16>(
                stream,
                definition.get_entry(10100688915994460070u64),
                "port",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            1673076945917317811u64 => Ok(self.address.clone().into()),
            628043273916406972u64 => Ok(self.ip.clone().into()),
            10100688915994460070u64 => Ok(self.port.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ClientConnected",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientFullConnectEvent {
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
}
impl ClientFullConnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ClientFullConnectEvent {
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(1673076945917317811u64),
                "address",
            )?,
            ip: read_value::<u32>(stream, definition.get_entry(628043273916406972u64), "ip")?,
            port: read_value::<u16>(
                stream,
                definition.get_entry(10100688915994460070u64),
                "port",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            1673076945917317811u64 => Ok(self.address.clone().into()),
            628043273916406972u64 => Ok(self.ip.clone().into()),
            10100688915994460070u64 => Ok(self.port.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ClientFullConnect",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HostQuitEvent {}
impl HostQuitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HostQuitEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HostQuit",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamInfoEvent {
    pub team_id: u8,
    pub team_name: MaybeUtf8String,
}
impl TeamInfoEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamInfoEvent {
            team_id: read_value::<u8>(
                stream,
                definition.get_entry(16102541790268531873u64),
                "team_id",
            )?,
            team_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(2516817673391228199u64),
                "team_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            16102541790268531873u64 => Ok(self.team_id.clone().into()),
            2516817673391228199u64 => Ok(self.team_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamInfo",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamScoreEvent {
    pub team_id: u8,
    pub score: u16,
}
impl TeamScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamScoreEvent {
            team_id: read_value::<u8>(
                stream,
                definition.get_entry(16102541790268531873u64),
                "team_id",
            )?,
            score: read_value::<u16>(
                stream,
                definition.get_entry(13911166232573650165u64),
                "score",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            16102541790268531873u64 => Ok(self.team_id.clone().into()),
            13911166232573650165u64 => Ok(self.score.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamScore",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayBroadcastAudioEvent {
    pub team: u8,
    pub sound: MaybeUtf8String,
    pub additional_flags: u16,
    pub player: u16,
}
impl TeamPlayBroadcastAudioEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayBroadcastAudioEvent {
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            sound: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(7337464818993397268u64),
                "sound",
            )?,
            additional_flags: read_value::<u16>(
                stream,
                definition.get_entry(10653216584182196624u64),
                "additional_flags",
            )?,
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18024489754618217260u64 => Ok(self.team.clone().into()),
            7337464818993397268u64 => Ok(self.sound.clone().into()),
            10653216584182196624u64 => Ok(self.additional_flags.clone().into()),
            5008278420455340480u64 => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayBroadcastAudio",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerTeamEvent {
    pub user_id: u16,
    pub team: u8,
    pub old_team: u8,
    pub disconnect: bool,
    pub auto_team: bool,
    pub silent: bool,
    pub name: MaybeUtf8String,
}
impl PlayerTeamEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerTeamEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            old_team: read_value::<u8>(
                stream,
                definition.get_entry(11079076405359550719u64),
                "old_team",
            )?,
            disconnect: read_value::<bool>(
                stream,
                definition.get_entry(6424045679635350635u64),
                "disconnect",
            )?,
            auto_team: read_value::<bool>(
                stream,
                definition.get_entry(9165371035831628223u64),
                "auto_team",
            )?,
            silent: read_value::<bool>(
                stream,
                definition.get_entry(6452236368899434340u64),
                "silent",
            )?,
            name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14176396743819860870u64),
                "name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            11079076405359550719u64 => Ok(self.old_team.clone().into()),
            6424045679635350635u64 => Ok(self.disconnect.clone().into()),
            9165371035831628223u64 => Ok(self.auto_team.clone().into()),
            6452236368899434340u64 => Ok(self.silent.clone().into()),
            14176396743819860870u64 => Ok(self.name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerTeam",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerClassEvent {
    pub user_id: u16,
    pub class: MaybeUtf8String,
}
impl PlayerClassEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerClassEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            class: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(15066323702654938015u64),
                "class",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            15066323702654938015u64 => Ok(self.class.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerClass",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDeathEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: MaybeUtf8String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: MaybeUtf8String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub player_penetrate_count: u16,
    pub assister_fallback: MaybeUtf8String,
    pub kill_streak_total: u16,
    pub kill_streak_wep: u16,
    pub kill_streak_assist: u16,
    pub kill_streak_victim: u16,
    pub ducks_streaked: u16,
    pub duck_streak_total: u16,
    pub duck_streak_assist: u16,
    pub duck_streak_victim: u16,
    pub rocket_jump: bool,
    pub weapon_def_index: u32,
    pub crit_type: u16,
}
impl PlayerDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDeathEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            victim_ent_index: read_value::<u32>(
                stream,
                definition.get_entry(7608903543976233025u64),
                "victim_ent_index",
            )?,
            inflictor_ent_index: read_value::<u32>(
                stream,
                definition.get_entry(7862267791693534473u64),
                "inflictor_ent_index",
            )?,
            attacker: read_value::<u16>(
                stream,
                definition.get_entry(7198542740550218478u64),
                "attacker",
            )?,
            weapon: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(11580461223051554305u64),
                "weapon",
            )?,
            weapon_id: read_value::<u16>(
                stream,
                definition.get_entry(5542695206485500884u64),
                "weapon_id",
            )?,
            damage_bits: read_value::<u32>(
                stream,
                definition.get_entry(2104626753992558984u64),
                "damage_bits",
            )?,
            custom_kill: read_value::<u16>(
                stream,
                definition.get_entry(9002408094759571186u64),
                "custom_kill",
            )?,
            assister: read_value::<u16>(
                stream,
                definition.get_entry(17978308754419261977u64),
                "assister",
            )?,
            weapon_log_class_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(8214628514117900939u64),
                "weapon_log_class_name",
            )?,
            stun_flags: read_value::<u16>(
                stream,
                definition.get_entry(16746745151415897845u64),
                "stun_flags",
            )?,
            death_flags: read_value::<u16>(
                stream,
                definition.get_entry(210841622282264177u64),
                "death_flags",
            )?,
            silent_kill: read_value::<bool>(
                stream,
                definition.get_entry(5449831253309542421u64),
                "silent_kill",
            )?,
            player_penetrate_count: read_value::<u16>(
                stream,
                definition.get_entry(6165847213797285919u64),
                "player_penetrate_count",
            )?,
            assister_fallback: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(2624120833319605424u64),
                "assister_fallback",
            )?,
            kill_streak_total: read_value::<u16>(
                stream,
                definition.get_entry(10219443329572148957u64),
                "kill_streak_total",
            )?,
            kill_streak_wep: read_value::<u16>(
                stream,
                definition.get_entry(14151704064294986651u64),
                "kill_streak_wep",
            )?,
            kill_streak_assist: read_value::<u16>(
                stream,
                definition.get_entry(3408761288007698574u64),
                "kill_streak_assist",
            )?,
            kill_streak_victim: read_value::<u16>(
                stream,
                definition.get_entry(14613767699666342005u64),
                "kill_streak_victim",
            )?,
            ducks_streaked: read_value::<u16>(
                stream,
                definition.get_entry(8814124002674372577u64),
                "ducks_streaked",
            )?,
            duck_streak_total: read_value::<u16>(
                stream,
                definition.get_entry(2758270581670703974u64),
                "duck_streak_total",
            )?,
            duck_streak_assist: read_value::<u16>(
                stream,
                definition.get_entry(10967369523768500963u64),
                "duck_streak_assist",
            )?,
            duck_streak_victim: read_value::<u16>(
                stream,
                definition.get_entry(620103205137188524u64),
                "duck_streak_victim",
            )?,
            rocket_jump: read_value::<bool>(
                stream,
                definition.get_entry(16207427969859362406u64),
                "rocket_jump",
            )?,
            weapon_def_index: read_value::<u32>(
                stream,
                definition.get_entry(4132306594868589054u64),
                "weapon_def_index",
            )?,
            crit_type: read_value::<u16>(
                stream,
                definition.get_entry(7263029362109349446u64),
                "crit_type",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            7608903543976233025u64 => Ok(self.victim_ent_index.clone().into()),
            7862267791693534473u64 => Ok(self.inflictor_ent_index.clone().into()),
            7198542740550218478u64 => Ok(self.attacker.clone().into()),
            11580461223051554305u64 => Ok(self.weapon.clone().into()),
            5542695206485500884u64 => Ok(self.weapon_id.clone().into()),
            2104626753992558984u64 => Ok(self.damage_bits.clone().into()),
            9002408094759571186u64 => Ok(self.custom_kill.clone().into()),
            17978308754419261977u64 => Ok(self.assister.clone().into()),
            8214628514117900939u64 => Ok(self.weapon_log_class_name.clone().into()),
            16746745151415897845u64 => Ok(self.stun_flags.clone().into()),
            210841622282264177u64 => Ok(self.death_flags.clone().into()),
            5449831253309542421u64 => Ok(self.silent_kill.clone().into()),
            6165847213797285919u64 => Ok(self.player_penetrate_count.clone().into()),
            2624120833319605424u64 => Ok(self.assister_fallback.clone().into()),
            10219443329572148957u64 => Ok(self.kill_streak_total.clone().into()),
            14151704064294986651u64 => Ok(self.kill_streak_wep.clone().into()),
            3408761288007698574u64 => Ok(self.kill_streak_assist.clone().into()),
            14613767699666342005u64 => Ok(self.kill_streak_victim.clone().into()),
            8814124002674372577u64 => Ok(self.ducks_streaked.clone().into()),
            2758270581670703974u64 => Ok(self.duck_streak_total.clone().into()),
            10967369523768500963u64 => Ok(self.duck_streak_assist.clone().into()),
            620103205137188524u64 => Ok(self.duck_streak_victim.clone().into()),
            16207427969859362406u64 => Ok(self.rocket_jump.clone().into()),
            4132306594868589054u64 => Ok(self.weapon_def_index.clone().into()),
            7263029362109349446u64 => Ok(self.crit_type.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDeath",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHurtEvent {
    pub user_id: u16,
    pub health: u16,
    pub attacker: u16,
    pub damage_amount: u16,
    pub custom: u16,
    pub show_disguised_crit: bool,
    pub crit: bool,
    pub mini_crit: bool,
    pub all_see_crit: bool,
    pub weapon_id: u16,
    pub bonus_effect: u8,
}
impl PlayerHurtEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHurtEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            health: read_value::<u16>(
                stream,
                definition.get_entry(9181103189905877455u64),
                "health",
            )?,
            attacker: read_value::<u16>(
                stream,
                definition.get_entry(7198542740550218478u64),
                "attacker",
            )?,
            damage_amount: read_value::<u16>(
                stream,
                definition.get_entry(7439038394412279612u64),
                "damage_amount",
            )?,
            custom: read_value::<u16>(
                stream,
                definition.get_entry(604290716149806926u64),
                "custom",
            )?,
            show_disguised_crit: read_value::<bool>(
                stream,
                definition.get_entry(14301803044080296297u64),
                "show_disguised_crit",
            )?,
            crit: read_value::<bool>(stream, definition.get_entry(1324453635955533101u64), "crit")?,
            mini_crit: read_value::<bool>(
                stream,
                definition.get_entry(18286698110279670006u64),
                "mini_crit",
            )?,
            all_see_crit: read_value::<bool>(
                stream,
                definition.get_entry(3290419718563846047u64),
                "all_see_crit",
            )?,
            weapon_id: read_value::<u16>(
                stream,
                definition.get_entry(5542695206485500884u64),
                "weapon_id",
            )?,
            bonus_effect: read_value::<u8>(
                stream,
                definition.get_entry(4613275483771643085u64),
                "bonus_effect",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            9181103189905877455u64 => Ok(self.health.clone().into()),
            7198542740550218478u64 => Ok(self.attacker.clone().into()),
            7439038394412279612u64 => Ok(self.damage_amount.clone().into()),
            604290716149806926u64 => Ok(self.custom.clone().into()),
            14301803044080296297u64 => Ok(self.show_disguised_crit.clone().into()),
            1324453635955533101u64 => Ok(self.crit.clone().into()),
            18286698110279670006u64 => Ok(self.mini_crit.clone().into()),
            3290419718563846047u64 => Ok(self.all_see_crit.clone().into()),
            5542695206485500884u64 => Ok(self.weapon_id.clone().into()),
            4613275483771643085u64 => Ok(self.bonus_effect.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHurt",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChatEvent {
    pub team_only: bool,
    pub user_id: u16,
    pub text: MaybeUtf8String,
}
impl PlayerChatEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerChatEvent {
            team_only: read_value::<bool>(
                stream,
                definition.get_entry(8997360128490965478u64),
                "team_only",
            )?,
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            text: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(18015793717152399486u64),
                "text",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            8997360128490965478u64 => Ok(self.team_only.clone().into()),
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            18015793717152399486u64 => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerChat",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerScoreEvent {
    pub user_id: u16,
    pub kills: u16,
    pub deaths: u16,
    pub score: u16,
}
impl PlayerScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerScoreEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            kills: read_value::<u16>(
                stream,
                definition.get_entry(8934927864608526494u64),
                "kills",
            )?,
            deaths: read_value::<u16>(
                stream,
                definition.get_entry(15487195249286514682u64),
                "deaths",
            )?,
            score: read_value::<u16>(
                stream,
                definition.get_entry(13911166232573650165u64),
                "score",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            8934927864608526494u64 => Ok(self.kills.clone().into()),
            15487195249286514682u64 => Ok(self.deaths.clone().into()),
            13911166232573650165u64 => Ok(self.score.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerScore",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerSpawnEvent {
    pub user_id: u16,
    pub team: u16,
    pub class: u16,
}
impl PlayerSpawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerSpawnEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            team: read_value::<u16>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            class: read_value::<u16>(
                stream,
                definition.get_entry(15066323702654938015u64),
                "class",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            15066323702654938015u64 => Ok(self.class.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerSpawn",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerShootEvent {
    pub user_id: u16,
    pub weapon: u8,
    pub mode: u8,
}
impl PlayerShootEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerShootEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            weapon: read_value::<u8>(
                stream,
                definition.get_entry(11580461223051554305u64),
                "weapon",
            )?,
            mode: read_value::<u8>(stream, definition.get_entry(954177780379921842u64), "mode")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            11580461223051554305u64 => Ok(self.weapon.clone().into()),
            954177780379921842u64 => Ok(self.mode.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerShoot",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUseEvent {
    pub user_id: u16,
    pub entity: u16,
}
impl PlayerUseEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerUseEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            entity: read_value::<u16>(
                stream,
                definition.get_entry(10409922166629367034u64),
                "entity",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            10409922166629367034u64 => Ok(self.entity.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerUse",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChangeNameEvent {
    pub user_id: u16,
    pub old_name: MaybeUtf8String,
    pub new_name: MaybeUtf8String,
}
impl PlayerChangeNameEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerChangeNameEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            old_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(11919108480345551253u64),
                "old_name",
            )?,
            new_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(8904377156710117674u64),
                "new_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            11919108480345551253u64 => Ok(self.old_name.clone().into()),
            8904377156710117674u64 => Ok(self.new_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerChangeName",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHintMessageEvent {
    pub hint_message: MaybeUtf8String,
}
impl PlayerHintMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHintMessageEvent {
            hint_message: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(13996716249204415567u64),
                "hint_message",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            13996716249204415567u64 => Ok(self.hint_message.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHintMessage",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BasePlayerTeleportedEvent {
    pub ent_index: u16,
}
impl BasePlayerTeleportedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BasePlayerTeleportedEvent {
            ent_index: read_value::<u16>(
                stream,
                definition.get_entry(17554918082946193550u64),
                "ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17554918082946193550u64 => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BasePlayerTeleported",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameInitEvent {}
impl GameInitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameInitEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameInit",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameNewMapEvent {
    pub map_name: MaybeUtf8String,
}
impl GameNewMapEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameNewMapEvent {
            map_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(18037678950216614794u64),
                "map_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18037678950216614794u64 => Ok(self.map_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameNewMap",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameStartEvent {
    pub rounds_limit: u32,
    pub time_limit: u32,
    pub frag_limit: u32,
    pub objective: MaybeUtf8String,
}
impl GameStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameStartEvent {
            rounds_limit: read_value::<u32>(
                stream,
                definition.get_entry(6594118856211890507u64),
                "rounds_limit",
            )?,
            time_limit: read_value::<u32>(
                stream,
                definition.get_entry(8925605756456439511u64),
                "time_limit",
            )?,
            frag_limit: read_value::<u32>(
                stream,
                definition.get_entry(9937264313491586980u64),
                "frag_limit",
            )?,
            objective: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(8095747183904291896u64),
                "objective",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            6594118856211890507u64 => Ok(self.rounds_limit.clone().into()),
            8925605756456439511u64 => Ok(self.time_limit.clone().into()),
            9937264313491586980u64 => Ok(self.frag_limit.clone().into()),
            8095747183904291896u64 => Ok(self.objective.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameStart",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameEndEvent {
    pub winner: u8,
}
impl GameEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameEndEvent {
            winner: read_value::<u8>(
                stream,
                definition.get_entry(4337804175666422150u64),
                "winner",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            4337804175666422150u64 => Ok(self.winner.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameEnd",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RoundStartEvent {
    pub time_limit: u32,
    pub frag_limit: u32,
    pub objective: MaybeUtf8String,
}
impl RoundStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RoundStartEvent {
            time_limit: read_value::<u32>(
                stream,
                definition.get_entry(8925605756456439511u64),
                "time_limit",
            )?,
            frag_limit: read_value::<u32>(
                stream,
                definition.get_entry(9937264313491586980u64),
                "frag_limit",
            )?,
            objective: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(8095747183904291896u64),
                "objective",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            8925605756456439511u64 => Ok(self.time_limit.clone().into()),
            9937264313491586980u64 => Ok(self.frag_limit.clone().into()),
            8095747183904291896u64 => Ok(self.objective.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RoundStart",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RoundEndEvent {
    pub winner: u8,
    pub reason: u8,
    pub message: MaybeUtf8String,
}
impl RoundEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RoundEndEvent {
            winner: read_value::<u8>(
                stream,
                definition.get_entry(4337804175666422150u64),
                "winner",
            )?,
            reason: read_value::<u8>(
                stream,
                definition.get_entry(7343356632300987961u64),
                "reason",
            )?,
            message: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(6080987277291999908u64),
                "message",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            4337804175666422150u64 => Ok(self.winner.clone().into()),
            7343356632300987961u64 => Ok(self.reason.clone().into()),
            6080987277291999908u64 => Ok(self.message.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RoundEnd",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameMessageEvent {
    pub target: u8,
    pub text: MaybeUtf8String,
}
impl GameMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameMessageEvent {
            target: read_value::<u8>(
                stream,
                definition.get_entry(1653916590517707752u64),
                "target",
            )?,
            text: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(18015793717152399486u64),
                "text",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            1653916590517707752u64 => Ok(self.target.clone().into()),
            18015793717152399486u64 => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameMessage",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BreakBreakableEvent {
    pub ent_index: u32,
    pub user_id: u16,
    pub material: u8,
}
impl BreakBreakableEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BreakBreakableEvent {
            ent_index: read_value::<u32>(
                stream,
                definition.get_entry(17554918082946193550u64),
                "ent_index",
            )?,
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            material: read_value::<u8>(
                stream,
                definition.get_entry(175488002581160416u64),
                "material",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17554918082946193550u64 => Ok(self.ent_index.clone().into()),
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            175488002581160416u64 => Ok(self.material.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BreakBreakable",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BreakPropEvent {
    pub ent_index: u32,
    pub user_id: u16,
}
impl BreakPropEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BreakPropEvent {
            ent_index: read_value::<u32>(
                stream,
                definition.get_entry(17554918082946193550u64),
                "ent_index",
            )?,
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17554918082946193550u64 => Ok(self.ent_index.clone().into()),
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BreakProp",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EntityKilledEvent {
    pub ent_index_killed: u32,
    pub ent_index_attacker: u32,
    pub ent_index_inflictor: u32,
    pub damage_bits: u32,
}
impl EntityKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EntityKilledEvent {
            ent_index_killed: read_value::<u32>(
                stream,
                definition.get_entry(9772342216534838146u64),
                "ent_index_killed",
            )?,
            ent_index_attacker: read_value::<u32>(
                stream,
                definition.get_entry(15130955426090253880u64),
                "ent_index_attacker",
            )?,
            ent_index_inflictor: read_value::<u32>(
                stream,
                definition.get_entry(12707416538007474931u64),
                "ent_index_inflictor",
            )?,
            damage_bits: read_value::<u32>(
                stream,
                definition.get_entry(2104626753992558984u64),
                "damage_bits",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9772342216534838146u64 => Ok(self.ent_index_killed.clone().into()),
            15130955426090253880u64 => Ok(self.ent_index_attacker.clone().into()),
            12707416538007474931u64 => Ok(self.ent_index_inflictor.clone().into()),
            2104626753992558984u64 => Ok(self.damage_bits.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EntityKilled",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BonusUpdatedEvent {
    pub num_advanced: u16,
    pub num_bronze: u16,
    pub num_silver: u16,
    pub num_gold: u16,
}
impl BonusUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BonusUpdatedEvent {
            num_advanced: read_value::<u16>(
                stream,
                definition.get_entry(18281866615588036317u64),
                "num_advanced",
            )?,
            num_bronze: read_value::<u16>(
                stream,
                definition.get_entry(17784477894966475211u64),
                "num_bronze",
            )?,
            num_silver: read_value::<u16>(
                stream,
                definition.get_entry(654857209225446882u64),
                "num_silver",
            )?,
            num_gold: read_value::<u16>(
                stream,
                definition.get_entry(4800941608394754913u64),
                "num_gold",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18281866615588036317u64 => Ok(self.num_advanced.clone().into()),
            17784477894966475211u64 => Ok(self.num_bronze.clone().into()),
            654857209225446882u64 => Ok(self.num_silver.clone().into()),
            4800941608394754913u64 => Ok(self.num_gold.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BonusUpdated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementEventEvent {
    pub achievement_name: MaybeUtf8String,
    pub cur_val: u16,
    pub max_val: u16,
}
impl AchievementEventEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(AchievementEventEvent {
            achievement_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(15691172238087995014u64),
                "achievement_name",
            )?,
            cur_val: read_value::<u16>(
                stream,
                definition.get_entry(5486189633889604213u64),
                "cur_val",
            )?,
            max_val: read_value::<u16>(
                stream,
                definition.get_entry(15860362688261047681u64),
                "max_val",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            15691172238087995014u64 => Ok(self.achievement_name.clone().into()),
            5486189633889604213u64 => Ok(self.cur_val.clone().into()),
            15860362688261047681u64 => Ok(self.max_val.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "AchievementEvent",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementIncrementEvent {
    pub achievement_id: u32,
    pub cur_val: u16,
    pub max_val: u16,
}
impl AchievementIncrementEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(AchievementIncrementEvent {
            achievement_id: read_value::<u32>(
                stream,
                definition.get_entry(17475110908491474368u64),
                "achievement_id",
            )?,
            cur_val: read_value::<u16>(
                stream,
                definition.get_entry(5486189633889604213u64),
                "cur_val",
            )?,
            max_val: read_value::<u16>(
                stream,
                definition.get_entry(15860362688261047681u64),
                "max_val",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17475110908491474368u64 => Ok(self.achievement_id.clone().into()),
            5486189633889604213u64 => Ok(self.cur_val.clone().into()),
            15860362688261047681u64 => Ok(self.max_val.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "AchievementIncrement",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PhysgunPickupEvent {
    pub ent_index: u32,
}
impl PhysgunPickupEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PhysgunPickupEvent {
            ent_index: read_value::<u32>(
                stream,
                definition.get_entry(17554918082946193550u64),
                "ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17554918082946193550u64 => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PhysgunPickup",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FlareIgniteNpcEvent {
    pub ent_index: u32,
}
impl FlareIgniteNpcEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(FlareIgniteNpcEvent {
            ent_index: read_value::<u32>(
                stream,
                definition.get_entry(17554918082946193550u64),
                "ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17554918082946193550u64 => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "FlareIgniteNpc",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HelicopterGrenadePuntMissEvent {}
impl HelicopterGrenadePuntMissEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HelicopterGrenadePuntMissEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HelicopterGrenadePuntMiss",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UserDataDownloadedEvent {}
impl UserDataDownloadedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(UserDataDownloadedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "UserDataDownloaded",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RagdollDissolvedEvent {
    pub ent_index: u32,
}
impl RagdollDissolvedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RagdollDissolvedEvent {
            ent_index: read_value::<u32>(
                stream,
                definition.get_entry(17554918082946193550u64),
                "ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17554918082946193550u64 => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RagdollDissolved",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVChangedModeEvent {
    pub old_mode: u16,
    pub new_mode: u16,
    pub obs_target: u16,
}
impl HLTVChangedModeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVChangedModeEvent {
            old_mode: read_value::<u16>(
                stream,
                definition.get_entry(13993189934714533949u64),
                "old_mode",
            )?,
            new_mode: read_value::<u16>(
                stream,
                definition.get_entry(874641438558876942u64),
                "new_mode",
            )?,
            obs_target: read_value::<u16>(
                stream,
                definition.get_entry(14360750886734159999u64),
                "obs_target",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            13993189934714533949u64 => Ok(self.old_mode.clone().into()),
            874641438558876942u64 => Ok(self.new_mode.clone().into()),
            14360750886734159999u64 => Ok(self.obs_target.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVChangedMode",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVChangedTargetEvent {
    pub mode: u16,
    pub old_target: u16,
    pub obs_target: u16,
}
impl HLTVChangedTargetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVChangedTargetEvent {
            mode: read_value::<u16>(stream, definition.get_entry(954177780379921842u64), "mode")?,
            old_target: read_value::<u16>(
                stream,
                definition.get_entry(16423341895021030510u64),
                "old_target",
            )?,
            obs_target: read_value::<u16>(
                stream,
                definition.get_entry(14360750886734159999u64),
                "obs_target",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            954177780379921842u64 => Ok(self.mode.clone().into()),
            16423341895021030510u64 => Ok(self.old_target.clone().into()),
            14360750886734159999u64 => Ok(self.obs_target.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVChangedTarget",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteEndedEvent {}
impl VoteEndedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteEndedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteEnded",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteStartedEvent {
    pub issue: MaybeUtf8String,
    pub param_1: MaybeUtf8String,
    pub team: u8,
    pub initiator: u32,
    pub voteidx: u32,
}
impl VoteStartedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteStartedEvent {
            issue: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(2041490703516169504u64),
                "issue",
            )?,
            param_1: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(4990490691588242105u64),
                "param_1",
            )?,
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            initiator: read_value::<u32>(
                stream,
                definition.get_entry(7196121162372295066u64),
                "initiator",
            )?,
            voteidx: read_value::<u32>(
                stream,
                definition.get_entry(5777630607239142584u64),
                "voteidx",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            2041490703516169504u64 => Ok(self.issue.clone().into()),
            4990490691588242105u64 => Ok(self.param_1.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            7196121162372295066u64 => Ok(self.initiator.clone().into()),
            5777630607239142584u64 => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteStarted",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteChangedEvent {
    pub vote_option_1: u8,
    pub vote_option_2: u8,
    pub vote_option_3: u8,
    pub vote_option_4: u8,
    pub vote_option_5: u8,
    pub potential_votes: u8,
    pub voteidx: u32,
}
impl VoteChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteChangedEvent {
            vote_option_1: read_value::<u8>(
                stream,
                definition.get_entry(8259566198638570134u64),
                "vote_option_1",
            )?,
            vote_option_2: read_value::<u8>(
                stream,
                definition.get_entry(8259565099126941923u64),
                "vote_option_2",
            )?,
            vote_option_3: read_value::<u8>(
                stream,
                definition.get_entry(8259563999615313712u64),
                "vote_option_3",
            )?,
            vote_option_4: read_value::<u8>(
                stream,
                definition.get_entry(8259571696196711189u64),
                "vote_option_4",
            )?,
            vote_option_5: read_value::<u8>(
                stream,
                definition.get_entry(8259570596685082978u64),
                "vote_option_5",
            )?,
            potential_votes: read_value::<u8>(
                stream,
                definition.get_entry(18034020270891649474u64),
                "potential_votes",
            )?,
            voteidx: read_value::<u32>(
                stream,
                definition.get_entry(5777630607239142584u64),
                "voteidx",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            8259566198638570134u64 => Ok(self.vote_option_1.clone().into()),
            8259565099126941923u64 => Ok(self.vote_option_2.clone().into()),
            8259563999615313712u64 => Ok(self.vote_option_3.clone().into()),
            8259571696196711189u64 => Ok(self.vote_option_4.clone().into()),
            8259570596685082978u64 => Ok(self.vote_option_5.clone().into()),
            18034020270891649474u64 => Ok(self.potential_votes.clone().into()),
            5777630607239142584u64 => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteChanged",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VotePassedEvent {
    pub details: MaybeUtf8String,
    pub param_1: MaybeUtf8String,
    pub team: u8,
    pub voteidx: u32,
}
impl VotePassedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VotePassedEvent {
            details: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(13353550034922503269u64),
                "details",
            )?,
            param_1: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(4990490691588242105u64),
                "param_1",
            )?,
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            voteidx: read_value::<u32>(
                stream,
                definition.get_entry(5777630607239142584u64),
                "voteidx",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            13353550034922503269u64 => Ok(self.details.clone().into()),
            4990490691588242105u64 => Ok(self.param_1.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            5777630607239142584u64 => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VotePassed",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteFailedEvent {
    pub team: u8,
    pub voteidx: u32,
}
impl VoteFailedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteFailedEvent {
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            voteidx: read_value::<u32>(
                stream,
                definition.get_entry(5777630607239142584u64),
                "voteidx",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18024489754618217260u64 => Ok(self.team.clone().into()),
            5777630607239142584u64 => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteFailed",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteCastEvent {
    pub vote_option: u8,
    pub team: u16,
    pub entity_id: u32,
    pub voteidx: u32,
}
impl VoteCastEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteCastEvent {
            vote_option: read_value::<u8>(
                stream,
                definition.get_entry(17670279370117350435u64),
                "vote_option",
            )?,
            team: read_value::<u16>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            entity_id: read_value::<u32>(
                stream,
                definition.get_entry(2085882069322833143u64),
                "entity_id",
            )?,
            voteidx: read_value::<u32>(
                stream,
                definition.get_entry(5777630607239142584u64),
                "voteidx",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17670279370117350435u64 => Ok(self.vote_option.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            2085882069322833143u64 => Ok(self.entity_id.clone().into()),
            5777630607239142584u64 => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteCast",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteOptionsEvent {
    pub count: u8,
    pub option_1: MaybeUtf8String,
    pub option_2: MaybeUtf8String,
    pub option_3: MaybeUtf8String,
    pub option_4: MaybeUtf8String,
    pub option_5: MaybeUtf8String,
    pub voteidx: u32,
}
impl VoteOptionsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteOptionsEvent {
            count: read_value::<u8>(
                stream,
                definition.get_entry(12818901015042040436u64),
                "count",
            )?,
            option_1: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14575392658851491999u64),
                "option_1",
            )?,
            option_2: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14575393758363120210u64),
                "option_2",
            )?,
            option_3: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14575394857874748421u64),
                "option_3",
            )?,
            option_4: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14575387161293350944u64),
                "option_4",
            )?,
            option_5: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(14575388260804979155u64),
                "option_5",
            )?,
            voteidx: read_value::<u32>(
                stream,
                definition.get_entry(5777630607239142584u64),
                "voteidx",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            12818901015042040436u64 => Ok(self.count.clone().into()),
            14575392658851491999u64 => Ok(self.option_1.clone().into()),
            14575393758363120210u64 => Ok(self.option_2.clone().into()),
            14575394857874748421u64 => Ok(self.option_3.clone().into()),
            14575387161293350944u64 => Ok(self.option_4.clone().into()),
            14575388260804979155u64 => Ok(self.option_5.clone().into()),
            5777630607239142584u64 => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteOptions",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplaySavedEvent {}
impl ReplaySavedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplaySavedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ReplaySaved",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EnteredPerformanceModeEvent {}
impl EnteredPerformanceModeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EnteredPerformanceModeEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EnteredPerformanceMode",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BrowseReplaysEvent {}
impl BrowseReplaysEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BrowseReplaysEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BrowseReplays",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayYoutubeStatsEvent {
    pub views: u32,
    pub likes: u32,
    pub favorited: u32,
}
impl ReplayYoutubeStatsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplayYoutubeStatsEvent {
            views: read_value::<u32>(
                stream,
                definition.get_entry(14625097093024684817u64),
                "views",
            )?,
            likes: read_value::<u32>(
                stream,
                definition.get_entry(9804554822404214111u64),
                "likes",
            )?,
            favorited: read_value::<u32>(
                stream,
                definition.get_entry(2653817720246189003u64),
                "favorited",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            14625097093024684817u64 => Ok(self.views.clone().into()),
            9804554822404214111u64 => Ok(self.likes.clone().into()),
            2653817720246189003u64 => Ok(self.favorited.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ReplayYoutubeStats",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InventoryUpdatedEvent {}
impl InventoryUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(InventoryUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "InventoryUpdated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CartUpdatedEvent {}
impl CartUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CartUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CartUpdated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StorePriceSheetUpdatedEvent {}
impl StorePriceSheetUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StorePriceSheetUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "StorePriceSheetUpdated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EconInventoryConnectedEvent {}
impl EconInventoryConnectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EconInventoryConnectedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EconInventoryConnected",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ItemSchemaInitializedEvent {}
impl ItemSchemaInitializedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ItemSchemaInitializedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ItemSchemaInitialized",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GcNewSessionEvent {}
impl GcNewSessionEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GcNewSessionEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GcNewSession",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GcLostSessionEvent {}
impl GcLostSessionEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GcLostSessionEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GcLostSession",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IntroFinishEvent {
    pub player: u16,
}
impl IntroFinishEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(IntroFinishEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "IntroFinish",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IntroNextCameraEvent {
    pub player: u16,
}
impl IntroNextCameraEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(IntroNextCameraEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "IntroNextCamera",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChangeClassEvent {
    pub user_id: u16,
    pub class: u16,
}
impl PlayerChangeClassEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerChangeClassEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            class: read_value::<u16>(
                stream,
                definition.get_entry(15066323702654938015u64),
                "class",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            15066323702654938015u64 => Ok(self.class.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerChangeClass",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TfMapTimeRemainingEvent {
    pub seconds: u32,
}
impl TfMapTimeRemainingEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TfMapTimeRemainingEvent {
            seconds: read_value::<u32>(
                stream,
                definition.get_entry(11456985514702388746u64),
                "seconds",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            11456985514702388746u64 => Ok(self.seconds.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TfMapTimeRemaining",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TfGameOverEvent {
    pub reason: MaybeUtf8String,
}
impl TfGameOverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TfGameOverEvent {
            reason: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(7343356632300987961u64),
                "reason",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            7343356632300987961u64 => Ok(self.reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TfGameOver",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CtfFlagCapturedEvent {
    pub capping_team: u16,
    pub capping_team_score: u16,
}
impl CtfFlagCapturedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CtfFlagCapturedEvent {
            capping_team: read_value::<u16>(
                stream,
                definition.get_entry(14568126206963925545u64),
                "capping_team",
            )?,
            capping_team_score: read_value::<u16>(
                stream,
                definition.get_entry(4559517251391003144u64),
                "capping_team_score",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            14568126206963925545u64 => Ok(self.capping_team.clone().into()),
            4559517251391003144u64 => Ok(self.capping_team_score.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CtfFlagCaptured",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointInitializedEvent {}
impl ControlPointInitializedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointInitializedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointInitialized",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateImagesEvent {
    pub index: u16,
}
impl ControlPointUpdateImagesEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUpdateImagesEvent {
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUpdateImages",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateLayoutEvent {
    pub index: u16,
}
impl ControlPointUpdateLayoutEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUpdateLayoutEvent {
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUpdateLayout",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateCappingEvent {
    pub index: u16,
}
impl ControlPointUpdateCappingEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUpdateCappingEvent {
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUpdateCapping",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateOwnerEvent {
    pub index: u16,
}
impl ControlPointUpdateOwnerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUpdateOwnerEvent {
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUpdateOwner",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointStartTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl ControlPointStartTouchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointStartTouchEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            area: read_value::<u16>(stream, definition.get_entry(9894459526856489156u64), "area")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            9894459526856489156u64 => Ok(self.area.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointStartTouch",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointEndTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl ControlPointEndTouchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointEndTouchEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            area: read_value::<u16>(stream, definition.get_entry(9894459526856489156u64), "area")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            9894459526856489156u64 => Ok(self.area.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointEndTouch",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointPulseElementEvent {
    pub player: u16,
}
impl ControlPointPulseElementEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointPulseElementEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointPulseElement",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointFakeCaptureEvent {
    pub player: u16,
    pub int_data: u16,
}
impl ControlPointFakeCaptureEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointFakeCaptureEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            int_data: read_value::<u16>(
                stream,
                definition.get_entry(17655270944800390939u64),
                "int_data",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            17655270944800390939u64 => Ok(self.int_data.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointFakeCapture",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointFakeCaptureMultiplierEvent {
    pub player: u16,
    pub int_data: u16,
}
impl ControlPointFakeCaptureMultiplierEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointFakeCaptureMultiplierEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            int_data: read_value::<u16>(
                stream,
                definition.get_entry(17655270944800390939u64),
                "int_data",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            17655270944800390939u64 => Ok(self.int_data.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointFakeCaptureMultiplier",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundSelectedEvent {
    pub round: MaybeUtf8String,
}
impl TeamPlayRoundSelectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundSelectedEvent {
            round: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(3536478298987656219u64),
                "round",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            3536478298987656219u64 => Ok(self.round.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundSelected",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundStartEvent {
    pub full_reset: bool,
}
impl TeamPlayRoundStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundStartEvent {
            full_reset: read_value::<bool>(
                stream,
                definition.get_entry(5520647792095461940u64),
                "full_reset",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5520647792095461940u64 => Ok(self.full_reset.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundStart",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundActiveEvent {}
impl TeamPlayRoundActiveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundActiveEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundActive",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWaitingBeginsEvent {}
impl TeamPlayWaitingBeginsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWaitingBeginsEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayWaitingBegins",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWaitingEndsEvent {}
impl TeamPlayWaitingEndsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWaitingEndsEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayWaitingEnds",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWaitingAboutToEndEvent {}
impl TeamPlayWaitingAboutToEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWaitingAboutToEndEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayWaitingAboutToEnd",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRestartRoundEvent {}
impl TeamPlayRestartRoundEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRestartRoundEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRestartRound",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayReadyRestartEvent {}
impl TeamPlayReadyRestartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayReadyRestartEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayReadyRestart",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundRestartSecondsEvent {
    pub seconds: u16,
}
impl TeamPlayRoundRestartSecondsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundRestartSecondsEvent {
            seconds: read_value::<u16>(
                stream,
                definition.get_entry(11456985514702388746u64),
                "seconds",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            11456985514702388746u64 => Ok(self.seconds.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundRestartSeconds",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTeamReadyEvent {
    pub team: u8,
}
impl TeamPlayTeamReadyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayTeamReadyEvent {
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18024489754618217260u64 => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayTeamReady",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundWinEvent {
    pub team: u8,
    pub win_reason: u8,
    pub flag_cap_limit: u16,
    pub full_round: u16,
    pub round_time: f32,
    pub losing_team_num_caps: u16,
    pub was_sudden_death: u8,
}
impl TeamPlayRoundWinEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundWinEvent {
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            win_reason: read_value::<u8>(
                stream,
                definition.get_entry(3803421146477351589u64),
                "win_reason",
            )?,
            flag_cap_limit: read_value::<u16>(
                stream,
                definition.get_entry(8774624256288798788u64),
                "flag_cap_limit",
            )?,
            full_round: read_value::<u16>(
                stream,
                definition.get_entry(11360866888973275703u64),
                "full_round",
            )?,
            round_time: read_value::<f32>(
                stream,
                definition.get_entry(17889722153966279533u64),
                "round_time",
            )?,
            losing_team_num_caps: read_value::<u16>(
                stream,
                definition.get_entry(1136537408317314454u64),
                "losing_team_num_caps",
            )?,
            was_sudden_death: read_value::<u8>(
                stream,
                definition.get_entry(16618607837222165313u64),
                "was_sudden_death",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18024489754618217260u64 => Ok(self.team.clone().into()),
            3803421146477351589u64 => Ok(self.win_reason.clone().into()),
            8774624256288798788u64 => Ok(self.flag_cap_limit.clone().into()),
            11360866888973275703u64 => Ok(self.full_round.clone().into()),
            17889722153966279533u64 => Ok(self.round_time.clone().into()),
            1136537408317314454u64 => Ok(self.losing_team_num_caps.clone().into()),
            16618607837222165313u64 => Ok(self.was_sudden_death.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundWin",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayUpdateTimerEvent {}
impl TeamPlayUpdateTimerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayUpdateTimerEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayUpdateTimer",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundStalemateEvent {
    pub reason: u8,
}
impl TeamPlayRoundStalemateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundStalemateEvent {
            reason: read_value::<u8>(
                stream,
                definition.get_entry(7343356632300987961u64),
                "reason",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            7343356632300987961u64 => Ok(self.reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundStalemate",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayOvertimeBeginEvent {}
impl TeamPlayOvertimeBeginEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayOvertimeBeginEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayOvertimeBegin",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayOvertimeEndEvent {}
impl TeamPlayOvertimeEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayOvertimeEndEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayOvertimeEnd",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlaySuddenDeathBeginEvent {}
impl TeamPlaySuddenDeathBeginEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlaySuddenDeathBeginEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlaySuddenDeathBegin",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlaySuddenDeathEndEvent {}
impl TeamPlaySuddenDeathEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlaySuddenDeathEndEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlaySuddenDeathEnd",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayGameOverEvent {
    pub reason: MaybeUtf8String,
}
impl TeamPlayGameOverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayGameOverEvent {
            reason: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(7343356632300987961u64),
                "reason",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            7343356632300987961u64 => Ok(self.reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayGameOver",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayMapTimeRemainingEvent {
    pub seconds: u16,
}
impl TeamPlayMapTimeRemainingEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayMapTimeRemainingEvent {
            seconds: read_value::<u16>(
                stream,
                definition.get_entry(11456985514702388746u64),
                "seconds",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            11456985514702388746u64 => Ok(self.seconds.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayMapTimeRemaining",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTimerFlashEvent {
    pub time_remaining: u16,
}
impl TeamPlayTimerFlashEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayTimerFlashEvent {
            time_remaining: read_value::<u16>(
                stream,
                definition.get_entry(4322130481848759231u64),
                "time_remaining",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            4322130481848759231u64 => Ok(self.time_remaining.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayTimerFlash",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTimerTimeAddedEvent {
    pub timer: u16,
    pub seconds_added: u16,
}
impl TeamPlayTimerTimeAddedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayTimerTimeAddedEvent {
            timer: read_value::<u16>(
                stream,
                definition.get_entry(2968869876298967810u64),
                "timer",
            )?,
            seconds_added: read_value::<u16>(
                stream,
                definition.get_entry(5796598285704248091u64),
                "seconds_added",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            2968869876298967810u64 => Ok(self.timer.clone().into()),
            5796598285704248091u64 => Ok(self.seconds_added.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayTimerTimeAdded",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointStartCaptureEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
    pub cap_team: u8,
    pub cappers: MaybeUtf8String,
    pub cap_time: f32,
}
impl TeamPlayPointStartCaptureEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayPointStartCaptureEvent {
            cp: read_value::<u8>(stream, definition.get_entry(622127901357767142u64), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(5054899044858273797u64),
                "cp_name",
            )?,
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            cap_team: read_value::<u8>(
                stream,
                definition.get_entry(9316766665943420626u64),
                "cap_team",
            )?,
            cappers: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(11245576010855792123u64),
                "cappers",
            )?,
            cap_time: read_value::<f32>(
                stream,
                definition.get_entry(6747273962184890386u64),
                "cap_time",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            622127901357767142u64 => Ok(self.cp.clone().into()),
            5054899044858273797u64 => Ok(self.cp_name.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            9316766665943420626u64 => Ok(self.cap_team.clone().into()),
            11245576010855792123u64 => Ok(self.cappers.clone().into()),
            6747273962184890386u64 => Ok(self.cap_time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayPointStartCapture",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointCapturedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
    pub cappers: MaybeUtf8String,
}
impl TeamPlayPointCapturedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayPointCapturedEvent {
            cp: read_value::<u8>(stream, definition.get_entry(622127901357767142u64), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(5054899044858273797u64),
                "cp_name",
            )?,
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            cappers: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(11245576010855792123u64),
                "cappers",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            622127901357767142u64 => Ok(self.cp.clone().into()),
            5054899044858273797u64 => Ok(self.cp_name.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            11245576010855792123u64 => Ok(self.cappers.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayPointCaptured",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointLockedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
}
impl TeamPlayPointLockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayPointLockedEvent {
            cp: read_value::<u8>(stream, definition.get_entry(622127901357767142u64), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(5054899044858273797u64),
                "cp_name",
            )?,
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            622127901357767142u64 => Ok(self.cp.clone().into()),
            5054899044858273797u64 => Ok(self.cp_name.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayPointLocked",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointUnlockedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
}
impl TeamPlayPointUnlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayPointUnlockedEvent {
            cp: read_value::<u8>(stream, definition.get_entry(622127901357767142u64), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(5054899044858273797u64),
                "cp_name",
            )?,
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            622127901357767142u64 => Ok(self.cp.clone().into()),
            5054899044858273797u64 => Ok(self.cp_name.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayPointUnlocked",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayCaptureBrokenEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub time_remaining: f32,
}
impl TeamPlayCaptureBrokenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayCaptureBrokenEvent {
            cp: read_value::<u8>(stream, definition.get_entry(622127901357767142u64), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(5054899044858273797u64),
                "cp_name",
            )?,
            time_remaining: read_value::<f32>(
                stream,
                definition.get_entry(4322130481848759231u64),
                "time_remaining",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            622127901357767142u64 => Ok(self.cp.clone().into()),
            5054899044858273797u64 => Ok(self.cp_name.clone().into()),
            4322130481848759231u64 => Ok(self.time_remaining.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayCaptureBroken",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayCaptureBlockedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub blocker: u8,
    pub victim: u8,
}
impl TeamPlayCaptureBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayCaptureBlockedEvent {
            cp: read_value::<u8>(stream, definition.get_entry(622127901357767142u64), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(5054899044858273797u64),
                "cp_name",
            )?,
            blocker: read_value::<u8>(
                stream,
                definition.get_entry(9150196623075249301u64),
                "blocker",
            )?,
            victim: read_value::<u8>(
                stream,
                definition.get_entry(3120917251440744469u64),
                "victim",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            622127901357767142u64 => Ok(self.cp.clone().into()),
            5054899044858273797u64 => Ok(self.cp_name.clone().into()),
            9150196623075249301u64 => Ok(self.blocker.clone().into()),
            3120917251440744469u64 => Ok(self.victim.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayCaptureBlocked",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayFlagEventEvent {
    pub player: u16,
    pub carrier: u16,
    pub event_type: u16,
    pub home: u8,
    pub team: u8,
}
impl TeamPlayFlagEventEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayFlagEventEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            carrier: read_value::<u16>(
                stream,
                definition.get_entry(6986802915220291447u64),
                "carrier",
            )?,
            event_type: read_value::<u16>(
                stream,
                definition.get_entry(5234087001556401511u64),
                "event_type",
            )?,
            home: read_value::<u8>(stream, definition.get_entry(4624382957487889774u64), "home")?,
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            6986802915220291447u64 => Ok(self.carrier.clone().into()),
            5234087001556401511u64 => Ok(self.event_type.clone().into()),
            4624382957487889774u64 => Ok(self.home.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayFlagEvent",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
    pub cappers: MaybeUtf8String,
    pub flag_cap_limit: u16,
    pub blue_score: u16,
    pub red_score: u16,
    pub blue_score_prev: u16,
    pub red_score_prev: u16,
    pub round_complete: u16,
    pub rounds_remaining: u16,
    pub player_1: u16,
    pub player_1_points: u16,
    pub player_2: u16,
    pub player_2_points: u16,
    pub player_3: u16,
    pub player_3_points: u16,
    pub kill_stream_player_1: u16,
    pub kill_stream_player_1_count: u16,
    pub game_over: u8,
}
impl TeamPlayWinPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWinPanelEvent {
            panel_style: read_value::<u8>(
                stream,
                definition.get_entry(3076948889484157827u64),
                "panel_style",
            )?,
            winning_team: read_value::<u8>(
                stream,
                definition.get_entry(12760025138952247085u64),
                "winning_team",
            )?,
            win_reason: read_value::<u8>(
                stream,
                definition.get_entry(3803421146477351589u64),
                "win_reason",
            )?,
            cappers: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(11245576010855792123u64),
                "cappers",
            )?,
            flag_cap_limit: read_value::<u16>(
                stream,
                definition.get_entry(8774624256288798788u64),
                "flag_cap_limit",
            )?,
            blue_score: read_value::<u16>(
                stream,
                definition.get_entry(12881173953286901124u64),
                "blue_score",
            )?,
            red_score: read_value::<u16>(
                stream,
                definition.get_entry(1115892277897790273u64),
                "red_score",
            )?,
            blue_score_prev: read_value::<u16>(
                stream,
                definition.get_entry(12382344057664863052u64),
                "blue_score_prev",
            )?,
            red_score_prev: read_value::<u16>(
                stream,
                definition.get_entry(9684982604781518527u64),
                "red_score_prev",
            )?,
            round_complete: read_value::<u16>(
                stream,
                definition.get_entry(12165785943437780003u64),
                "round_complete",
            )?,
            rounds_remaining: read_value::<u16>(
                stream,
                definition.get_entry(10434023640517397027u64),
                "rounds_remaining",
            )?,
            player_1: read_value::<u16>(
                stream,
                definition.get_entry(2316304829487618708u64),
                "player_1",
            )?,
            player_1_points: read_value::<u16>(
                stream,
                definition.get_entry(5201979123115161946u64),
                "player_1_points",
            )?,
            player_2: read_value::<u16>(
                stream,
                definition.get_entry(2316308128022503341u64),
                "player_2",
            )?,
            player_2_points: read_value::<u16>(
                stream,
                definition.get_entry(17826909355416759905u64),
                "player_2_points",
            )?,
            player_3: read_value::<u16>(
                stream,
                definition.get_entry(2316307028510875130u64),
                "player_3",
            )?,
            player_3_points: read_value::<u16>(
                stream,
                definition.get_entry(3496125461192453068u64),
                "player_3_points",
            )?,
            kill_stream_player_1: read_value::<u16>(
                stream,
                definition.get_entry(5168633764732789397u64),
                "kill_stream_player_1",
            )?,
            kill_stream_player_1_count: read_value::<u16>(
                stream,
                definition.get_entry(15596392604614003293u64),
                "kill_stream_player_1_count",
            )?,
            game_over: read_value::<u8>(
                stream,
                definition.get_entry(17040732377939006530u64),
                "game_over",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            3076948889484157827u64 => Ok(self.panel_style.clone().into()),
            12760025138952247085u64 => Ok(self.winning_team.clone().into()),
            3803421146477351589u64 => Ok(self.win_reason.clone().into()),
            11245576010855792123u64 => Ok(self.cappers.clone().into()),
            8774624256288798788u64 => Ok(self.flag_cap_limit.clone().into()),
            12881173953286901124u64 => Ok(self.blue_score.clone().into()),
            1115892277897790273u64 => Ok(self.red_score.clone().into()),
            12382344057664863052u64 => Ok(self.blue_score_prev.clone().into()),
            9684982604781518527u64 => Ok(self.red_score_prev.clone().into()),
            12165785943437780003u64 => Ok(self.round_complete.clone().into()),
            10434023640517397027u64 => Ok(self.rounds_remaining.clone().into()),
            2316304829487618708u64 => Ok(self.player_1.clone().into()),
            5201979123115161946u64 => Ok(self.player_1_points.clone().into()),
            2316308128022503341u64 => Ok(self.player_2.clone().into()),
            17826909355416759905u64 => Ok(self.player_2_points.clone().into()),
            2316307028510875130u64 => Ok(self.player_3.clone().into()),
            3496125461192453068u64 => Ok(self.player_3_points.clone().into()),
            5168633764732789397u64 => Ok(self.kill_stream_player_1.clone().into()),
            15596392604614003293u64 => Ok(self.kill_stream_player_1_count.clone().into()),
            17040732377939006530u64 => Ok(self.game_over.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayWinPanel",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTeamBalancedPlayerEvent {
    pub player: u16,
    pub team: u8,
}
impl TeamPlayTeamBalancedPlayerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayTeamBalancedPlayerEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            18024489754618217260u64 => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayTeamBalancedPlayer",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlaySetupFinishedEvent {}
impl TeamPlaySetupFinishedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlaySetupFinishedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlaySetupFinished",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayAlertEvent {
    pub alert_type: u16,
}
impl TeamPlayAlertEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayAlertEvent {
            alert_type: read_value::<u16>(
                stream,
                definition.get_entry(5455556148004663490u64),
                "alert_type",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5455556148004663490u64 => Ok(self.alert_type.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayAlert",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TrainingCompleteEvent {
    pub next_map: MaybeUtf8String,
    pub map: MaybeUtf8String,
    pub text: MaybeUtf8String,
}
impl TrainingCompleteEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TrainingCompleteEvent {
            next_map: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(13669032538247983969u64),
                "next_map",
            )?,
            map: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(580780841256168849u64),
                "map",
            )?,
            text: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(18015793717152399486u64),
                "text",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            13669032538247983969u64 => Ok(self.next_map.clone().into()),
            580780841256168849u64 => Ok(self.map.clone().into()),
            18015793717152399486u64 => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TrainingComplete",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowFreezePanelEvent {
    pub killer: u16,
}
impl ShowFreezePanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ShowFreezePanelEvent {
            killer: read_value::<u16>(
                stream,
                definition.get_entry(7927307102854403058u64),
                "killer",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            7927307102854403058u64 => Ok(self.killer.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ShowFreezePanel",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HideFreezePanelEvent {}
impl HideFreezePanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HideFreezePanelEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HideFreezePanel",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FreezeCamStartedEvent {}
impl FreezeCamStartedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(FreezeCamStartedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "FreezeCamStarted",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChangeTeamEvent {}
impl LocalPlayerChangeTeamEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChangeTeamEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerChangeTeam",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerScoreChangedEvent {
    pub score: u16,
}
impl LocalPlayerScoreChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerScoreChangedEvent {
            score: read_value::<u16>(
                stream,
                definition.get_entry(13911166232573650165u64),
                "score",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            13911166232573650165u64 => Ok(self.score.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerScoreChanged",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChangeClassEvent {}
impl LocalPlayerChangeClassEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChangeClassEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerChangeClass",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerRespawnEvent {}
impl LocalPlayerRespawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerRespawnEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerRespawn",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BuildingInfoChangedEvent {
    pub building_type: u8,
    pub object_mode: u8,
    pub remove: u8,
}
impl BuildingInfoChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BuildingInfoChangedEvent {
            building_type: read_value::<u8>(
                stream,
                definition.get_entry(11928805672381350942u64),
                "building_type",
            )?,
            object_mode: read_value::<u8>(
                stream,
                definition.get_entry(10575483099853176920u64),
                "object_mode",
            )?,
            remove: read_value::<u8>(
                stream,
                definition.get_entry(18444559702367749501u64),
                "remove",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            11928805672381350942u64 => Ok(self.building_type.clone().into()),
            10575483099853176920u64 => Ok(self.object_mode.clone().into()),
            18444559702367749501u64 => Ok(self.remove.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BuildingInfoChanged",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChangeDisguiseEvent {
    pub disguised: bool,
}
impl LocalPlayerChangeDisguiseEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChangeDisguiseEvent {
            disguised: read_value::<bool>(
                stream,
                definition.get_entry(12536453236572645170u64),
                "disguised",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            12536453236572645170u64 => Ok(self.disguised.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerChangeDisguise",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerAccountChangedEvent {
    pub old_value: u16,
    pub new_value: u16,
}
impl PlayerAccountChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerAccountChangedEvent {
            old_value: read_value::<u16>(
                stream,
                definition.get_entry(12791125728741340200u64),
                "old_value",
            )?,
            new_value: read_value::<u16>(
                stream,
                definition.get_entry(4557144879184858675u64),
                "new_value",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            12791125728741340200u64 => Ok(self.old_value.clone().into()),
            4557144879184858675u64 => Ok(self.new_value.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerAccountChanged",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpyPdaResetEvent {}
impl SpyPdaResetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SpyPdaResetEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SpyPdaReset",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FlagStatusUpdateEvent {
    pub user_id: u16,
    pub ent_index: u32,
}
impl FlagStatusUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(FlagStatusUpdateEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            ent_index: read_value::<u32>(
                stream,
                definition.get_entry(17554918082946193550u64),
                "ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            17554918082946193550u64 => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "FlagStatusUpdate",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerStatsUpdatedEvent {
    pub force_upload: bool,
}
impl PlayerStatsUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerStatsUpdatedEvent {
            force_upload: read_value::<bool>(
                stream,
                definition.get_entry(9059172244575932677u64),
                "force_upload",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9059172244575932677u64 => Ok(self.force_upload.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerStatsUpdated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayingCommentaryEvent {}
impl PlayingCommentaryEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayingCommentaryEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayingCommentary",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChargeDeployedEvent {
    pub user_id: u16,
    pub target_id: u16,
}
impl PlayerChargeDeployedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerChargeDeployedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            target_id: read_value::<u16>(
                stream,
                definition.get_entry(10554794794880602069u64),
                "target_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            10554794794880602069u64 => Ok(self.target_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerChargeDeployed",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBuiltObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl PlayerBuiltObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerBuiltObjectEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            object: read_value::<u16>(
                stream,
                definition.get_entry(10231808476453998586u64),
                "object",
            )?,
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            10231808476453998586u64 => Ok(self.object.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerBuiltObject",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUpgradedObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
    pub is_builder: bool,
}
impl PlayerUpgradedObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerUpgradedObjectEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            object: read_value::<u16>(
                stream,
                definition.get_entry(10231808476453998586u64),
                "object",
            )?,
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
            is_builder: read_value::<bool>(
                stream,
                definition.get_entry(16922823115528993136u64),
                "is_builder",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            10231808476453998586u64 => Ok(self.object.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            16922823115528993136u64 => Ok(self.is_builder.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerUpgradedObject",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerCarryObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl PlayerCarryObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerCarryObjectEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            object: read_value::<u16>(
                stream,
                definition.get_entry(10231808476453998586u64),
                "object",
            )?,
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            10231808476453998586u64 => Ok(self.object.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerCarryObject",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDropObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl PlayerDropObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDropObjectEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            object: read_value::<u16>(
                stream,
                definition.get_entry(10231808476453998586u64),
                "object",
            )?,
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            10231808476453998586u64 => Ok(self.object.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDropObject",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectRemovedEvent {
    pub user_id: u16,
    pub object_type: u16,
    pub index: u16,
}
impl ObjectRemovedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ObjectRemovedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            object_type: read_value::<u16>(
                stream,
                definition.get_entry(6025769100165056826u64),
                "object_type",
            )?,
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            6025769100165056826u64 => Ok(self.object_type.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ObjectRemoved",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDestroyedEvent {
    pub user_id: u16,
    pub attacker: u16,
    pub assister: u16,
    pub weapon: MaybeUtf8String,
    pub weapon_id: u16,
    pub object_type: u16,
    pub index: u16,
    pub was_building: bool,
}
impl ObjectDestroyedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ObjectDestroyedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            attacker: read_value::<u16>(
                stream,
                definition.get_entry(7198542740550218478u64),
                "attacker",
            )?,
            assister: read_value::<u16>(
                stream,
                definition.get_entry(17978308754419261977u64),
                "assister",
            )?,
            weapon: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(11580461223051554305u64),
                "weapon",
            )?,
            weapon_id: read_value::<u16>(
                stream,
                definition.get_entry(5542695206485500884u64),
                "weapon_id",
            )?,
            object_type: read_value::<u16>(
                stream,
                definition.get_entry(6025769100165056826u64),
                "object_type",
            )?,
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
            was_building: read_value::<bool>(
                stream,
                definition.get_entry(13090762770129151523u64),
                "was_building",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            7198542740550218478u64 => Ok(self.attacker.clone().into()),
            17978308754419261977u64 => Ok(self.assister.clone().into()),
            11580461223051554305u64 => Ok(self.weapon.clone().into()),
            5542695206485500884u64 => Ok(self.weapon_id.clone().into()),
            6025769100165056826u64 => Ok(self.object_type.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            13090762770129151523u64 => Ok(self.was_building.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ObjectDestroyed",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDetonatedEvent {
    pub user_id: u16,
    pub object_type: u16,
    pub index: u16,
}
impl ObjectDetonatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ObjectDetonatedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            object_type: read_value::<u16>(
                stream,
                definition.get_entry(6025769100165056826u64),
                "object_type",
            )?,
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            6025769100165056826u64 => Ok(self.object_type.clone().into()),
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ObjectDetonated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementEarnedEvent {
    pub player: u8,
    pub achievement: u16,
}
impl AchievementEarnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(AchievementEarnedEvent {
            player: read_value::<u8>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            achievement: read_value::<u16>(
                stream,
                definition.get_entry(7071905471600864408u64),
                "achievement",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            7071905471600864408u64 => Ok(self.achievement.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "AchievementEarned",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpecTargetUpdatedEvent {}
impl SpecTargetUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SpecTargetUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SpecTargetUpdated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TournamentStateUpdateEvent {
    pub user_id: u16,
    pub name_change: bool,
    pub ready_state: u16,
    pub new_name: MaybeUtf8String,
}
impl TournamentStateUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TournamentStateUpdateEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            name_change: read_value::<bool>(
                stream,
                definition.get_entry(2507746477842667538u64),
                "name_change",
            )?,
            ready_state: read_value::<u16>(
                stream,
                definition.get_entry(14125289189230288425u64),
                "ready_state",
            )?,
            new_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(8904377156710117674u64),
                "new_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            2507746477842667538u64 => Ok(self.name_change.clone().into()),
            14125289189230288425u64 => Ok(self.ready_state.clone().into()),
            8904377156710117674u64 => Ok(self.new_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TournamentStateUpdate",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TournamentEnableCountdownEvent {}
impl TournamentEnableCountdownEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TournamentEnableCountdownEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TournamentEnableCountdown",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerCalledForMedicEvent {
    pub user_id: u16,
}
impl PlayerCalledForMedicEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerCalledForMedicEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerCalledForMedic",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerAskedForBallEvent {
    pub user_id: u16,
}
impl PlayerAskedForBallEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerAskedForBallEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerAskedForBall",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerBecameObserverEvent {}
impl LocalPlayerBecameObserverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerBecameObserverEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerBecameObserver",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerIgnitedInvEvent {
    pub pyro_ent_index: u8,
    pub victim_ent_index: u8,
    pub medic_ent_index: u8,
}
impl PlayerIgnitedInvEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerIgnitedInvEvent {
            pyro_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(2481028106190820025u64),
                "pyro_ent_index",
            )?,
            victim_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(7608903543976233025u64),
                "victim_ent_index",
            )?,
            medic_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(9117426914612648485u64),
                "medic_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            2481028106190820025u64 => Ok(self.pyro_ent_index.clone().into()),
            7608903543976233025u64 => Ok(self.victim_ent_index.clone().into()),
            9117426914612648485u64 => Ok(self.medic_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerIgnitedInv",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerIgnitedEvent {
    pub pyro_ent_index: u8,
    pub victim_ent_index: u8,
    pub weapon_id: u8,
}
impl PlayerIgnitedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerIgnitedEvent {
            pyro_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(2481028106190820025u64),
                "pyro_ent_index",
            )?,
            victim_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(7608903543976233025u64),
                "victim_ent_index",
            )?,
            weapon_id: read_value::<u8>(
                stream,
                definition.get_entry(5542695206485500884u64),
                "weapon_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            2481028106190820025u64 => Ok(self.pyro_ent_index.clone().into()),
            7608903543976233025u64 => Ok(self.victim_ent_index.clone().into()),
            5542695206485500884u64 => Ok(self.weapon_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerIgnited",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerExtinguishedEvent {
    pub victim: u8,
    pub healer: u8,
    pub item_definition_index: u16,
}
impl PlayerExtinguishedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerExtinguishedEvent {
            victim: read_value::<u8>(
                stream,
                definition.get_entry(3120917251440744469u64),
                "victim",
            )?,
            healer: read_value::<u8>(
                stream,
                definition.get_entry(9195440821534910520u64),
                "healer",
            )?,
            item_definition_index: read_value::<u16>(
                stream,
                definition.get_entry(4926523576391011283u64),
                "item_definition_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            3120917251440744469u64 => Ok(self.victim.clone().into()),
            9195440821534910520u64 => Ok(self.healer.clone().into()),
            4926523576391011283u64 => Ok(self.item_definition_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerExtinguished",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerTeleportedEvent {
    pub user_id: u16,
    pub builder_id: u16,
    pub dist: f32,
}
impl PlayerTeleportedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerTeleportedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            builder_id: read_value::<u16>(
                stream,
                definition.get_entry(3387979893847309533u64),
                "builder_id",
            )?,
            dist: read_value::<f32>(
                stream,
                definition.get_entry(14574961654905149033u64),
                "dist",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            3387979893847309533u64 => Ok(self.builder_id.clone().into()),
            14574961654905149033u64 => Ok(self.dist.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerTeleported",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealedMedicCallEvent {
    pub user_id: u16,
}
impl PlayerHealedMedicCallEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHealedMedicCallEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHealedMedicCall",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChargeReadyEvent {}
impl LocalPlayerChargeReadyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChargeReadyEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerChargeReady",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerWindDownEvent {}
impl LocalPlayerWindDownEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerWindDownEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerWindDown",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerInvulnedEvent {
    pub user_id: u16,
    pub medic_user_id: u16,
}
impl PlayerInvulnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerInvulnedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            medic_user_id: read_value::<u16>(
                stream,
                definition.get_entry(1211611822706104928u64),
                "medic_user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            1211611822706104928u64 => Ok(self.medic_user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerInvulned",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscortSpeedEvent {
    pub team: u8,
    pub speed: u8,
    pub players: u8,
}
impl EscortSpeedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EscortSpeedEvent {
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            speed: read_value::<u8>(
                stream,
                definition.get_entry(2486349329025994304u64),
                "speed",
            )?,
            players: read_value::<u8>(
                stream,
                definition.get_entry(11016838732397775657u64),
                "players",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18024489754618217260u64 => Ok(self.team.clone().into()),
            2486349329025994304u64 => Ok(self.speed.clone().into()),
            11016838732397775657u64 => Ok(self.players.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EscortSpeed",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscortProgressEvent {
    pub team: u8,
    pub progress: f32,
    pub reset: bool,
}
impl EscortProgressEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EscortProgressEvent {
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            progress: read_value::<f32>(
                stream,
                definition.get_entry(17983033035584588230u64),
                "progress",
            )?,
            reset: read_value::<bool>(
                stream,
                definition.get_entry(1086335023529244512u64),
                "reset",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18024489754618217260u64 => Ok(self.team.clone().into()),
            17983033035584588230u64 => Ok(self.progress.clone().into()),
            1086335023529244512u64 => Ok(self.reset.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EscortProgress",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscortRecedeEvent {
    pub team: u8,
    pub recede_time: f32,
}
impl EscortRecedeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EscortRecedeEvent {
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            recede_time: read_value::<f32>(
                stream,
                definition.get_entry(12986815124312535790u64),
                "recede_time",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18024489754618217260u64 => Ok(self.team.clone().into()),
            12986815124312535790u64 => Ok(self.recede_time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EscortRecede",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameUIActivatedEvent {}
impl GameUIActivatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameUIActivatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameUIActivated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameUIHiddenEvent {}
impl GameUIHiddenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameUIHiddenEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameUIHidden",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerEscortScoreEvent {
    pub player: u8,
    pub points: u8,
}
impl PlayerEscortScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerEscortScoreEvent {
            player: read_value::<u8>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            points: read_value::<u8>(
                stream,
                definition.get_entry(15925482014108518566u64),
                "points",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            15925482014108518566u64 => Ok(self.points.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerEscortScore",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealOnHitEvent {
    pub amount: u16,
    pub ent_index: u8,
    pub weapon_def_index: u32,
}
impl PlayerHealOnHitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHealOnHitEvent {
            amount: read_value::<u16>(
                stream,
                definition.get_entry(9301057475299076457u64),
                "amount",
            )?,
            ent_index: read_value::<u8>(
                stream,
                definition.get_entry(17554918082946193550u64),
                "ent_index",
            )?,
            weapon_def_index: read_value::<u32>(
                stream,
                definition.get_entry(4132306594868589054u64),
                "weapon_def_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9301057475299076457u64 => Ok(self.amount.clone().into()),
            17554918082946193550u64 => Ok(self.ent_index.clone().into()),
            4132306594868589054u64 => Ok(self.weapon_def_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHealOnHit",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerStealSandvichEvent {
    pub owner: u16,
    pub target: u16,
}
impl PlayerStealSandvichEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerStealSandvichEvent {
            owner: read_value::<u16>(
                stream,
                definition.get_entry(12002927925776846068u64),
                "owner",
            )?,
            target: read_value::<u16>(
                stream,
                definition.get_entry(1653916590517707752u64),
                "target",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            12002927925776846068u64 => Ok(self.owner.clone().into()),
            1653916590517707752u64 => Ok(self.target.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerStealSandvich",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowClassLayoutEvent {
    pub show: bool,
}
impl ShowClassLayoutEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ShowClassLayoutEvent {
            show: read_value::<bool>(stream, definition.get_entry(5106060638016120252u64), "show")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5106060638016120252u64 => Ok(self.show.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ShowClassLayout",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowVsPanelEvent {
    pub show: bool,
}
impl ShowVsPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ShowVsPanelEvent {
            show: read_value::<bool>(stream, definition.get_entry(5106060638016120252u64), "show")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5106060638016120252u64 => Ok(self.show.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ShowVsPanel",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDamagedEvent {
    pub amount: u16,
    pub kind: u32,
}
impl PlayerDamagedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDamagedEvent {
            amount: read_value::<u16>(
                stream,
                definition.get_entry(9301057475299076457u64),
                "amount",
            )?,
            kind: read_value::<u32>(
                stream,
                definition.get_entry(12075340201627130925u64),
                "kind",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9301057475299076457u64 => Ok(self.amount.clone().into()),
            12075340201627130925u64 => Ok(self.kind.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDamaged",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaPlayerNotificationEvent {
    pub player: u8,
    pub message: u8,
}
impl ArenaPlayerNotificationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ArenaPlayerNotificationEvent {
            player: read_value::<u8>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            message: read_value::<u8>(
                stream,
                definition.get_entry(6080987277291999908u64),
                "message",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            6080987277291999908u64 => Ok(self.message.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArenaPlayerNotification",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaMatchMaxStreakEvent {
    pub team: u8,
    pub streak: u8,
}
impl ArenaMatchMaxStreakEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ArenaMatchMaxStreakEvent {
            team: read_value::<u8>(
                stream,
                definition.get_entry(18024489754618217260u64),
                "team",
            )?,
            streak: read_value::<u8>(
                stream,
                definition.get_entry(5722439984700485459u64),
                "streak",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            18024489754618217260u64 => Ok(self.team.clone().into()),
            5722439984700485459u64 => Ok(self.streak.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArenaMatchMaxStreak",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaRoundStartEvent {}
impl ArenaRoundStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ArenaRoundStartEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArenaRoundStart",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
    pub cappers: MaybeUtf8String,
    pub flag_cap_limit: u16,
    pub blue_score: u16,
    pub red_score: u16,
    pub blue_score_prev: u16,
    pub red_score_prev: u16,
    pub round_complete: u16,
    pub player_1: u16,
    pub player_1_damage: u16,
    pub player_1_healing: u16,
    pub player_1_lifetime: u16,
    pub player_1_kills: u16,
    pub player_2: u16,
    pub player_2_damage: u16,
    pub player_2_healing: u16,
    pub player_2_lifetime: u16,
    pub player_2_kills: u16,
    pub player_3: u16,
    pub player_3_damage: u16,
    pub player_3_healing: u16,
    pub player_3_lifetime: u16,
    pub player_3_kills: u16,
    pub player_4: u16,
    pub player_4_damage: u16,
    pub player_4_healing: u16,
    pub player_4_lifetime: u16,
    pub player_4_kills: u16,
    pub player_5: u16,
    pub player_5_damage: u16,
    pub player_5_healing: u16,
    pub player_5_lifetime: u16,
    pub player_5_kills: u16,
    pub player_6: u16,
    pub player_6_damage: u16,
    pub player_6_healing: u16,
    pub player_6_lifetime: u16,
    pub player_6_kills: u16,
}
impl ArenaWinPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ArenaWinPanelEvent {
            panel_style: read_value::<u8>(
                stream,
                definition.get_entry(3076948889484157827u64),
                "panel_style",
            )?,
            winning_team: read_value::<u8>(
                stream,
                definition.get_entry(12760025138952247085u64),
                "winning_team",
            )?,
            win_reason: read_value::<u8>(
                stream,
                definition.get_entry(3803421146477351589u64),
                "win_reason",
            )?,
            cappers: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(11245576010855792123u64),
                "cappers",
            )?,
            flag_cap_limit: read_value::<u16>(
                stream,
                definition.get_entry(8774624256288798788u64),
                "flag_cap_limit",
            )?,
            blue_score: read_value::<u16>(
                stream,
                definition.get_entry(12881173953286901124u64),
                "blue_score",
            )?,
            red_score: read_value::<u16>(
                stream,
                definition.get_entry(1115892277897790273u64),
                "red_score",
            )?,
            blue_score_prev: read_value::<u16>(
                stream,
                definition.get_entry(12382344057664863052u64),
                "blue_score_prev",
            )?,
            red_score_prev: read_value::<u16>(
                stream,
                definition.get_entry(9684982604781518527u64),
                "red_score_prev",
            )?,
            round_complete: read_value::<u16>(
                stream,
                definition.get_entry(12165785943437780003u64),
                "round_complete",
            )?,
            player_1: read_value::<u16>(
                stream,
                definition.get_entry(2316304829487618708u64),
                "player_1",
            )?,
            player_1_damage: read_value::<u16>(
                stream,
                definition.get_entry(10394500961929970236u64),
                "player_1_damage",
            )?,
            player_1_healing: read_value::<u16>(
                stream,
                definition.get_entry(4434975577362185857u64),
                "player_1_healing",
            )?,
            player_1_lifetime: read_value::<u16>(
                stream,
                definition.get_entry(9525810424766332458u64),
                "player_1_lifetime",
            )?,
            player_1_kills: read_value::<u16>(
                stream,
                definition.get_entry(9144050188623277698u64),
                "player_1_kills",
            )?,
            player_2: read_value::<u16>(
                stream,
                definition.get_entry(2316308128022503341u64),
                "player_2",
            )?,
            player_2_damage: read_value::<u16>(
                stream,
                definition.get_entry(10233858120128677491u64),
                "player_2_damage",
            )?,
            player_2_healing: read_value::<u16>(
                stream,
                definition.get_entry(14185483197478656496u64),
                "player_2_healing",
            )?,
            player_2_lifetime: read_value::<u16>(
                stream,
                definition.get_entry(4136489886055437333u64),
                "player_2_lifetime",
            )?,
            player_2_kills: read_value::<u16>(
                stream,
                definition.get_entry(4674312054664562187u64),
                "player_2_kills",
            )?,
            player_3: read_value::<u16>(
                stream,
                definition.get_entry(2316307028510875130u64),
                "player_3",
            )?,
            player_3_damage: read_value::<u16>(
                stream,
                definition.get_entry(39363359054721358u64),
                "player_3_damage",
            )?,
            player_3_healing: read_value::<u16>(
                stream,
                definition.get_entry(3236287490998805827u64),
                "player_3_healing",
            )?,
            player_3_lifetime: read_value::<u16>(
                stream,
                definition.get_entry(14621494996524927732u64),
                "player_3_lifetime",
            )?,
            player_3_kills: read_value::<u16>(
                stream,
                definition.get_entry(3863298646261365396u64),
                "player_3_kills",
            )?,
            player_4: read_value::<u16>(
                stream,
                definition.get_entry(2316301530952734075u64),
                "player_4",
            )?,
            player_4_damage: read_value::<u16>(
                stream,
                definition.get_entry(10597470269304895533u64),
                "player_4_damage",
            )?,
            player_4_healing: read_value::<u16>(
                stream,
                definition.get_entry(16447535156948377850u64),
                "player_4_healing",
            )?,
            player_4_lifetime: read_value::<u16>(
                stream,
                definition.get_entry(7059298629593792111u64),
                "player_4_lifetime",
            )?,
            player_4_kills: read_value::<u16>(
                stream,
                definition.get_entry(3991910781166784861u64),
                "player_4_kills",
            )?,
            player_5: read_value::<u16>(
                stream,
                definition.get_entry(2316300431441105864u64),
                "player_5",
            )?,
            player_5_damage: read_value::<u16>(
                stream,
                definition.get_entry(5792678822030670832u64),
                "player_5_damage",
            )?,
            player_5_healing: read_value::<u16>(
                stream,
                definition.get_entry(10549804756926849709u64),
                "player_5_healing",
            )?,
            player_5_lifetime: read_value::<u16>(
                stream,
                definition.get_entry(5937207214365195550u64),
                "player_5_lifetime",
            )?,
            player_5_kills: read_value::<u16>(
                stream,
                definition.get_entry(12801315132847255054u64),
                "player_5_kills",
            )?,
            player_6: read_value::<u16>(
                stream,
                definition.get_entry(2316303729975990497u64),
                "player_6",
            )?,
            player_6_damage: read_value::<u16>(
                stream,
                definition.get_entry(11907089275040254807u64),
                "player_6_damage",
            )?,
            player_6_healing: read_value::<u16>(
                stream,
                definition.get_entry(12917045825324352380u64),
                "player_6_healing",
            )?,
            player_6_lifetime: read_value::<u16>(
                stream,
                definition.get_entry(2459117320880115529u64),
                "player_6_lifetime",
            )?,
            player_6_kills: read_value::<u16>(
                stream,
                definition.get_entry(9062282318485088775u64),
                "player_6_kills",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            3076948889484157827u64 => Ok(self.panel_style.clone().into()),
            12760025138952247085u64 => Ok(self.winning_team.clone().into()),
            3803421146477351589u64 => Ok(self.win_reason.clone().into()),
            11245576010855792123u64 => Ok(self.cappers.clone().into()),
            8774624256288798788u64 => Ok(self.flag_cap_limit.clone().into()),
            12881173953286901124u64 => Ok(self.blue_score.clone().into()),
            1115892277897790273u64 => Ok(self.red_score.clone().into()),
            12382344057664863052u64 => Ok(self.blue_score_prev.clone().into()),
            9684982604781518527u64 => Ok(self.red_score_prev.clone().into()),
            12165785943437780003u64 => Ok(self.round_complete.clone().into()),
            2316304829487618708u64 => Ok(self.player_1.clone().into()),
            10394500961929970236u64 => Ok(self.player_1_damage.clone().into()),
            4434975577362185857u64 => Ok(self.player_1_healing.clone().into()),
            9525810424766332458u64 => Ok(self.player_1_lifetime.clone().into()),
            9144050188623277698u64 => Ok(self.player_1_kills.clone().into()),
            2316308128022503341u64 => Ok(self.player_2.clone().into()),
            10233858120128677491u64 => Ok(self.player_2_damage.clone().into()),
            14185483197478656496u64 => Ok(self.player_2_healing.clone().into()),
            4136489886055437333u64 => Ok(self.player_2_lifetime.clone().into()),
            4674312054664562187u64 => Ok(self.player_2_kills.clone().into()),
            2316307028510875130u64 => Ok(self.player_3.clone().into()),
            39363359054721358u64 => Ok(self.player_3_damage.clone().into()),
            3236287490998805827u64 => Ok(self.player_3_healing.clone().into()),
            14621494996524927732u64 => Ok(self.player_3_lifetime.clone().into()),
            3863298646261365396u64 => Ok(self.player_3_kills.clone().into()),
            2316301530952734075u64 => Ok(self.player_4.clone().into()),
            10597470269304895533u64 => Ok(self.player_4_damage.clone().into()),
            16447535156948377850u64 => Ok(self.player_4_healing.clone().into()),
            7059298629593792111u64 => Ok(self.player_4_lifetime.clone().into()),
            3991910781166784861u64 => Ok(self.player_4_kills.clone().into()),
            2316300431441105864u64 => Ok(self.player_5.clone().into()),
            5792678822030670832u64 => Ok(self.player_5_damage.clone().into()),
            10549804756926849709u64 => Ok(self.player_5_healing.clone().into()),
            5937207214365195550u64 => Ok(self.player_5_lifetime.clone().into()),
            12801315132847255054u64 => Ok(self.player_5_kills.clone().into()),
            2316303729975990497u64 => Ok(self.player_6.clone().into()),
            11907089275040254807u64 => Ok(self.player_6_damage.clone().into()),
            12917045825324352380u64 => Ok(self.player_6_healing.clone().into()),
            2459117320880115529u64 => Ok(self.player_6_lifetime.clone().into()),
            9062282318485088775u64 => Ok(self.player_6_kills.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArenaWinPanel",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PveWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
}
impl PveWinPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PveWinPanelEvent {
            panel_style: read_value::<u8>(
                stream,
                definition.get_entry(3076948889484157827u64),
                "panel_style",
            )?,
            winning_team: read_value::<u8>(
                stream,
                definition.get_entry(12760025138952247085u64),
                "winning_team",
            )?,
            win_reason: read_value::<u8>(
                stream,
                definition.get_entry(3803421146477351589u64),
                "win_reason",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            3076948889484157827u64 => Ok(self.panel_style.clone().into()),
            12760025138952247085u64 => Ok(self.winning_team.clone().into()),
            3803421146477351589u64 => Ok(self.win_reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PveWinPanel",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AirDashEvent {
    pub player: u8,
}
impl AirDashEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(AirDashEvent {
            player: read_value::<u8>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "AirDash",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LandedEvent {
    pub player: u8,
}
impl LandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LandedEvent {
            player: read_value::<u8>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "Landed",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDamageDodgedEvent {
    pub damage: u16,
}
impl PlayerDamageDodgedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDamageDodgedEvent {
            damage: read_value::<u16>(
                stream,
                definition.get_entry(9179190079975030720u64),
                "damage",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9179190079975030720u64 => Ok(self.damage.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDamageDodged",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerStunnedEvent {
    pub stunner: u16,
    pub victim: u16,
    pub victim_capping: bool,
    pub big_stun: bool,
}
impl PlayerStunnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerStunnedEvent {
            stunner: read_value::<u16>(
                stream,
                definition.get_entry(14931815241283043822u64),
                "stunner",
            )?,
            victim: read_value::<u16>(
                stream,
                definition.get_entry(3120917251440744469u64),
                "victim",
            )?,
            victim_capping: read_value::<bool>(
                stream,
                definition.get_entry(6103089581884104798u64),
                "victim_capping",
            )?,
            big_stun: read_value::<bool>(
                stream,
                definition.get_entry(2754291295915618874u64),
                "big_stun",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            14931815241283043822u64 => Ok(self.stunner.clone().into()),
            3120917251440744469u64 => Ok(self.victim.clone().into()),
            6103089581884104798u64 => Ok(self.victim_capping.clone().into()),
            2754291295915618874u64 => Ok(self.big_stun.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerStunned",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoutGrandSlamEvent {
    pub scout_id: u16,
    pub target_id: u16,
}
impl ScoutGrandSlamEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ScoutGrandSlamEvent {
            scout_id: read_value::<u16>(
                stream,
                definition.get_entry(7064780806045746163u64),
                "scout_id",
            )?,
            target_id: read_value::<u16>(
                stream,
                definition.get_entry(13239627154349772880u64),
                "target_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            7064780806045746163u64 => Ok(self.scout_id.clone().into()),
            13239627154349772880u64 => Ok(self.target_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ScoutGrandSlam",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoutSlamdollLandedEvent {
    pub target_index: u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ScoutSlamdollLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ScoutSlamdollLandedEvent {
            target_index: read_value::<u16>(
                stream,
                definition.get_entry(654399427416389339u64),
                "target_index",
            )?,
            x: read_value::<f32>(stream, definition.get_entry(12638214688346347271u64), "x")?,
            y: read_value::<f32>(stream, definition.get_entry(12638213588834719060u64), "y")?,
            z: read_value::<f32>(stream, definition.get_entry(12638216887369603693u64), "z")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            654399427416389339u64 => Ok(self.target_index.clone().into()),
            12638214688346347271u64 => Ok(self.x.clone().into()),
            12638213588834719060u64 => Ok(self.y.clone().into()),
            12638216887369603693u64 => Ok(self.z.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ScoutSlamdollLanded",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArrowImpactEvent {
    pub attached_entity: u16,
    pub shooter: u16,
    pub bone_index_attached: u16,
    pub bone_position_x: f32,
    pub bone_position_y: f32,
    pub bone_position_z: f32,
    pub bone_angles_x: f32,
    pub bone_angles_y: f32,
    pub bone_angles_z: f32,
    pub projectile_type: u16,
    pub is_crit: bool,
}
impl ArrowImpactEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ArrowImpactEvent {
            attached_entity: read_value::<u16>(
                stream,
                definition.get_entry(961881214683019338u64),
                "attached_entity",
            )?,
            shooter: read_value::<u16>(
                stream,
                definition.get_entry(13826298252589476871u64),
                "shooter",
            )?,
            bone_index_attached: read_value::<u16>(
                stream,
                definition.get_entry(6360700909751330309u64),
                "bone_index_attached",
            )?,
            bone_position_x: read_value::<f32>(
                stream,
                definition.get_entry(10378941398473653372u64),
                "bone_position_x",
            )?,
            bone_position_y: read_value::<f32>(
                stream,
                definition.get_entry(10378942497985281583u64),
                "bone_position_y",
            )?,
            bone_position_z: read_value::<f32>(
                stream,
                definition.get_entry(10378943597496909794u64),
                "bone_position_z",
            )?,
            bone_angles_x: read_value::<f32>(
                stream,
                definition.get_entry(5043211860521425589u64),
                "bone_angles_x",
            )?,
            bone_angles_y: read_value::<f32>(
                stream,
                definition.get_entry(5043210761009797378u64),
                "bone_angles_y",
            )?,
            bone_angles_z: read_value::<f32>(
                stream,
                definition.get_entry(5043209661498169167u64),
                "bone_angles_z",
            )?,
            projectile_type: read_value::<u16>(
                stream,
                definition.get_entry(9968460680690579194u64),
                "projectile_type",
            )?,
            is_crit: read_value::<bool>(
                stream,
                definition.get_entry(15363801587511353021u64),
                "is_crit",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            961881214683019338u64 => Ok(self.attached_entity.clone().into()),
            13826298252589476871u64 => Ok(self.shooter.clone().into()),
            6360700909751330309u64 => Ok(self.bone_index_attached.clone().into()),
            10378941398473653372u64 => Ok(self.bone_position_x.clone().into()),
            10378942497985281583u64 => Ok(self.bone_position_y.clone().into()),
            10378943597496909794u64 => Ok(self.bone_position_z.clone().into()),
            5043211860521425589u64 => Ok(self.bone_angles_x.clone().into()),
            5043210761009797378u64 => Ok(self.bone_angles_y.clone().into()),
            5043209661498169167u64 => Ok(self.bone_angles_z.clone().into()),
            9968460680690579194u64 => Ok(self.projectile_type.clone().into()),
            15363801587511353021u64 => Ok(self.is_crit.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArrowImpact",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerJaratedEvent {
    pub thrower_ent_index: u8,
    pub victim_ent_index: u8,
}
impl PlayerJaratedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerJaratedEvent {
            thrower_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(7733252424873930810u64),
                "thrower_ent_index",
            )?,
            victim_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(7608903543976233025u64),
                "victim_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            7733252424873930810u64 => Ok(self.thrower_ent_index.clone().into()),
            7608903543976233025u64 => Ok(self.victim_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerJarated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerJaratedFadeEvent {
    pub thrower_ent_index: u8,
    pub victim_ent_index: u8,
}
impl PlayerJaratedFadeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerJaratedFadeEvent {
            thrower_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(7733252424873930810u64),
                "thrower_ent_index",
            )?,
            victim_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(7608903543976233025u64),
                "victim_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            7733252424873930810u64 => Ok(self.thrower_ent_index.clone().into()),
            7608903543976233025u64 => Ok(self.victim_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerJaratedFade",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerShieldBlockedEvent {
    pub attacker_ent_index: u8,
    pub blocker_ent_index: u8,
}
impl PlayerShieldBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerShieldBlockedEvent {
            attacker_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(1824579233645723292u64),
                "attacker_ent_index",
            )?,
            blocker_ent_index: read_value::<u8>(
                stream,
                definition.get_entry(1554237712520490433u64),
                "blocker_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            1824579233645723292u64 => Ok(self.attacker_ent_index.clone().into()),
            1554237712520490433u64 => Ok(self.blocker_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerShieldBlocked",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerPinnedEvent {
    pub pinned: u8,
}
impl PlayerPinnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerPinnedEvent {
            pinned: read_value::<u8>(
                stream,
                definition.get_entry(6882141757131022863u64),
                "pinned",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            6882141757131022863u64 => Ok(self.pinned.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerPinned",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealedByMedicEvent {
    pub medic: u8,
}
impl PlayerHealedByMedicEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHealedByMedicEvent {
            medic: read_value::<u8>(
                stream,
                definition.get_entry(12912869923554243305u64),
                "medic",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            12912869923554243305u64 => Ok(self.medic.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHealedByMedic",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerSappedObjectEvent {
    pub user_id: u16,
    pub owner_id: u16,
    pub object: u8,
    pub sapper_id: u16,
}
impl PlayerSappedObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerSappedObjectEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            owner_id: read_value::<u16>(
                stream,
                definition.get_entry(3274630577613078265u64),
                "owner_id",
            )?,
            object: read_value::<u8>(
                stream,
                definition.get_entry(10231808476453998586u64),
                "object",
            )?,
            sapper_id: read_value::<u16>(
                stream,
                definition.get_entry(14334448895032880407u64),
                "sapper_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            3274630577613078265u64 => Ok(self.owner_id.clone().into()),
            10231808476453998586u64 => Ok(self.object.clone().into()),
            14334448895032880407u64 => Ok(self.sapper_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerSappedObject",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ItemFoundEvent {
    pub player: u8,
    pub quality: u8,
    pub method: u8,
    pub item_def: u32,
    pub is_strange: u8,
    pub is_unusual: u8,
    pub wear: f32,
}
impl ItemFoundEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ItemFoundEvent {
            player: read_value::<u8>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
            quality: read_value::<u8>(
                stream,
                definition.get_entry(8827161447210483302u64),
                "quality",
            )?,
            method: read_value::<u8>(
                stream,
                definition.get_entry(2525399976365011888u64),
                "method",
            )?,
            item_def: read_value::<u32>(
                stream,
                definition.get_entry(13929934279997928333u64),
                "item_def",
            )?,
            is_strange: read_value::<u8>(
                stream,
                definition.get_entry(4841352240690495167u64),
                "is_strange",
            )?,
            is_unusual: read_value::<u8>(
                stream,
                definition.get_entry(17913208470555068296u64),
                "is_unusual",
            )?,
            wear: read_value::<f32>(stream, definition.get_entry(4427899308794289118u64), "wear")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            8827161447210483302u64 => Ok(self.quality.clone().into()),
            2525399976365011888u64 => Ok(self.method.clone().into()),
            13929934279997928333u64 => Ok(self.item_def.clone().into()),
            4841352240690495167u64 => Ok(self.is_strange.clone().into()),
            17913208470555068296u64 => Ok(self.is_unusual.clone().into()),
            4427899308794289118u64 => Ok(self.wear.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ItemFound",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowAnnotationEvent {
    pub world_pos_x: f32,
    pub world_pos_y: f32,
    pub world_pos_z: f32,
    pub world_normal_x: f32,
    pub world_normal_y: f32,
    pub world_normal_z: f32,
    pub id: u32,
    pub text: MaybeUtf8String,
    pub lifetime: f32,
    pub visibility_bit_field: u32,
    pub follow_ent_index: u32,
    pub show_distance: bool,
    pub play_sound: MaybeUtf8String,
    pub show_effect: bool,
}
impl ShowAnnotationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ShowAnnotationEvent {
            world_pos_x: read_value::<f32>(
                stream,
                definition.get_entry(17136446437287500353u64),
                "world_pos_x",
            )?,
            world_pos_y: read_value::<f32>(
                stream,
                definition.get_entry(17136445337775872142u64),
                "world_pos_y",
            )?,
            world_pos_z: read_value::<f32>(
                stream,
                definition.get_entry(17136444238264243931u64),
                "world_pos_z",
            )?,
            world_normal_x: read_value::<f32>(
                stream,
                definition.get_entry(6094703810714717260u64),
                "world_normal_x",
            )?,
            world_normal_y: read_value::<f32>(
                stream,
                definition.get_entry(6094704910226345471u64),
                "world_normal_y",
            )?,
            world_normal_z: read_value::<f32>(
                stream,
                definition.get_entry(6094706009737973682u64),
                "world_normal_z",
            )?,
            id: read_value::<u32>(stream, definition.get_entry(628021283683842752u64), "id")?,
            text: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(18015793717152399486u64),
                "text",
            )?,
            lifetime: read_value::<f32>(
                stream,
                definition.get_entry(17408443252967694094u64),
                "lifetime",
            )?,
            visibility_bit_field: read_value::<u32>(
                stream,
                definition.get_entry(3514840313863479388u64),
                "visibility_bit_field",
            )?,
            follow_ent_index: read_value::<u32>(
                stream,
                definition.get_entry(5396668150069485958u64),
                "follow_ent_index",
            )?,
            show_distance: read_value::<bool>(
                stream,
                definition.get_entry(10815115409701055510u64),
                "show_distance",
            )?,
            play_sound: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry(9580177307637293387u64),
                "play_sound",
            )?,
            show_effect: read_value::<bool>(
                stream,
                definition.get_entry(16061793018411867440u64),
                "show_effect",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17136446437287500353u64 => Ok(self.world_pos_x.clone().into()),
            17136445337775872142u64 => Ok(self.world_pos_y.clone().into()),
            17136444238264243931u64 => Ok(self.world_pos_z.clone().into()),
            6094703810714717260u64 => Ok(self.world_normal_x.clone().into()),
            6094704910226345471u64 => Ok(self.world_normal_y.clone().into()),
            6094706009737973682u64 => Ok(self.world_normal_z.clone().into()),
            628021283683842752u64 => Ok(self.id.clone().into()),
            18015793717152399486u64 => Ok(self.text.clone().into()),
            17408443252967694094u64 => Ok(self.lifetime.clone().into()),
            3514840313863479388u64 => Ok(self.visibility_bit_field.clone().into()),
            5396668150069485958u64 => Ok(self.follow_ent_index.clone().into()),
            10815115409701055510u64 => Ok(self.show_distance.clone().into()),
            9580177307637293387u64 => Ok(self.play_sound.clone().into()),
            16061793018411867440u64 => Ok(self.show_effect.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ShowAnnotation",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HideAnnotationEvent {
    pub id: u32,
}
impl HideAnnotationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HideAnnotationEvent {
            id: read_value::<u32>(stream, definition.get_entry(628021283683842752u64), "id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            628021283683842752u64 => Ok(self.id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HideAnnotation",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PostInventoryApplicationEvent {
    pub user_id: u16,
}
impl PostInventoryApplicationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PostInventoryApplicationEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PostInventoryApplication",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUnlockUpdatedEvent {
    pub index: u16,
    pub time: f32,
}
impl ControlPointUnlockUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUnlockUpdatedEvent {
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
            time: read_value::<f32>(stream, definition.get_entry(2185518981507421060u64), "time")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9497966886403524235u64 => Ok(self.index.clone().into()),
            2185518981507421060u64 => Ok(self.time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUnlockUpdated",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DeployBuffBannerEvent {
    pub buff_type: u8,
    pub buff_owner: u16,
}
impl DeployBuffBannerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DeployBuffBannerEvent {
            buff_type: read_value::<u8>(
                stream,
                definition.get_entry(15706957908546287009u64),
                "buff_type",
            )?,
            buff_owner: read_value::<u16>(
                stream,
                definition.get_entry(16619542197404164576u64),
                "buff_owner",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            15706957908546287009u64 => Ok(self.buff_type.clone().into()),
            16619542197404164576u64 => Ok(self.buff_owner.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DeployBuffBanner",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBuffEvent {
    pub user_id: u16,
    pub buff_owner: u16,
    pub buff_type: u8,
}
impl PlayerBuffEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerBuffEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            buff_owner: read_value::<u16>(
                stream,
                definition.get_entry(16619542197404164576u64),
                "buff_owner",
            )?,
            buff_type: read_value::<u8>(
                stream,
                definition.get_entry(15706957908546287009u64),
                "buff_type",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            16619542197404164576u64 => Ok(self.buff_owner.clone().into()),
            15706957908546287009u64 => Ok(self.buff_type.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerBuff",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MedicDeathEvent {
    pub user_id: u16,
    pub attacker: u16,
    pub healing: u16,
    pub charged: bool,
}
impl MedicDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MedicDeathEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            attacker: read_value::<u16>(
                stream,
                definition.get_entry(7198542740550218478u64),
                "attacker",
            )?,
            healing: read_value::<u16>(
                stream,
                definition.get_entry(2721180038881757981u64),
                "healing",
            )?,
            charged: read_value::<bool>(
                stream,
                definition.get_entry(17260212114554334191u64),
                "charged",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            7198542740550218478u64 => Ok(self.attacker.clone().into()),
            2721180038881757981u64 => Ok(self.healing.clone().into()),
            17260212114554334191u64 => Ok(self.charged.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MedicDeath",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OvertimeNagEvent {}
impl OvertimeNagEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(OvertimeNagEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "OvertimeNag",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamsChangedEvent {}
impl TeamsChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamsChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamsChanged",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenPumpkinGrabEvent {
    pub user_id: u16,
}
impl HalloweenPumpkinGrabEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HalloweenPumpkinGrabEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HalloweenPumpkinGrab",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketJumpEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl RocketJumpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RocketJumpEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            play_sound: read_value::<bool>(
                stream,
                definition.get_entry(2035986273219443074u64),
                "play_sound",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            2035986273219443074u64 => Ok(self.play_sound.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RocketJump",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketJumpLandedEvent {
    pub user_id: u16,
}
impl RocketJumpLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RocketJumpLandedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RocketJumpLanded",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StickyJumpEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl StickyJumpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StickyJumpEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            play_sound: read_value::<bool>(
                stream,
                definition.get_entry(2035986273219443074u64),
                "play_sound",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            2035986273219443074u64 => Ok(self.play_sound.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "StickyJump",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StickyJumpLandedEvent {
    pub user_id: u16,
}
impl StickyJumpLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StickyJumpLandedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "StickyJumpLanded",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketPackLaunchEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl RocketPackLaunchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RocketPackLaunchEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            play_sound: read_value::<bool>(
                stream,
                definition.get_entry(2035986273219443074u64),
                "play_sound",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            2035986273219443074u64 => Ok(self.play_sound.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RocketPackLaunch",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketPackLandedEvent {
    pub user_id: u16,
}
impl RocketPackLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RocketPackLandedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RocketPackLanded",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MedicDefendedEvent {
    pub user_id: u16,
    pub medic: u16,
}
impl MedicDefendedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MedicDefendedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            medic: read_value::<u16>(
                stream,
                definition.get_entry(12912869923554243305u64),
                "medic",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            12912869923554243305u64 => Ok(self.medic.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MedicDefended",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerHealedEvent {
    pub amount: u16,
}
impl LocalPlayerHealedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerHealedEvent {
            amount: read_value::<u16>(
                stream,
                definition.get_entry(9301057475299076457u64),
                "amount",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9301057475299076457u64 => Ok(self.amount.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerHealed",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDestroyedPipeBombEvent {
    pub user_id: u16,
}
impl PlayerDestroyedPipeBombEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDestroyedPipeBombEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDestroyedPipeBomb",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDeflectedEvent {
    pub user_id: u16,
    pub owner_id: u16,
    pub weapon_id: u16,
    pub object_ent_index: u16,
}
impl ObjectDeflectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ObjectDeflectedEvent {
            user_id: read_value::<u16>(
                stream,
                definition.get_entry(17826443226371159423u64),
                "user_id",
            )?,
            owner_id: read_value::<u16>(
                stream,
                definition.get_entry(3274630577613078265u64),
                "owner_id",
            )?,
            weapon_id: read_value::<u16>(
                stream,
                definition.get_entry(5542695206485500884u64),
                "weapon_id",
            )?,
            object_ent_index: read_value::<u16>(
                stream,
                definition.get_entry(8474579522830253112u64),
                "object_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            17826443226371159423u64 => Ok(self.user_id.clone().into()),
            3274630577613078265u64 => Ok(self.owner_id.clone().into()),
            5542695206485500884u64 => Ok(self.weapon_id.clone().into()),
            8474579522830253112u64 => Ok(self.object_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ObjectDeflected",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerMvpEvent {
    pub player: u16,
}
impl PlayerMvpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerMvpEvent {
            player: read_value::<u16>(
                stream,
                definition.get_entry(5008278420455340480u64),
                "player",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            5008278420455340480u64 => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerMvp",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RaidSpawnMobEvent {}
impl RaidSpawnMobEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RaidSpawnMobEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RaidSpawnMob",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RaidSpawnSquadEvent {}
impl RaidSpawnSquadEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RaidSpawnSquadEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RaidSpawnSquad",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NavBlockedEvent {
    pub area: u32,
    pub blocked: bool,
}
impl NavBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(NavBlockedEvent {
            area: read_value::<u32>(stream, definition.get_entry(9894459526856489156u64), "area")?,
            blocked: read_value::<bool>(
                stream,
                definition.get_entry(9150172433819428659u64),
                "blocked",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9894459526856489156u64 => Ok(self.area.clone().into()),
            9150172433819428659u64 => Ok(self.blocked.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "NavBlocked",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(entry)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PathTrackPassedEvent {
    pub index: u16,
}
impl PathTrackPassedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PathTrackPassedEvent {
            index: read_value::<u16>(
                stream,
                definition.get_entry(9497966886403524235u64),
                "index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &GameEventEntry) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field.hash {
            9497966886403524235u64 => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PathTrackPassed",
                field: "todo".into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinÛž÷Ó†òµë(š+my×2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B7566÷&TWfVçB°Ð¢V"66÷&W#¢SbÀÐ¢V"76—7FW#¢SbÀÐ¢V"ö–çG3¢S‚ÀÐ§ÐÐ¦–×Â7566÷&TWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…7566÷&TWfVçB°Ð¢66÷&W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒC“cSƒScScƒ“sƒ“7ScB’ÀÐ¢'66÷&W""ÀÐ¢“òÀÐ¢76—7FW#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs“sƒ3ƒsSCC“#c“swScB’ÀÐ¢&76—7FW""ÀÐ¢“òÀÐ¢ö–çG3¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒS“#SCƒ#CƒSƒScgScB’ÀÐ¢'ö–çG2"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢C“cSƒScScƒ“sƒ“7ScBÓâö²‡6VÆbç66÷&W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s“sƒ3ƒsSCC“#c“swScBÓâö²‡6VÆbæ76—7FW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢S“#SCƒ#CƒSƒScgScBÓâö²‡6VÆbçö–çG2æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%7566÷&R"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B74g&VTWfVçB°Ð¢V"÷væW#¢SbÀÐ¢V"GF6¶W#¢SbÀÐ§ÐÐ¦–×Â74g&VTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…74g&VTWfVçB°Ð¢÷væW#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ##“#s“#SsscƒCcc‡ScB’ÀÐ¢&÷væW""ÀÐ¢“òÀÐ¢GF6¶W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs“ƒSC#sCSS#ƒCs‡ScB’ÀÐ¢&GF6¶W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢##“#s“#SsscƒCcc‡ScBÓâö²‡6VÆbæ÷væW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s“ƒSC#sCSS#ƒCs‡ScBÓâö²‡6VÆbæGF6¶W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%74g&VR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B75746Vv‡DWfVçB°Ð¢V"76W#¢SbÀÐ¢V"6F6†W#¢SbÀÐ¢V"F—7C¢c3"ÀÐ¢V"GW&F–öã¢c3"ÀÐ§ÐÐ¦–×Â75746Vv‡DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…75746Vv‡DWfVçB°Ð¢76W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒscsc““3cC#3ƒ7ScB’ÀÐ¢'76W""ÀÐ¢“òÀÐ¢6F6†W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒS#s#3s3##3sSScB’ÀÐ¢&6F6†W""ÀÐ¢“òÀÐ¢F—7C¢&VE÷fÇVS££Æc3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒCSsC“ccSC“SC“37ScB’ÀÐ¢&F—7B"ÀÐ¢“òÀÐ¢GW&F–öã¢&VE÷fÇVS££Æc3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ#cƒ“cSSSSScB’ÀÐ¢&GW&F–öâ"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢ƒscsc““3cC#3ƒ7ScBÓâö²‡6VÆbç76W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢S#s#3s3##3sSScBÓâö²‡6VÆbæ6F6†W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢CSsC“ccSC“SC“37ScBÓâö²‡6VÆbæF—7Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢#cƒ“cSSSSScBÓâö²‡6VÆbæGW&F–öâæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%75746Vv‡B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B74&ÆÅ7FöÆVäWfVçB°Ð¢V"f–7F–Ó¢SbÀÐ¢V"GF6¶W#¢SbÀÐ§ÐÐ¦–×Â74&ÆÅ7FöÆVäWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…74&ÆÅ7FöÆVäWfVçB°Ð¢f–7F–Ó¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ3#“s#SCCsCCCc—ScB’ÀÐ¢'f–7F–Ò"ÀÐ¢“òÀÐ¢GF6¶W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs“ƒSC#sCSS#ƒCs‡ScB’ÀÐ¢&GF6¶W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢3#“s#SCCsCCCc—ScBÓâö²‡6VÆbçf–7F–Òæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s“ƒSC#sCSS#ƒCs‡ScBÓâö²‡6VÆbæGF6¶W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%74&ÆÅ7FöÆVâ"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B74&ÆÄ&Æö6¶VDWfVçB°Ð¢V"÷væW#¢SbÀÐ¢V"&Æö6¶W#¢SbÀÐ§ÐÐ¦–×Â74&ÆÄ&Æö6¶VDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…74&ÆÄ&Æö6¶VDWfVçB°Ð¢÷væW#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ##“#s“#SsscƒCcc‡ScB’ÀÐ¢&÷væW""ÀÐ¢“òÀÐ¢&Æö6¶W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ“S“cc#3sS#C“3ScB’ÀÐ¢&&Æö6¶W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢##“#s“#SsscƒCcc‡ScBÓâö²‡6VÆbæ÷væW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢“S“cc#3sS#C“3ScBÓâö²‡6VÆbæ&Æö6¶W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%74&ÆÄ&Æö6¶VB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BFÖvU&WfVçFVDWfVçB°Ð¢V"&WfVçF÷#¢SbÀÐ¢V"f–7F–Ó¢SbÀÐ¢V"Ö÷VçC¢SbÀÐ¢V"6öæF—F–öã¢SbÀÐ§ÐÐ¦–×ÂFÖvU&WfVçFVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„FÖvU&WfVçFVDWfVçB°Ð¢&WfVçF÷#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒCSs3csCsSSƒsCc#‡ScB’ÀÐ¢'&WfVçF÷""ÀÐ¢“òÀÐ¢f–7F–Ó¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ3#“s#SCCsCCCc—ScB’ÀÐ¢'f–7F–Ò"ÀÐ¢“òÀÐ¢Ö÷VçC¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ“3SsCsS#““scCSwScB’ÀÐ¢&Ö÷VçB"ÀÐ¢“òÀÐ¢6öæF—F–öã¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒS#“c33ƒƒS“c##sScB’ÀÐ¢&6öæF—F–öâ"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢CSs3csCsSSƒsCc#‡ScBÓâö²‡6VÆbç&WfVçF÷"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢3#“s#SCCsCCCc—ScBÓâö²‡6VÆbçf–7F–Òæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢“3SsCsS#““scCSwScBÓâö²‡6VÆbæÖ÷VçBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢S#“c33ƒƒS“c##sScBÓâö²‡6VÆbæ6öæF—F–öâæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$FÖvU&WfVçFVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B†ÆÆ÷vVVä&÷74¶–ÆÆVDWfVçB°Ð¢V"&÷73¢SbÀÐ¢V"¶–ÆÆW#¢SbÀÐ§ÐÐ¦–×Â†ÆÆ÷vVVä&÷74¶–ÆÆVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„†ÆÆ÷vVVä&÷74¶–ÆÆVDWfVçB°Ð¢&÷73¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒCsƒ33“3#ƒ“sƒ“'ScB’ÀÐ¢&&÷72"ÀÐ¢“òÀÐ¢¶–ÆÆW#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs“#s3s#ƒSCC3S‡ScB’ÀÐ¢&¶–ÆÆW""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Csƒ33“3#ƒ“sƒ“'ScBÓâö²‡6VÆbæ&÷72æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s“#s3s#ƒSCC3S‡ScBÓâö²‡6VÆbæ¶–ÆÆW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$†ÆÆ÷vVVä&÷74¶–ÆÆVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BW66VDÆö÷D—6ÆæDWfVçB°Ð¢V"Æ–W#¢SbÀÐ§ÐÐ¦–×ÂW66VDÆö÷D—6ÆæDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„W66VDÆö÷D—6ÆæDWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$W66VDÆö÷D—6ÆæB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BFvvVEÆ–W$4—DWfVçB°Ð¢V"Æ–W#¢SbÀÐ§ÐÐ¦–×ÂFvvVEÆ–W$4—DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…FvvVEÆ–W$4—DWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%FvvVEÆ–W$4—B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÖW&6×W57GVææVDWfVçB°Ð¢V"Æ–W#¢SbÀÐ§ÐÐ¦–×ÂÖW&6×W57GVææVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„ÖW&6×W57GVææVDWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$ÖW&6×W57GVææVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÖW&6×W5&÷f÷VæDWfVçB°Ð¢V"Æ–W#¢SbÀÐ§ÐÐ¦–×ÂÖW&6×W5&÷f÷VæDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„ÖW&6×W5&÷f÷VæDWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$ÖW&6×W5&÷f÷VæB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVDWfVçB°Ð¢V"Æ–W#¢SbÀÐ§ÐÐ¦–×Â†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVDWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B6¶VÆWFöä¶–ÆÆVEVW7DWfVçB°Ð¢V"Æ–W#¢SbÀÐ§ÐÐ¦–×Â6¶VÆWFöä¶–ÆÆVEVW7DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…6¶VÆWFöä¶–ÆÆVEVW7DWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%6¶VÆWFöä¶–ÆÆVEVW7B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B6¶VÆWFöä¶–æt¶–ÆÆVEVW7DWfVçB°Ð¢V"Æ–W#¢SbÀÐ§ÐÐ¦–×Â6¶VÆWFöä¶–æt¶–ÆÆVEVW7DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…6¶VÆWFöä¶–æt¶–ÆÆVEVW7DWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%6¶VÆWFöä¶–æt¶–ÆÆVEVW7B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BW66T†VÆÄWfVçB°Ð¢V"Æ–W#¢SbÀÐ§ÐÐ¦–×ÂW66T†VÆÄWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„W66T†VÆÄWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$W66T†VÆÂ"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B7&÷757V7G&Ä'&–FvTWfVçB°Ð¢V"Æ–W#¢SbÀÐ§ÐÐ¦–×Â7&÷757V7G&Ä'&–FvTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„7&÷757V7G&Ä'&–FvTWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$7&÷757V7G&Ä'&–FvR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÖ–æ”vÖUvöäWfVçB°Ð¢V"Æ–W#¢SbÀÐ¢V"vÖS¢SbÀÐ§ÐÐ¦–×ÂÖ–æ”vÖUvöäWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„Ö–æ”vÖUvöäWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢vÖS¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSC“C3#s#c#S“—ScB’ÀÐ¢&vÖR"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢SC“C3#s#c#S“—ScBÓâö²‡6VÆbævÖRæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$Ö–æ”vÖUvöâ"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&W7väv†÷7DWfVçB°Ð¢V"&Wf—fW#¢SbÀÐ¢V"v†÷7C¢SbÀÐ§ÐÐ¦–×Â&W7väv†÷7DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&W7väv†÷7DWfVçB°Ð¢&Wf—fW#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ#33S#SS##C3#gScB’ÀÐ¢'&Wf—fW""ÀÐ¢“òÀÐ¢v†÷7C¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒsScCƒSs33c“ssƒ‡ScB’ÀÐ¢&v†÷7B"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢#33S#SS##C3#gScBÓâö²‡6VÆbç&Wf—fW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢sScCƒSs33c“ssƒ‡ScBÓâö²‡6VÆbæv†÷7Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&W7väv†÷7B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B¶–ÆÄ–ä†VÆÄWfVçB°Ð¢V"¶–ÆÆW#¢SbÀÐ¢V"f–7F–Ó¢SbÀÐ§ÐÐ¦–×Â¶–ÆÄ–ä†VÆÄWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„¶–ÆÄ–ä†VÆÄWfVçB°Ð¢¶–ÆÆW#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs“#s3s#ƒSCC3S‡ScB’ÀÐ¢&¶–ÆÆW""ÀÐ¢“òÀÐ¢f–7F–Ó¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ3#“s#SCCsCCCc—ScB’ÀÐ¢'f–7F–Ò"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢s“#s3s#ƒSCC3S‡ScBÓâö²‡6VÆbæ¶–ÆÆW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢3#“s#SCCsCCCc—ScBÓâö²‡6VÆbçf–7F–Òæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$¶–ÆÄ–ä†VÆÂ"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B†ÆÆ÷vVVäGV6´6öÆÆV7FVDWfVçB°Ð¢V"6öÆÆV7F÷#¢SbÀÐ§ÐÐ¦–×Â†ÆÆ÷vVVäGV6´6öÆÆV7FVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„†ÆÆ÷vVVäGV6´6öÆÆV7FVDWfVçB°Ð¢6öÆÆV7F÷#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ3ƒ“S3cs““CC3cSgScB’ÀÐ¢&6öÆÆV7F÷""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢3ƒ“S3cs““CC3cSgScBÓâö²‡6VÆbæ6öÆÆV7F÷"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$†ÆÆ÷vVVäGV6´6öÆÆV7FVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B7V6–Å66÷&TWfVçB°Ð¢V"Æ–W#¢S‚ÀÐ§ÐÐ¦–×Â7V6–Å66÷&TWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…7V6–Å66÷&TWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%7V6–Å66÷&R"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BFVÔÆVFW$¶–ÆÆVDWfVçB°Ð¢V"¶–ÆÆW#¢S‚ÀÐ¢V"f–7F–Ó¢S‚ÀÐ§ÐÐ¦–×ÂFVÔÆVFW$¶–ÆÆVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…FVÔÆVFW$¶–ÆÆVDWfVçB°Ð¢¶–ÆÆW#¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs“#s3s#ƒSCC3S‡ScB’ÀÐ¢&¶–ÆÆW""ÀÐ¢“òÀÐ¢f–7F–Ó¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ3#“s#SCCsCCCc—ScB’ÀÐ¢'f–7F–Ò"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢s“#s3s#ƒSCC3S‡ScBÓâö²‡6VÆbæ¶–ÆÆW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢3#“s#SCCsCCCc—ScBÓâö²‡6VÆbçf–7F–Òæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%FVÔÆVFW$¶–ÆÆVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVDWfVçB°Ð¢V"–çFVæFVE÷F&vWC¢S‚ÀÐ¢V"6öÆÆV7F–æu÷Æ–W#¢S‚ÀÐ¢V"6÷VÅö6÷VçC¢S‚ÀÐ§ÐÐ¦–×Â†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVDWfVçB°Ð¢–çFVæFVE÷F&vWC¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒcƒ#C3sƒS3ccs“cCƒ'ScB’ÀÐ¢&–çFVæFVE÷F&vWB"ÀÐ¢“òÀÐ¢6öÆÆV7F–æu÷Æ–W#¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ#SsSc#3Sc“#ScB’ÀÐ¢&6öÆÆV7F–æu÷Æ–W""ÀÐ¢“òÀÐ¢6÷VÅö6÷VçC¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒsSCScCcS#s##ƒsc3GScB’ÀÐ¢'6÷VÅö6÷VçB"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢cƒ#C3sƒS3ccs“cCƒ'ScBÓâö²‡6VÆbæ–çFVæFVE÷F&vWBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢#SsSc#3Sc“#ScBÓâö²‡6VÆbæ6öÆÆV7F–æu÷Æ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢sSCScCcS#s##ƒsc3GScBÓâö²‡6VÆbç6÷VÅö6÷VçBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&V6Æ7VÆFUG'V6TWfVçB·ÐÐ¦–×Â&V6Æ7VÆFUG'V6TWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&V6Æ7VÆFUG'V6TWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&V6Æ7VÆFUG'V6R"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BFVE&–ævW$6†VDFVF„WfVçB°Ð¢V"7“¢S‚ÀÐ¢V"GF6¶W#¢S‚ÀÐ§ÐÐ¦–×ÂFVE&–ævW$6†VDFVF„WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„FVE&–ævW$6†VDFVF„WfVçB°Ð¢7“¢&VE÷fÇVS££ÇSƒâ‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒ“3“Ccs3ƒSƒ3s3ScsCS7ScB’Â'7’"“òÀÐ¢GF6¶W#¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs“ƒSC#sCSS#ƒCs‡ScB’ÀÐ¢&GF6¶W""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢“3“Ccs3ƒSƒ3s3ScsCS7ScBÓâö²‡6VÆbç7’æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s“ƒSC#sCSS#ƒCs‡ScBÓâö²‡6VÆbæGF6¶W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$FVE&–ævW$6†VDFVF‚"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B7&÷76&÷t†VÄWfVçB°Ð¢V"†VÆW#¢S‚ÀÐ¢V"F&vWC¢S‚ÀÐ¢V"Ö÷VçC¢SbÀÐ§ÐÐ¦–×Â7&÷76&÷t†VÄWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„7&÷76&÷t†VÄWfVçB°Ð¢†VÆW#¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ““SCCƒ#S3C“S#ScB’ÀÐ¢&†VÆW""ÀÐ¢“òÀÐ¢F&vWC¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒcS3“cS“SssssS'ScB’ÀÐ¢'F&vWB"ÀÐ¢“òÀÐ¢Ö÷VçC¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ“3SsCsS#““scCSwScB’ÀÐ¢&Ö÷VçB"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢““SCCƒ#S3C“S#ScBÓâö²‡6VÆbæ†VÆW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢cS3“cS“SssssS'ScBÓâö²‡6VÆbçF&vWBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢“3SsCsS#““scCSwScBÓâö²‡6VÆbæÖ÷VçBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$7&÷76&÷t†VÂ"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BFÖvTÖ—F–vFVDWfVçB°Ð¢V"Ö—F–vF÷#¢S‚ÀÐ¢V"FÖvVC¢S‚ÀÐ¢V"Ö÷VçC¢SbÀÐ¢V"—FVÕöFVf–æ—F–öåö–æFWƒ¢SbÀÐ§ÐÐ¦–×ÂFÖvTÖ—F–vFVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„FÖvTÖ—F–vFVDWfVçB°Ð¢Ö—F–vF÷#¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc3ƒ3“sƒ3#3s3c—ScB’ÀÐ¢&Ö—F–vF÷""ÀÐ¢“òÀÐ¢FÖvVC¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ#“3ƒCSsscƒ“3“S“Cc‡ScB’ÀÐ¢&FÖvVB"ÀÐ¢“òÀÐ¢Ö÷VçC¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ“3SsCsS#““scCSwScB’ÀÐ¢&Ö÷VçB"ÀÐ¢“òÀÐ¢—FVÕöFVf–æ—F–öåö–æFWƒ¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒC“#cS#3Ssc3“#ƒ7ScB’ÀÐ¢&—FVÕöFVf–æ—F–öåö–æFW‚"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢c3ƒ3“sƒ3#3s3c—ScBÓâö²‡6VÆbæÖ—F–vF÷"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢#“3ƒCSsscƒ“3“S“Cc‡ScBÓâö²‡6VÆbæFÖvVBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢“3SsCsS#““scCSwScBÓâö²‡6VÆbæÖ÷VçBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢C“#cS#3Ssc3“#ƒ7ScBÓâö²‡6VÆbæ—FVÕöFVf–æ—F–öåö–æFW‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$FÖvTÖ—F–vFVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B–ÆöEW6†VDWfVçB°Ð¢V"W6†W#¢S‚ÀÐ¢V"F—7Fæ6S¢SbÀÐ§ÐÐ¦–×Â–ÆöEW6†VDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…–ÆöEW6†VDWfVçB°Ð¢W6†W#¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc3cCc3#“CsƒCC3ƒƒ3ScB’ÀÐ¢'W6†W""ÀÐ¢“òÀÐ¢F—7Fæ6S¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒS“ScƒSSSc““SCƒƒ'ScB’ÀÐ¢&F—7Fæ6R"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢c3cCc3#“CsƒCC3ƒƒ3ScBÓâö²‡6VÆbçW6†W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢S“ScƒSSSc““SCƒƒ'ScBÓâö²‡6VÆbæF—7Fæ6Ræ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%–ÆöEW6†VB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÆ–W$&æFöæVDÖF6„WfVçB°Ð¢V"vÖUö÷fW#¢&ööÂÀÐ§ÐÐ¦–×ÂÆ–W$&æFöæVDÖF6„WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…Æ–W$&æFöæVDÖF6„WfVçB°Ð¢vÖUö÷fW#¢&VE÷fÇVS££Æ&ööÃâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒsCs3#3ss“3“cS3ScB’ÀÐ¢&vÖUö÷fW""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢sCs3#3ss“3“cS3ScBÓâö²‡6VÆbævÖUö÷fW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%Æ–W$&æFöæVDÖF6‚"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B6ÄG&vÆ–æTWfVçB°Ð¢V"Æ–W#¢S‚ÀÐ¢V"æVÃ¢S‚ÀÐ¢V"Æ–æS¢S‚ÀÐ¢V"ƒ¢c3"ÀÐ¢V"“¢c3"ÀÐ§ÐÐ¦–×Â6ÄG&vÆ–æTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„6ÄG&vÆ–æTWfVçB°Ð¢Æ–W#¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ#sƒC#CSS3CCƒScB’ÀÐ¢'Æ–W""ÀÐ¢“òÀÐ¢æVÃ¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒcƒ#“#S#Sc3“ccc3cƒWScB’ÀÐ¢'æVÂ"ÀÐ¢“òÀÐ¢Æ–æS¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ3sƒC#“3#Cƒs##cƒ3—ScB’ÀÐ¢&Æ–æR"ÀÐ¢“òÀÐ¢ƒ¢&VE÷fÇVS££Æc3#â‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒ#c3ƒ#Ccƒƒ3Cc3Cs#sScB’Â'‚"“òÀÐ¢“¢&VE÷fÇVS££Æc3#â‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒ#c3ƒ#3Sƒƒƒ3Cs“cScB’Â'’"“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ#sƒC#CSS3CCƒScBÓâö²‡6VÆbçÆ–W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢cƒ#“#S#Sc3“ccc3cƒWScBÓâö²‡6VÆbçæVÂæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢3sƒC#“3#Cƒs##cƒ3—ScBÓâö²‡6VÆbæÆ–æRæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢#c3ƒ#Ccƒƒ3Cc3Cs#sScBÓâö²‡6VÆbç‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢#c3ƒ#3Sƒƒƒ3Cs“cScBÓâö²‡6VÆbç’æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$6ÄG&vÆ–æR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&W7F'EF–ÖW%F–ÖTWfVçB°Ð¢V"F–ÖS¢S‚ÀÐ§ÐÐ¦–×Â&W7F'EF–ÖW%F–ÖTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&W7F'EF–ÖW%F–ÖTWfVçB°Ð¢F–ÖS¢&VE÷fÇVS££ÇSƒâ‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒ#ƒSSƒ“ƒSsC#cScB’Â'F–ÖR"“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢#ƒSSƒ“ƒSsC#cScBÓâö²‡6VÆbçF–ÖRæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&W7F'EF–ÖW%F–ÖR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7Bv–äÆ–Ö—D6†ævVDWfVçB·ÐÐ¦–×Âv–äÆ–Ö—D6†ævVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…v–äÆ–Ö—D6†ævVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%v–äÆ–Ö—D6†ævVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7Bv–åæVÅ6†÷u66÷&W4WfVçB·ÐÐ¦–×Âv–åæVÅ6†÷u66÷&W4WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…v–åæVÅ6†÷u66÷&W4WfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%v–åæVÅ6†÷u66÷&W2"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BF÷7G&V×5&WVW7Df–æ—6†VDWfVçB·ÐÐ¦–×ÂF÷7G&V×5&WVW7Df–æ—6†VDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…F÷7G&V×5&WVW7Df–æ—6†VDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%F÷7G&V×5&WVW7Df–æ—6†VB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B6ö×WF—F—fU7FFT6†ævVDWfVçB·ÐÐ¦–×Â6ö×WF—F—fU7FFT6†ævVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„6ö×WF—F—fU7FFT6†ævVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$6ö×WF—F—fU7FFT6†ævVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BvÆö&Åv$FFWFFVDWfVçB·ÐÐ¦–×ÂvÆö&Åv$FFWFFVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„vÆö&Åv$FFWFFVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$vÆö&Åv$FFWFFVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B7F÷vF6„6†ævVDWfVçB·ÐÐ¦–×Â7F÷vF6„6†ævVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…7F÷vF6„6†ævVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%7F÷vF6„6†ævVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BG57F÷WfVçB·ÐÐ¦–×ÂG57F÷WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„G57F÷WfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$G57F÷"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BG567&VVç6†÷DWfVçB°Ð¢V"FVÆ“¢c3"ÀÐ§ÐÐ¦–×ÂG567&VVç6†÷DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„G567&VVç6†÷DWfVçB°Ð¢FVÆ“¢&VE÷fÇVS££Æc3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒSƒ“#scSsscc3sssƒGScB’ÀÐ¢&FVÆ’"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Sƒ“#scSsscc3sssƒGScBÓâö²‡6VÆbæFVÆ’æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$G567&VVç6†÷B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B6†÷tÖF6…7VÖÖ'”WfVçB·ÐÐ¦–×Â6†÷tÖF6…7VÖÖ'”WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…6†÷tÖF6…7VÖÖ'”WfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%6†÷tÖF6…7VÖÖ'’"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BW‡W&–Væ6T6†ævVDWfVçB·ÐÐ¦–×ÂW‡W&–Væ6T6†ævVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„W‡W&–Væ6T6†ævVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$W‡W&–Væ6T6†ævVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&Vv–å‡ÆW'WfVçB·ÐÐ¦–×Â&Vv–å‡ÆW'WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„&Vv–å‡ÆW'WfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$&Vv–å‡ÆW'"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÖF6†Ö¶W%7FG5WFFVDWfVçB·ÐÐ¦–×ÂÖF6†Ö¶W%7FG5WFFVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„ÖF6†Ö¶W%7FG5WFFVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$ÖF6†Ö¶W%7FG5WFFVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&VÖF6…f÷FUW&–öD÷fW$WfVçB°Ð¢V"7V66W73¢&ööÂÀÐ§ÐÐ¦–×Â&VÖF6…f÷FUW&–öD÷fW$WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&VÖF6…f÷FUW&–öD÷fW$WfVçB°Ð¢7V66W73¢&VE÷fÇVS££Æ&ööÃâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒs#“C#cƒC3s#cGScB’ÀÐ¢'7V66W72"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢ƒs#“C#cƒC3s#cGScBÓâö²‡6VÆbç7V66W72æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&VÖF6…f÷FUW&–öD÷fW""ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&VÖF6„f–ÆVEFô7&VFTWfVçB·ÐÐ¦–×Â&VÖF6„f–ÆVEFô7&VFTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&VÖF6„f–ÆVEFô7&VFTWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&VÖF6„f–ÆVEFô7&VFR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÆ–W%&VÖF6„6†ævTWfVçB·ÐÐ¦–×ÂÆ–W%&VÖF6„6†ævTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…Æ–W%&VÖF6„6†ævTWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%Æ–W%&VÖF6„6†ævR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B–æuWFFVDWfVçB·ÐÐ¦–×Â–æuWFFVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…–æuWFFVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%–æuWFFVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÔÕ7FG5WFFVDWfVçB·ÐÐ¦–×ÂÔÕ7FG5WFFVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„ÔÕ7FG5WFFVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$ÔÕ7FG5WFFVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÆ–W$æW‡DÖf÷FT6†ævTWfVçB°Ð¢V"Öö–æFWƒ¢S‚ÀÐ¢V"f÷FS¢S‚ÀÐ§ÐÐ¦–×ÂÆ–W$æW‡DÖf÷FT6†ævTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…Æ–W$æW‡DÖf÷FT6†ævTWfVçB°Ð¢Öö–æFWƒ¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒCƒ“sƒ“ƒ#sƒ#3c“ScB’ÀÐ¢&Öö–æFW‚"ÀÐ¢“òÀÐ¢f÷FS¢&VE÷fÇVS££ÇSƒâ‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒ3SC3CccC“S3“CScSScB’Â'f÷FR"“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Cƒ“sƒ“ƒ#sƒ#3c“ScBÓâö²‡6VÆbæÖö–æFW‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢3SC3CccC“S3“CScSScBÓâö²‡6VÆbçf÷FRæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%Æ–W$æW‡DÖf÷FT6†ævR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7Bf÷FTÖ46†ævVDWfVçB·ÐÐ¦–×Âf÷FTÖ46†ævVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…f÷FTÖ46†ævVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%f÷FTÖ46†ævVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&÷FôFVd6†ævVDWfVçB°Ð¢V"¶–æC¢S‚ÀÐ¢V"FVf–æ—F–öåö–æFWƒ¢S3"ÀÐ¢V"7&VFVC¢&ööÂÀÐ¢V"FVÆWFVC¢&ööÂÀÐ¢V"W&6Uö†—7F÷'“¢&ööÂÀÐ§ÐÐ¦–×Â&÷FôFVd6†ævVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&÷FôFVd6†ævVDWfVçB°Ð¢¶–æC¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ#sS3C#c#s3“#WScB’ÀÐ¢&¶–æB"ÀÐ¢“òÀÐ¢FVf–æ—F–öåö–æFWƒ¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ3cCCƒsSƒƒsƒ3cGScB’ÀÐ¢&FVf–æ—F–öåö–æFW‚"ÀÐ¢“òÀÐ¢7&VFVC¢&VE÷fÇVS££Æ&ööÃâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc#SƒSƒSƒcSs““WScB’ÀÐ¢&7&VFVB"ÀÐ¢“òÀÐ¢FVÆWFVC¢&VE÷fÇVS££Æ&ööÃâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ##CCCCƒ3Scc“S#C#gScB’ÀÐ¢&FVÆWFVB"ÀÐ¢“òÀÐ¢W&6Uö†—7F÷'“¢&VE÷fÇVS££Æ&ööÃâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒsCSScssƒ3sƒ3###Sƒ'ScB’ÀÐ¢&W&6Uö†—7F÷'’"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢#sS3C#c#s3“#WScBÓâö²‡6VÆbæ¶–æBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢3cCCƒsSƒƒsƒ3cGScBÓâö²‡6VÆbæFVf–æ—F–öåö–æFW‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢c#SƒSƒSƒcSs““WScBÓâö²‡6VÆbæ7&VFVBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢##CCCCƒ3Scc“S#C#gScBÓâö²‡6VÆbæFVÆWFVBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢sCSScssƒ3sƒ3###Sƒ'ScBÓâö²‡6VÆbæW&6Uö†—7F÷'’æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&÷FôFVd6†ævVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÆ–W$FöÖ–æF–öäWfVçB°Ð¢V"FöÖ–æF÷#¢SbÀÐ¢V"FöÖ–æFVC¢SbÀÐ¢V"FöÖ–æF–öç3¢SbÀÐ§ÐÐ¦–×ÂÆ–W$FöÖ–æF–öäWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…Æ–W$FöÖ–æF–öäWfVçB°Ð¢FöÖ–æF÷#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs3ƒCccƒsc“3cSC“'ScB’ÀÐ¢&FöÖ–æF÷""ÀÐ¢“òÀÐ¢FöÖ–æFVC¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒsCCsCssCS3#ƒƒ“C‡ScB’ÀÐ¢&FöÖ–æFVB"ÀÐ¢“òÀÐ¢FöÖ–æF–öç3¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒCSS“ƒ##ScC3ƒ3SGScB’ÀÐ¢&FöÖ–æF–öç2"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢s3ƒCccƒsc“3cSC“'ScBÓâö²‡6VÆbæFöÖ–æF÷"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢sCCsCssCS3#ƒƒ“C‡ScBÓâö²‡6VÆbæFöÖ–æFVBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢CSS“ƒ##ScC3ƒ3SGScBÓâö²‡6VÆbæFöÖ–æF–öç2æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%Æ–W$FöÖ–æF–öâ"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÆ–W%&ö6¶WE6µW6†VDWfVçB°Ð¢V"W6†W#¢SbÀÐ¢V"W6†VC¢SbÀÐ§ÐÐ¦–×ÂÆ–W%&ö6¶WE6µW6†VDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…Æ–W%&ö6¶WE6µW6†VDWfVçB°Ð¢W6†W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc3cCc3#“CsƒCC3ƒƒ3ScB’ÀÐ¢'W6†W""ÀÐ¢“òÀÐ¢W6†VC¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc3cCcsSCƒc#cC3ƒsgScB’ÀÐ¢'W6†VB"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢c3cCc3#“CsƒCC3ƒƒ3ScBÓâö²‡6VÆbçW6†W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢c3cCcsSCƒc#cC3ƒsgScBÓâö²‡6VÆbçW6†VBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%Æ–W%&ö6¶WE6µW6†VB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BVW7E&WVW7DWfVçB°Ð¢V"&WVW7C¢S3"ÀÐ¢V"×6s¢Ö–&UWFc…7G&–ærÀÐ§ÐÐ¦–×ÂVW7E&WVW7DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…VW7E&WVW7DWfVçB°Ð¢&WVW7C¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒƒCS####c“cCS‡ScB’ÀÐ¢'&WVW7B"ÀÐ¢“òÀÐ¢×6s¢&VE÷fÇVS££ÄÖ–&UWFc…7G&–æsâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒScsSS#ccƒS“ƒCS‡ScB’ÀÐ¢&×6r"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢ƒƒCS####c“cCS‡ScBÓâö²‡6VÆbç&WVW7Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢ScsSS#ccƒS“ƒCS‡ScBÓâö²‡6VÆbæ×6ræ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%VW7E&WVW7B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BVW7E&W7öç6TWfVçB°Ð¢V"&WVW7C¢S3"ÀÐ¢V"7V66W73¢&ööÂÀÐ¢V"×6s¢Ö–&UWFc…7G&–ærÀÐ§ÐÐ¦–×ÂVW7E&W7öç6TWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…VW7E&W7öç6TWfVçB°Ð¢&WVW7C¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒƒCS####c“cCS‡ScB’ÀÐ¢'&WVW7B"ÀÐ¢“òÀÐ¢7V66W73¢&VE÷fÇVS££Æ&ööÃâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒs#“C#cƒC3s#cGScB’ÀÐ¢'7V66W72"ÀÐ¢“òÀÐ¢×6s¢&VE÷fÇVS££ÄÖ–&UWFc…7G&–æsâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒScsSS#ccƒS“ƒCS‡ScB’ÀÐ¢&×6r"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢ƒƒCS####c“cCS‡ScBÓâö²‡6VÆbç&WVW7Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢ƒs#“C#cƒC3s#cGScBÓâö²‡6VÆbç7V66W72æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢ScsSS#ccƒS“ƒCS‡ScBÓâö²‡6VÆbæ×6ræ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%VW7E&W7öç6R"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BVW7E&öw&W74WfVçB°Ð¢V"÷væW#¢SbÀÐ¢V"66÷&W#¢SbÀÐ¢V"¶–æC¢S‚ÀÐ¢V"6ö×ÆWFVC¢&ööÂÀÐ¢V"VW7EöFVf–æ—F–öåö–æFWƒ¢S3"ÀÐ§ÐÐ¦–×ÂVW7E&öw&W74WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…VW7E&öw&W74WfVçB°Ð¢÷væW#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ##“#s“#SsscƒCcc‡ScB’ÀÐ¢&÷væW""ÀÐ¢“òÀÐ¢66÷&W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒC“cSƒScScƒ“sƒ“7ScB’ÀÐ¢'66÷&W""ÀÐ¢“òÀÐ¢¶–æC¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ#sS3C#c#s3“#WScB’ÀÐ¢&¶–æB"ÀÐ¢“òÀÐ¢6ö×ÆWFVC¢&VE÷fÇVS££Æ&ööÃâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒ#“3#3CsC“s3c#gScB’ÀÐ¢&6ö×ÆWFVB"ÀÐ¢“òÀÐ¢VW7EöFVf–æ—F–öåö–æFWƒ¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc“3#3SSccSƒCcwScB’ÀÐ¢'VW7EöFVf–æ—F–öåö–æFW‚"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢##“#s“#SsscƒCcc‡ScBÓâö²‡6VÆbæ÷væW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢C“cSƒScScƒ“sƒ“7ScBÓâö²‡6VÆbç66÷&W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢#sS3C#c#s3“#WScBÓâö²‡6VÆbæ¶–æBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢ƒ#“3#3CsC“s3c#gScBÓâö²‡6VÆbæ6ö×ÆWFVBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢c“3#3SSccSƒCcwScBÓâö²‡6VÆbçVW7EöFVf–æ—F–öåö–æFW‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%VW7E&öw&W72"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&ö¦V7F–ÆU&VÖ÷fVDWfVçB°Ð¢V"GF6¶W#¢S‚ÀÐ¢V"vVöåöFVeö–æFWƒ¢S3"ÀÐ¢V"çVÕö†—C¢S‚ÀÐ¢V"çVÕöF—&V7Eö†—C¢S‚ÀÐ§ÐÐ¦–×Â&ö¦V7F–ÆU&VÖ÷fVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&ö¦V7F–ÆU&VÖ÷fVDWfVçB°Ð¢GF6¶W#¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs“ƒSC#sCSS#ƒCs‡ScB’ÀÐ¢&GF6¶W""ÀÐ¢“òÀÐ¢vVöåöFVeö–æFWƒ¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒC3#3cS“CƒcƒSƒ“SGScB’ÀÐ¢'vVöåöFVeö–æFW‚"ÀÐ¢“òÀÐ¢çVÕö†—C¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ#SS3Sƒcsƒ#ƒƒSsS7ScB’ÀÐ¢&çVÕö†—B"ÀÐ¢“òÀÐ¢çVÕöF—&V7Eö†—C¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒcCƒ““3cs#“ƒcWScB’ÀÐ¢&çVÕöF—&V7Eö†—B"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢s“ƒSC#sCSS#ƒCs‡ScBÓâö²‡6VÆbæGF6¶W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢C3#3cS“CƒcƒSƒ“SGScBÓâö²‡6VÆbçvVöåöFVeö–æFW‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢#SS3Sƒcsƒ#ƒƒSsS7ScBÓâö²‡6VÆbæçVÕö†—Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢ƒcCƒ““3cs#“ƒcWScBÓâö²‡6VÆbæçVÕöF—&V7Eö†—Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&ö¦V7F–ÆU&VÖ÷fVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BVW7DÖFF6†ævVDWfVçB·ÐÐ¦–×ÂVW7DÖFF6†ævVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…VW7DÖFF6†ævVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%VW7DÖFF6†ævVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7Bv4F÷W6VEÆ–W$–væ—FVDWfVçB°Ð¢V"–væ—FW#¢SbÀÐ¢V"F÷W6W#¢SbÀÐ¢V"f–7F–Ó¢SbÀÐ§ÐÐ¦–×Âv4F÷W6VEÆ–W$–væ—FVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„v4F÷W6VEÆ–W$–væ—FVDWfVçB°Ð¢–væ—FW#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ“C#cC3Cc“c3#““C7ScB’ÀÐ¢&–væ—FW""ÀÐ¢“òÀÐ¢F÷W6W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒƒƒc“c33ƒc“#““3SS—ScB’ÀÐ¢&F÷W6W""ÀÐ¢“òÀÐ¢f–7F–Ó¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ3#“s#SCCsCCCc—ScB’ÀÐ¢'f–7F–Ò"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢“C#cC3Cc“c3#““C7ScBÓâö²‡6VÆbæ–væ—FW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢ƒƒƒc“c33ƒc“#““3SS—ScBÓâö²‡6VÆbæF÷W6W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢3#“s#SCCsCCCc—ScBÓâö²‡6VÆbçf–7F–Òæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$v4F÷W6VEÆ–W$–væ—FVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BVW7EGW&ä–å7FFTWfVçB°Ð¢V"7FFS¢SbÀÐ§ÐÐ¦–×ÂVW7EGW&ä–å7FFTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…VW7EGW&ä–å7FFTWfVçB°Ð¢7FFS¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒssssccCƒ“cSC“SScB’ÀÐ¢'7FFR"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢ssssccCƒ“cSC“SScBÓâö²‡6VÆbç7FFRæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%VW7EGW&ä–å7FFR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B—FV×46¶æ÷vÆVFvVDWfVçB·ÐÐ¦–×Â—FV×46¶æ÷vÆVFvVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„—FV×46¶æ÷vÆVFvVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$—FV×46¶æ÷vÆVFvVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B6W$¶–ÆÆVDWfVçB°Ð¢V"&Æö6¶W#¢SbÀÐ¢V"f–7F–Ó¢SbÀÐ§ÐÐ¦–×Â6W$¶–ÆÆVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„6W$¶–ÆÆVDWfVçB°Ð¢&Æö6¶W#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ“S“cc#3sS#C“3ScB’ÀÐ¢&&Æö6¶W""ÀÐ¢“òÀÐ¢f–7F–Ó¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ3#“s#SCCsCCCc—ScB’ÀÐ¢'f–7F–Ò"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢“S“cc#3sS#C“3ScBÓâö²‡6VÆbæ&Æö6¶W"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢3#“s#SCCsCCCc—ScBÓâö²‡6VÆbçf–7F–Òæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$6W$¶–ÆÆVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7BÖ–äÖVçU7F&–Æ—¦VDWfVçB·ÐÐ¦–×ÂÖ–äÖVçU7F&–Æ—¦VDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„Ö–äÖVçU7F&–Æ—¦VDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$Ö–äÖVçU7F&–Æ—¦VB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7Bv÷&ÆE7FGW46†ævVDWfVçB·ÐÐ¦–×Âv÷&ÆE7FGW46†ævVDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…v÷&ÆE7FGW46†ævVDWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%v÷&ÆE7FGW46†ævVB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B„ÅEe7FGW4WfVçB°Ð¢V"6Æ–VçG3¢S3"ÀÐ¢V"6Æ÷G3¢S3"ÀÐ¢V"&÷†–W3¢SbÀÐ¢V"Ö7FW#¢Ö–&UWFc…7G&–ærÀÐ§ÐÐ¦–×Â„ÅEe7FGW4WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„„ÅEe7FGW4WfVçB°Ð¢6Æ–VçG3¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒscC“ƒƒsSƒSsS33ScB’ÀÐ¢&6Æ–VçG2"ÀÐ¢“òÀÐ¢6Æ÷G3¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒcc#sCcSSCCCƒ#3sGScB’ÀÐ¢'6Æ÷G2"ÀÐ¢“òÀÐ¢&÷†–W3¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒS3sSS##3scsscccwScB’ÀÐ¢'&÷†–W2"ÀÐ¢“òÀÐ¢Ö7FW#¢&VE÷fÇVS££ÄÖ–&UWFc…7G&–æsâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs#S“cSƒcƒ#CsS3C7ScB’ÀÐ¢&Ö7FW""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢scC“ƒƒsSƒSsS33ScBÓâö²‡6VÆbæ6Æ–VçG2æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢cc#sCcSSCCCƒ#3sGScBÓâö²‡6VÆbç6Æ÷G2æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢S3sSS##3scsscccwScBÓâö²‡6VÆbç&÷†–W2æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s#S“cSƒcƒ#CsS3C7ScBÓâö²‡6VÆbæÖ7FW"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$„ÅEe7FGW2"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B„ÅEd6ÖW&ÖäWfVçB°Ð¢V"–æFWƒ¢SbÀÐ§ÐÐ¦–×Â„ÅEd6ÖW&ÖäWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„„ÅEd6ÖW&ÖäWfVçB°Ð¢–æFWƒ¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ“C“s“ccƒƒcC3S#C#3WScB’ÀÐ¢&–æFW‚"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢“C“s“ccƒƒcC3S#C#3WScBÓâö²‡6VÆbæ–æFW‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$„ÅEd6ÖW&Öâ"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B„ÅEe&æ´6ÖW&WfVçB°Ð¢V"–æFWƒ¢S‚ÀÐ¢V"&æ³¢c3"ÀÐ¢V"F&vWC¢SbÀÐ§ÐÐ¦–×Â„ÅEe&æ´6ÖW&WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„„ÅEe&æ´6ÖW&WfVçB°Ð¢–æFWƒ¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ“C“s“ccƒƒcC3S#C#3WScB’ÀÐ¢&–æFW‚"ÀÐ¢“òÀÐ¢&æ³¢&VE÷fÇVS££Æc3#â‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒs“Cc“s“#3#ƒ“cwScB’Â'&æ²"“òÀÐ¢F&vWC¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒcS3“cS“SssssS'ScB’ÀÐ¢'F&vWB"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢“C“s“ccƒƒcC3S#C#3WScBÓâö²‡6VÆbæ–æFW‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s“Cc“s“#3#ƒ“cwScBÓâö²‡6VÆbç&æ²æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢cS3“cS“SssssS'ScBÓâö²‡6VÆbçF&vWBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$„ÅEe&æ´6ÖW&"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B„ÅEe&æ´VçF—G”WfVçB°Ð¢V"–æFWƒ¢SbÀÐ¢V"&æ³¢c3"ÀÐ¢V"F&vWC¢SbÀÐ§ÐÐ¦–×Â„ÅEe&æ´VçF—G”WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„„ÅEe&æ´VçF—G”WfVçB°Ð¢–æFWƒ¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ“C“s“ccƒƒcC3S#C#3WScB’ÀÐ¢&–æFW‚"ÀÐ¢“òÀÐ¢&æ³¢&VE÷fÇVS££Æc3#â‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒs“Cc“s“#3#ƒ“cwScB’Â'&æ²"“òÀÐ¢F&vWC¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒcS3“cS“SssssS'ScB’ÀÐ¢'F&vWB"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢“C“s“ccƒƒcC3S#C#3WScBÓâö²‡6VÆbæ–æFW‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s“Cc“s“#3#ƒ“cwScBÓâö²‡6VÆbç&æ²æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢cS3“cS“SssssS'ScBÓâö²‡6VÆbçF&vWBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$„ÅEe&æ´VçF—G’"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B„ÅEdf—†VDWfVçB°Ð¢V"÷5÷ƒ¢S3"ÀÐ¢V"÷5÷“¢S3"ÀÐ¢V"÷5÷£¢S3"ÀÐ¢V"F†WF¢SbÀÐ¢V"†“¢SbÀÐ¢V"öfg6WC¢SbÀÐ¢V"f÷c¢c3"ÀÐ¢V"F&vWC¢SbÀÐ§ÐÐ¦–×Â„ÅEdf—†VDWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„„ÅEdf—†VDWfVçB°Ð¢÷5÷ƒ¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒS3SS3““Cs““33WScB’ÀÐ¢'÷5÷‚"ÀÐ¢“òÀÐ¢÷5÷“¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒS3CCCC3c3cSGScB’ÀÐ¢'÷5÷’"ÀÐ¢“òÀÐ¢÷5÷£¢&VE÷fÇVS££ÇS3#â€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒS3ss3ƒ“s#C“s3wScB’ÀÐ¢'÷5÷¢"ÀÐ¢“òÀÐ¢F†WF¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc“#33“S#CsSCƒ7ScB’ÀÐ¢'F†WF"ÀÐ¢“òÀÐ¢†“¢&VE÷fÇVS££ÇScâ‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒƒc#SSC#s#c#C#sƒ‡ScB’Â'†’"“òÀÐ¢öfg6WC¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs3Sƒ3cSc3ƒCScgScB’ÀÐ¢&öfg6WB"ÀÐ¢“òÀÐ¢f÷c¢&VE÷fÇVS££Æc3#â‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒS“#““cƒ““S3“3ƒƒGScB’Â&f÷b"“òÀÐ¢F&vWC¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒcS3“cS“SssssS'ScB’ÀÐ¢'F&vWB"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢S3SS3““Cs““33WScBÓâö²‡6VÆbç÷5÷‚æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢S3CCCC3c3cSGScBÓâö²‡6VÆbç÷5÷’æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢S3ss3ƒ“s#C“s3wScBÓâö²‡6VÆbç÷5÷¢æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢c“#33“S#CsSCƒ7ScBÓâö²‡6VÆbçF†WFæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢ƒc#SSC#s#c#C#sƒ‡ScBÓâö²‡6VÆbç†’æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s3Sƒ3cSc3ƒCScgScBÓâö²‡6VÆbæöfg6WBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢S“#““cƒ““S3“3ƒƒGScBÓâö²‡6VÆbæf÷bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢cS3“cS“SssssS'ScBÓâö²‡6VÆbçF&vWBæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$„ÅEdf—†VB"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B„ÅEd6†6TWfVçB°Ð¢V"F&vWEó¢SbÀÐ¢V"F&vWEó#¢SbÀÐ¢V"F—7Fæ6S¢SbÀÐ¢V"F†WF¢SbÀÐ¢V"†“¢SbÀÐ¢V"–æW'F–¢S‚ÀÐ¢V"–åöW–S¢S‚ÀÐ§ÐÐ¦–×Â„ÅEd6†6TWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„„ÅEd6†6TWfVçB°Ð¢F&vWEó¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs#CCS3ƒs“S#“3sCscC7ScB’ÀÐ¢'F&vWEó"ÀÐ¢“òÀÐ¢F&vWEó#¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒs#CCS3“ƒ“CƒS3sSƒSGScB’ÀÐ¢'F&vWEó""ÀÐ¢“òÀÐ¢F—7Fæ6S¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒS“ScƒSSSc““SCƒƒ'ScB’ÀÐ¢&F—7Fæ6R"ÀÐ¢“òÀÐ¢F†WF¢&VE÷fÇVS££ÇScâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc“#33“S#CsSCƒ7ScB’ÀÐ¢'F†WF"ÀÐ¢“òÀÐ¢†“¢&VE÷fÇVS££ÇScâ‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒƒc#SSC#s#c#C#sƒ‡ScB’Â'†’"“òÀÐ¢–æW'F–¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc#ssSƒCsCsS#c37ScB’ÀÐ¢&–æW'F–"ÀÐ¢“òÀÐ¢–åöW–S¢&VE÷fÇVS££ÇSƒâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒ#c“S#“3ƒ“sS“#ƒwScB’ÀÐ¢&–åöW–R"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢s#CCS3ƒs“S#“3sCscC7ScBÓâö²‡6VÆbçF&vWEóæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢s#CCS3“ƒ“CƒS3sSƒSGScBÓâö²‡6VÆbçF&vWEó"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢S“ScƒSSSc““SCƒƒ'ScBÓâö²‡6VÆbæF—7Fæ6Ræ6ÆöæR‚’æ–çFò‚’’ÀÐ¢c“#33“S#CsSCƒ7ScBÓâö²‡6VÆbçF†WFæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢ƒc#SSC#s#c#C#sƒ‡ScBÓâö²‡6VÆbç†’æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢c#ssSƒCsCsS#c37ScBÓâö²‡6VÆbæ–æW'F–æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢#c“S#“3ƒ“sS“#ƒwScBÓâö²‡6VÆbæ–åöW–Ræ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$„ÅEd6†6R"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B„ÅEdÖW76vTWfVçB°Ð¢V"FW‡C¢Ö–&UWFc…7G&–ærÀÐ§ÐÐ¦–×Â„ÅEdÖW76vTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„„ÅEdÖW76vTWfVçB°Ð¢FW‡C¢&VE÷fÇVS££ÄÖ–&UWFc…7G&–æsâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒSs“3ssS#3““CƒgScB’ÀÐ¢'FW‡B"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢ƒSs“3ssS#3““CƒgScBÓâö²‡6VÆbçFW‡Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$„ÅEdÖW76vR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B„ÅEeF—FÆTWfVçB°Ð¢V"FW‡C¢Ö–&UWFc…7G&–ærÀÐ§ÐÐ¦–×Â„ÅEeF—FÆTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„„ÅEeF—FÆTWfVçB°Ð¢FW‡C¢&VE÷fÇVS££ÄÖ–&UWFc…7G&–æsâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒSs“3ssS#3““CƒgScB’ÀÐ¢'FW‡B"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢ƒSs“3ssS#3““CƒgScBÓâö²‡6VÆbçFW‡Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$„ÅEeF—FÆR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B„ÅEd6†DWfVçB°Ð¢V"FW‡C¢Ö–&UWFc…7G&–ærÀÐ§ÐÐ¦–×Â„ÅEd6†DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²„„ÅEd6†DWfVçB°Ð¢FW‡C¢&VE÷fÇVS££ÄÖ–&UWFc…7G&–æsâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒƒSs“3ssS#3““CƒgScB’ÀÐ¢'FW‡B"ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢ƒSs“3ssS#3““CƒgScBÓâö²‡6VÆbçFW‡Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢$„ÅEd6†B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&WÆ•7F'E&V6÷&DWfVçB·ÐÐ¦–×Â&WÆ•7F'E&V6÷&DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&WÆ•7F'E&V6÷&DWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&WÆ•7F'E&V6÷&B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&WÆ•6W76–öä–æfôWfVçB°Ð¢V"6ã¢Ö–&UWFc…7G&–ærÀÐ¢V"F“¢S‚ÀÐ¢V"6#¢S3"ÀÐ¢V"7C¢S3"ÀÐ§ÐÐ¦–×Â&WÆ•6W76–öä–æfôWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&WÆ•6W76–öä–æfôWfVçB°Ð¢6ã¢&VE÷fÇVS££ÄÖ–&UWFc…7G&–æsâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒc3sc#C#ƒƒSgScB’ÀÐ¢'6â"ÀÐ¢“òÀÐ¢F“¢&VE÷fÇVS££ÇSƒâ‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒcs3s3CSC3CCC#sScB’Â&F’"“òÀÐ¢6#¢&VE÷fÇVS££ÇS3#â‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒc##C3#“CS#Sc#“gScB’Â&6""“òÀÐ¢7C¢&VE÷fÇVS££ÇS3#â‡7G&VÒÂFVf–æ—F–öâævWEöVçG'’ƒc3sc3C#3#cCc3#cgScB’Â'7B"“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢c3sc#C#ƒƒSgScBÓâö²‡6VÆbç6âæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢cs3s3CSC3CCC#sScBÓâö²‡6VÆbæF’æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢c##C3#“CS#Sc#“gScBÓâö²‡6VÆbæ6"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢c3sc3C#3#cCc3#cgScBÓâö²‡6VÆbç7Bæ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&WÆ•6W76–öä–æfò"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&WÆ”VæE&V6÷&DWfVçB·ÐÐ¦–×Â&WÆ”VæE&V6÷&DWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&WÆ”VæE&V6÷&DWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&WÆ”VæE&V6÷&B"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&WÆ•&WÆ—4f–Æ&ÆTWfVçB·ÐÐ¦–×Â&WÆ•&WÆ—4f–Æ&ÆTWfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&WÆ•&WÆ—4f–Æ&ÆTWfVçB·ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&WÆ•&WÆ—4f–Æ&ÆR"ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ§V"7G'V7B&WÆ•6W'fW$W'&÷$WfVçB°Ð¢V"W'&÷#¢Ö–&UWFc…7G&–ærÀÐ§ÐÐ¦–×Â&WÆ•6W'fW$W'&÷$WfVçB°Ð¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²…&WÆ•6W'fW$W'&÷$WfVçB°Ð¢W'&÷#¢&VE÷fÇVS££ÄÖ–&UWFc…7G&–æsâ€Ð¢7G&VÒÀÐ¢FVf–æ—F–öâævWEöVçG'’ƒCƒ“ƒ““ccCCsCc—ScB’ÀÐ¢&W'&÷""ÀÐ¢“òÀÐ¢ÒÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâvWEöf–VÆB‚g6VÆbÂf–VÆC¢dvÖTWfVçDVçG'’’Óâ&W7VÇCÄvÖTWfVçEfÇVSâ°Ð¢5¶ÆÆ÷r†6Æ—“£¦6ÆöæUööåö6÷’Â6Æ—“£¦ÖF6…÷6–ævÆUö&–æF–ær•ÐÐ¢ÖF6‚f–VÆBæ†6‚°Ð¢Cƒ“ƒ““ccCCsCc—ScBÓâö²‡6VÆbæW'&÷"æ6ÆöæR‚’æ–çFò‚’’ÀÐ¢òÓâW'"…'6TW'&÷#£¤Ö—76–ætvÖTWfVçEfÇVR°Ð¢G“¢%&WÆ•6W'fW$W'&÷""ÀÐ¢f–VÆC¢'FöFò"æ–çFò‚’ÀÐ¢Ò’ÀÐ¢ÐÐ¢ÐÐ¢5¶ÆÆ÷r‡VçW6VE÷f&–&ÆW2•ÐÐ¢fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢f÷"VçG'’–âfFVf–æ—F–öâæVçG&–W2°Ð¢ÆWBfÇVRÒ6VÆ`Ð¢ævWEöf–VÆB†VçG'’Ð¢çVçw&ö÷%öVÇ6R‡Å÷ÂVçG'’æ¶–æBæFVfVÇE÷fÇVR‚’“°Ð¢7G&VÒçw&—FR‚gfÇVR“ó°Ð¢ÐÐ¢ö²‚‚’Ð¢ÐÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„FV'VrÂ'F–ÄWÂ6W&–Æ—¦RÂFW6W&–Æ—¦RÂ6ÆöæR•ÐÐ¢5·6W&FR‡FrÒ'G—R"•ÐÐ§V"VçVÒvÖTWfVçB°Ð¢6W'fW%7vâ„&÷ƒÅ6W'fW%7väWfVçCâ’ÀÐ¢6W'fW$6†ævTÆWfVÄf–ÆVB…6W'fW$6†ævTÆWfVÄf–ÆVDWfVçB’ÀÐ¢6W'fW%6‡WFF÷vâ…6W'fW%6‡WFF÷väWfVçB’ÀÐ¢6W'fW$7f"…6W'fW$7f$WfVçB’ÀÐ¢6W'fW$ÖW76vR…6W'fW$ÖW76vTWfVçB’ÀÐ¢6W'fW$FD&â„&÷ƒÅ6W'fW$FD&äWfVçCâ’ÀÐ¢6W'fW%&VÖ÷fT&â…6W'fW%&VÖ÷fT&äWfVçB’ÀÐ¢Æ–W$6öææV7B…Æ–W$6öææV7DWfVçB’ÀÐ¢Æ–W$6öææV7D6Æ–VçB…Æ–W$6öææV7D6Æ–VçDWfVçB’ÀÐ¢Æ–W$–æfò…Æ–W$–æfôWfVçB’ÀÐ¢Æ–W$F—66öææV7B…Æ–W$F—66öææV7DWfVçB’ÀÐ¢Æ–W$7F—fFR…Æ–W$7F—fFTWfVçB’ÀÐ¢Æ–W%6’…Æ–W%6”WfVçB’ÀÐ¢6Æ–VçDF—66öææV7B„6Æ–VçDF—66öææV7DWfVçB’ÀÐ¢6Æ–VçD&Vv–ä6öææV7B„6Æ–VçD&Vv–ä6öææV7DWfVçB’ÀÐ¢6Æ–VçD6öææV7FVB„6Æ–VçD6öææV7FVDWfVçB’ÀÐ¢6Æ–VçDgVÆÄ6öææV7B„6Æ–VçDgVÆÄ6öææV7DWfVçB’ÀÐ¢†÷7EV—B„†÷7EV—DWfVçB’ÀÐ¢FVÔ–æfò…FVÔ–æfôWfVçB’ÀÐ¢FVÕ66÷&R…FVÕ66÷&TWfVçB’ÀÐ¢FVÕÆ”'&öF67DVF–ò…FVÕÆ”'&öF67DVF–ôWfVçB’ÀÐ¢Æ–W%FVÒ…Æ–W%FVÔWfVçB’ÀÐ¢Æ–W$6Æ72…Æ–W$6Æ74WfVçB’ÀÐ¢Æ–W$FVF‚„&÷ƒÅÆ–W$FVF„WfVçCâ’ÀÐ¢Æ–W$‡W'B…Æ–W$‡W'DWfVçB’ÀÐ¢Æ–W$6†B…Æ–W$6†DWfVçB’ÀÐ¢Æ–W%66÷&R…Æ–W%66÷&TWfVçB’ÀÐ¢Æ–W%7vâ…Æ–W%7väWfVçB’ÀÐ¢Æ–W%6†ö÷B…Æ–W%6†ö÷DWfVçB’ÀÐ¢Æ–W%W6R…Æ–W%W6TWfVçB’ÀÐ¢Æ–W$6†ævTæÖR…Æ–W$6†ævTæÖTWfVçB’ÀÐ¢Æ–W$†–çDÖW76vR…Æ–W$†–çDÖW76vTWfVçB’ÀÐ¢&6UÆ–W%FVÆW÷'FVB„&6UÆ–W%FVÆW÷'FVDWfVçB’ÀÐ¢vÖT–æ—B„vÖT–æ—DWfVçB’ÀÐ¢vÖTæWtÖ„vÖTæWtÖWfVçB’ÀÐ¢vÖU7F'B„vÖU7F'DWfVçB’ÀÐ¢vÖTVæB„vÖTVæDWfVçB’ÀÐ¢&÷VæE7F'B…&÷VæE7F'DWfVçB’ÀÐ¢&÷VæDVæB…&÷VæDVæDWfVçB’ÀÐ¢vÖTÖW76vR„vÖTÖW76vTWfVçB’ÀÐ¢'&V´'&V¶&ÆR„'&V´'&V¶&ÆTWfVçB’ÀÐ¢'&Vµ&÷„'&Vµ&÷WfVçB’ÀÐ¢VçF—G”¶–ÆÆVB„VçF—G”¶–ÆÆVDWfVçB’ÀÐ¢&öçW5WFFVB„&öçW5WFFVDWfVçB’ÀÐ¢6†–WfVÖVçDWfVçB„6†–WfVÖVçDWfVçDWfVçB’ÀÐ¢6†–WfVÖVçD–æ7&VÖVçB„6†–WfVÖVçD–æ7&VÖVçDWfVçB’ÀÐ¢‡—6wVå–6·W…‡—6wVå–6·WWfVçB’ÀÐ¢fÆ&T–væ—FTç2„fÆ&T–væ—FTç4WfVçB’ÀÐ¢†VÆ–6÷FW$w&VæFUVçDÖ—72„†VÆ–6÷FW$w&VæFUVçDÖ—74WfVçB’ÀÐ¢W6W$FFF÷væÆöFVB…W6W$FFF÷væÆöFVDWfVçB’ÀÐ¢&vFöÆÄF—76öÇfVB…&vFöÆÄF—76öÇfVDWfVçB’ÀÐ¢„ÅEd6†ævVDÖöFR„„ÅEd6†ævVDÖöFTWfVçB’ÀÐ¢„ÅEd6†ævVEF&vWB„„ÅEd6†ævVEF&vWDWfVçB’ÀÐ¢f÷FTVæFVB…f÷FTVæFVDWfVçB’ÀÐ¢f÷FU7F'FVB…f÷FU7F'FVDWfVçB’ÀÐ¢f÷FT6†ævVB…f÷FT6†ævVDWfVçB’ÀÐ¢f÷FU76VB…f÷FU76VDWfVçB’ÀÐ¢f÷FTf–ÆVB…f÷FTf–ÆVDWfVçB’ÀÐ¢f÷FT67B…f÷FT67DWfVçB’ÀÐ¢f÷FT÷F–öç2„&÷ƒÅf÷FT÷F–öç4WfVçCâ’ÀÐ¢&WÆ•6fVB…&WÆ•6fVDWfVçB’ÀÐ¢VçFW&VEW&f÷&Öæ6TÖöFR„VçFW&VEW&f÷&Öæ6TÖöFTWfVçB’ÀÐ¢'&÷w6U&WÆ—2„'&÷w6U&WÆ—4WfVçB’ÀÐ¢&WÆ•–÷WGV&U7FG2…&WÆ•–÷WGV&U7FG4WfVçB’ÀÐ¢–çfVçF÷'•WFFVB„–çfVçF÷'•WFFVDWfVçB’ÀÐ¢6'EWFFVB„6'EWFFVDWfVçB’ÀÐ¢7F÷&U&–6U6†VWEWFFVB…7F÷&U&–6U6†VWEWFFVDWfVçB’ÀÐ¢V6öä–çfVçF÷'”6öææV7FVB„V6öä–çfVçF÷'”6öææV7FVDWfVçB’ÀÐ¢—FVÕ66†VÖ–æ—F–Æ—¦VB„—FVÕ66†VÖ–æ—F–Æ—¦VDWfVçB’ÀÐ¢v4æWu6W76–öâ„v4æWu6W76–öäWfVçB’ÀÐ¢v4Æ÷7E6W76–öâ„v4Æ÷7E6W76–öäWfVçB’ÀÐ¢–çG&ôf–æ—6‚„–çG&ôf–æ—6„WfVçB’ÀÐ¢–çG&ôæW‡D6ÖW&„–çG&ôæW‡D6ÖW&WfVçB’ÀÐ¢Æ–W$6†ævT6Æ72…Æ–W$6†ævT6Æ74WfVçB’ÀÐ¢FdÖF–ÖU&VÖ–æ–ær…FdÖF–ÖU&VÖ–æ–ætWfVçB’ÀÐ¢FdvÖT÷fW"…FdvÖT÷fW$WfVçB’ÀÐ¢7FdfÆt6GW&VB„7FdfÆt6GW&VDWfVçB’ÀÐ¢6öçG&öÅö–çD–æ—F–Æ—¦VB„6öçG&öÅö–çD–æ—F–Æ—¦VDWfVçB’ÀÐ¢6öçG&öÅö–çEWFFT–ÖvW2„6öçG&öÅö–çEWFFT–ÖvW4WfVçB’ÀÐ¢6öçG&öÅö–çEWFFTÆ–÷WB„6öçG&öÅö–çEWFFTÆ–÷WDWfVçB’ÀÐ¢6öçG&öÅö–çEWFFT6–ær„6öçG&öÅö–çEWFFT6–ætWfVçB’ÀÐ¢6öçG&öÅö–çEWFFT÷væW"„6öçG&öÅö–çEWFFT÷væW$WfVçB’ÀÐ¢6öçG&öÅö–çE7F'EF÷V6‚„6öçG&öÅö–çE7F'EF÷V6„WfVçB’ÀÐ¢6öçG&öÅö–çDVæEF÷V6‚„6öçG&öÅö–çDVæEF÷V6„WfVçB’ÀÐ¢6öçG&öÅö–çEVÇ6TVÆVÖVçB„6öçG&öÅö–çEVÇ6TVÆVÖVçDWfVçB’ÀÐ¢6öçG&öÅö–çDf¶T6GW&R„6öçG&öÅö–çDf¶T6GW&TWfVçB’ÀÐ¢6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W"„6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W$WfVçB’ÀÐ¢FVÕÆ•&÷VæE6VÆV7FVB…FVÕÆ•&÷VæE6VÆV7FVDWfVçB’ÀÐ¢FVÕÆ•&÷VæE7F'B…FVÕÆ•&÷VæE7F'DWfVçB’ÀÐ¢FVÕÆ•&÷VæD7F—fR…FVÕÆ•&÷VæD7F—fTWfVçB’ÀÐ¢FVÕÆ•v—F–æt&Vv–ç2…FVÕÆ•v—F–æt&Vv–ç4WfVçB’ÀÐ¢FVÕÆ•v—F–ætVæG2…FVÕÆ•v—F–ætVæG4WfVçB’ÀÐ¢FVÕÆ•v—F–æt&÷WEFôVæB…FVÕÆ•v—F–æt&÷WEFôVæDWfVçB’ÀÐ¢FVÕÆ•&W7F'E&÷VæB…FVÕÆ•&W7F'E&÷VæDWfVçB’ÀÐ¢FVÕÆ•&VG•&W7F'B…FVÕÆ•&VG•&W7F'DWfVçB’ÀÐ¢FVÕÆ•&÷VæE&W7F'E6V6öæG2…FVÕÆ•&÷VæE&W7F'E6V6öæG4WfVçB’ÀÐ¢FVÕÆ•FVÕ&VG’…FVÕÆ•FVÕ&VG”WfVçB’ÀÐ¢FVÕÆ•&÷VæEv–â…FVÕÆ•&÷VæEv–äWfVçB’ÀÐ¢FVÕÆ•WFFUF–ÖW"…FVÕÆ•WFFUF–ÖW$WfVçB’ÀÐ¢FVÕÆ•&÷VæE7FÆVÖFR…FVÕÆ•&÷VæE7FÆVÖFTWfVçB’ÀÐ¢FVÕÆ”÷fW'F–ÖT&Vv–â…FVÕÆ”÷fW'F–ÖT&Vv–äWfVçB’ÀÐ¢FVÕÆ”÷fW'F–ÖTVæB…FVÕÆ”÷fW'F–ÖTVæDWfVçB’ÀÐ¢FVÕÆ•7VFFVäFVF„&Vv–â…FVÕÆ•7VFFVäFVF„&Vv–äWfVçB’ÀÐ¢FVÕÆ•7VFFVäFVF„VæB…FVÕÆ•7VFFVäFVF„VæDWfVçB’ÀÐ¢FVÕÆ”vÖT÷fW"…FVÕÆ”vÖT÷fW$WfVçB’ÀÐ¢FVÕÆ”ÖF–ÖU&VÖ–æ–ær…FVÕÆ”ÖF–ÖU&VÖ–æ–ætWfVçB’ÀÐ¢FVÕÆ•F–ÖW$fÆ6‚…FVÕÆ•F–ÖW$fÆ6„WfVçB’ÀÐ¢FVÕÆ•F–ÖW%F–ÖTFFVB…FVÕÆ•F–ÖW%F–ÖTFFVDWfVçB’ÀÐ¢FVÕÆ•ö–çE7F'D6GW&R…FVÕÆ•ö–çE7F'D6GW&TWfVçB’ÀÐ¢FVÕÆ•ö–çD6GW&VB…FVÕÆ•ö–çD6GW&VDWfVçB’ÀÐ¢FVÕÆ•ö–çDÆö6¶VB…FVÕÆ•ö–çDÆö6¶VDWfVçB’ÀÐ¢FVÕÆ•ö–çEVæÆö6¶VB…FVÕÆ•ö–çEVæÆö6¶VDWfVçB’ÀÐ¢FVÕÆ”6GW&T'&ö¶Vâ…FVÕÆ”6GW&T'&ö¶VäWfVçB’ÀÐ¢FVÕÆ”6GW&T&Æö6¶VB…FVÕÆ”6GW&T&Æö6¶VDWfVçB’ÀÐ¢FVÕÆ”fÆtWfVçB…FVÕÆ”fÆtWfVçDWfVçB’ÀÐ¢FVÕÆ•v–åæVÂ…FVÕÆ•v–åæVÄWfVçB’ÀÐ¢FVÕÆ•FVÔ&Ææ6VEÆ–W"…FVÕÆ•FVÔ&Ææ6VEÆ–W$WfVçB’ÀÐ¢FVÕÆ•6WGWf–æ—6†VB…FVÕÆ•6WGWf–æ—6†VDWfVçB’ÀÐ¢FVÕÆ”ÆW'B…FVÕÆ”ÆW'DWfVçB’ÀÐ¢G&–æ–æt6ö×ÆWFR…G&–æ–æt6ö×ÆWFTWfVçB’ÀÐ¢6†÷tg&VW¦UæVÂ…6†÷tg&VW¦UæVÄWfVçB’ÀÐ¢†–FTg&VW¦UæVÂ„†–FTg&VW¦UæVÄWfVçB’ÀÐ¢g&VW¦T6Õ7F'FVB„g&VW¦T6Õ7F'FVDWfVçB’ÀÐ¢Æö6ÅÆ–W$6†ævUFVÒ„Æö6ÅÆ–W$6†ævUFVÔWfVçB’ÀÐ¢Æö6ÅÆ–W%66÷&T6†ævVB„Æö6ÅÆ–W%66÷&T6†ævVDWfVçB’ÀÐ¢Æö6ÅÆ–W$6†ævT6Æ72„Æö6ÅÆ–W$6†ævT6Æ74WfVçB’ÀÐ¢Æö6ÅÆ–W%&W7vâ„Æö6ÅÆ–W%&W7väWfVçB’ÀÐ¢'V–ÆF–æt–æfô6†ævVB„'V–ÆF–æt–æfô6†ævVDWfVçB’ÀÐ¢Æö6ÅÆ–W$6†ævTF—6wV—6R„Æö6ÅÆ–W$6†ævTF—6wV—6TWfVçB’ÀÐ¢Æ–W$66÷VçD6†ævVB…Æ–W$66÷VçD6†ævVDWfVçB’ÀÐ¢7•F&W6WB…7•F&W6WDWfVçB’ÀÐ¢fÆu7FGW5WFFR„fÆu7FGW5WFFTWfVçB’ÀÐ¢Æ–W%7FG5WFFVB…Æ–W%7FG5WFFVDWfVçB’ÀÐ¢Æ––æt6öÖÖVçF'’…Æ––æt6öÖÖVçF'”WfVçB’ÀÐ¢Æ–W$6†&vTFWÆ÷–VB…Æ–W$6†&vTFWÆ÷–VDWfVçB’ÀÐ¢Æ–W$'V–ÇDö&¦V7B…Æ–W$'V–ÇDö&¦V7DWfVçB’ÀÐ¢Æ–W%Ww&FVDö&¦V7B…Æ–W%Ww&FVDö&¦V7DWfVçB’ÀÐ¢Æ–W$6''”ö&¦V7B…Æ–W$6''”ö&¦V7DWfVçB’ÀÐ¢Æ–W$G&÷ö&¦V7B…Æ–W$G&÷ö&¦V7DWfVçB’ÀÐ¢ö&¦V7E&VÖ÷fVB„ö&¦V7E&VÖ÷fVDWfVçB’ÀÐ¢ö&¦V7DFW7G&÷–VB„ö&¦V7DFW7G&÷–VDWfVçB’ÀÐ¢ö&¦V7DFWFöæFVB„ö&¦V7DFWFöæFVDWfVçB’ÀÐ¢6†–WfVÖVçDV&æVB„6†–WfVÖVçDV&æVDWfVçB’ÀÐ¢7V5F&vWEWFFVB…7V5F&vWEWFFVDWfVçB’ÀÐ¢F÷W&æÖVçE7FFUWFFR…F÷W&æÖVçE7FFUWFFTWfVçB’ÀÐ¢F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâ…F÷W&æÖVçDVæ&ÆT6÷VçFF÷väWfVçB’ÀÐ¢Æ–W$6ÆÆVDf÷$ÖVF–2…Æ–W$6ÆÆVDf÷$ÖVF–4WfVçB’ÀÐ¢Æ–W$6¶VDf÷$&ÆÂ…Æ–W$6¶VDf÷$&ÆÄWfVçB’ÀÐ¢Æö6ÅÆ–W$&V6ÖTö'6W'fW"„Æö6ÅÆ–W$&V6ÖTö'6W'fW$WfVçB’ÀÐ¢Æ–W$–væ—FVD–çb…Æ–W$–væ—FVD–çdWfVçB’ÀÐ¢Æ–W$–væ—FVB…Æ–W$–væ—FVDWfVçB’ÀÐ¢Æ–W$W‡F–æwV—6†VB…Æ–W$W‡F–æwV—6†VDWfVçB’ÀÐ¢Æ–W%FVÆW÷'FVB…Æ–W%FVÆW÷'FVDWfVçB’ÀÐ¢Æ–W$†VÆVDÖVF–46ÆÂ…Æ–W$†VÆVDÖVF–46ÆÄWfVçB’ÀÐ¢Æö6ÅÆ–W$6†&vU&VG’„Æö6ÅÆ–W$6†&vU&VG”WfVçB’ÀÐ¢Æö6ÅÆ–W%v–æDF÷vâ„Æö6ÅÆ–W%v–æDF÷väWfVçB’ÀÐ¢Æ–W$–çgVÆæVB…Æ–W$–çgVÆæVDWfVçB’ÀÐ¢W66÷'E7VVB„W66÷'E7VVDWfVçB’ÀÐ¢W66÷'E&öw&W72„W66÷'E&öw&W74WfVçB’ÀÐ¢W66÷'E&V6VFR„W66÷'E&V6VFTWfVçB’ÀÐ¢vÖUT”7F—fFVB„vÖUT”7F—fFVDWfVçB’ÀÐ¢vÖUT”†–FFVâ„vÖUT”†–FFVäWfVçB’ÀÐ¢Æ–W$W66÷'E66÷&R…Æ–W$W66÷'E66÷&TWfVçB’ÀÐ¢Æ–W$†VÄöä†—B…Æ–W$†VÄöä†—DWfVçB’ÀÐ¢Æ–W%7FVÅ6æGf–6‚…Æ–W%7FVÅ6æGf–6„WfVçB’ÀÐ¢6†÷t6Æ74Æ–÷WB…6†÷t6Æ74Æ–÷WDWfVçB’ÀÐ¢6†÷ug5æVÂ…6†÷ug5æVÄWfVçB’ÀÐ¢Æ–W$FÖvVB…Æ–W$FÖvVDWfVçB’ÀÐ¢&VæÆ–W$æ÷F–f–6F–öâ„&VæÆ–W$æ÷F–f–6F–öäWfVçB’ÀÐ¢&VæÖF6„Ö…7G&V²„&VæÖF6„Ö…7G&V´WfVçB’ÀÐ¢&Væ&÷VæE7F'B„&Væ&÷VæE7F'DWfVçB’ÀÐ¢&Væv–åæVÂ„&Væv–åæVÄWfVçB’ÀÐ¢fUv–åæVÂ…fUv–åæVÄWfVçB’ÀÐ¢—$F6‚„—$F6„WfVçB’ÀÐ¢ÆæFVB„ÆæFVDWfVçB’ÀÐ¢Æ–W$FÖvTFöFvVB…Æ–W$FÖvTFöFvVDWfVçB’ÀÐ¢Æ–W%7GVææVB…Æ–W%7GVææVDWfVçB’ÀÐ¢66÷WDw&æE6ÆÒ…66÷WDw&æE6ÆÔWfVçB’ÀÐ¢66÷WE6ÆÖFöÆÄÆæFVB…66÷WE6ÆÖFöÆÄÆæFVDWfVçB’ÀÐ¢'&÷t–×7B„'&÷t–×7DWfVçB’ÀÐ¢Æ–W$¦&FVB…Æ–W$¦&FVDWfVçB’ÀÐ¢Æ–W$¦&FVDfFR…Æ–W$¦&FVDfFTWfVçB’ÀÐ¢Æ–W%6†–VÆD&Æö6¶VB…Æ–W%6†–VÆD&Æö6¶VDWfVçB’ÀÐ¢Æ–W%–ææVB…Æ–W%–ææVDWfVçB’ÀÐ¢Æ–W$†VÆVD'”ÖVF–2…Æ–W$†VÆVD'”ÖVF–4WfVçB’ÀÐ¢Æ–W%6VDö&¦V7B…Æ–W%6VDö&¦V7DWfVçB’ÀÐ¢—FVÔf÷VæB„—FVÔf÷VæDWfVçB’ÀÐ¢6†÷tææ÷FF–öâ…6†÷tææ÷FF–öäWfVçB’ÀÐ¢†–FTææ÷FF–öâ„†–FTææ÷FF–öäWfVçB’ÀÐ¢÷7D–çfVçF÷'”Æ–6F–öâ…÷7D–çfVçF÷'”Æ–6F–öäWfVçB’ÀÐ¢6öçG&öÅö–çEVæÆö6µWFFVB„6öçG&öÅö–çEVæÆö6µWFFVDWfVçB’ÀÐ¢FWÆ÷”'Vfd&ææW"„FWÆ÷”'Vfd&ææW$WfVçB’ÀÐ¢Æ–W$'Vfb…Æ–W$'VfdWfVçB’ÀÐ¢ÖVF–4FVF‚„ÖVF–4FVF„WfVçB’ÀÐ¢÷fW'F–ÖTær„÷fW'F–ÖTætWfVçB’ÀÐ¢FV×46†ævVB…FV×46†ævVDWfVçB’ÀÐ¢†ÆÆ÷vVVåV×¶–äw&"„†ÆÆ÷vVVåV×¶–äw&$WfVçB’ÀÐ¢&ö6¶WD§V×…&ö6¶WD§V×WfVçB’ÀÐ¢&ö6¶WD§V×ÆæFVB…&ö6¶WD§V×ÆæFVDWfVçB’ÀÐ¢7F–6·”§V×…7F–6·”§V×WfVçB’ÀÐ¢7F–6·”§V×ÆæFVB…7F–6·”§V×ÆæFVDWfVçB’ÀÐ¢&ö6¶WE6´ÆVæ6‚…&ö6¶WE6´ÆVæ6„WfVçB’ÀÐ¢&ö6¶WE6´ÆæFVB…&ö6¶WE6´ÆæFVDWfVçB’ÀÐ¢ÖVF–4FVfVæFVB„ÖVF–4FVfVæFVDWfVçB’ÀÐ¢Æö6ÅÆ–W$†VÆVB„Æö6ÅÆ–W$†VÆVDWfVçB’ÀÐ¢Æ–W$FW7G&÷–VE—T&öÖ"…Æ–W$FW7G&÷–VE—T&öÖ$WfVçB’ÀÐ¢ö&¦V7DFVfÆV7FVB„ö&¦V7DFVfÆV7FVDWfVçB’ÀÐ¢Æ–W$×g…Æ–W$×gWfVçB’ÀÐ¢&–E7väÖö"…&–E7väÖö$WfVçB’ÀÐ¢&–E7vå7VB…&–E7vå7VDWfVçB’ÀÐ¢æd&Æö6¶VB„æd&Æö6¶VDWfVçB’ÀÐ¢F…G&6µ76VB…F…G&6µ76VDWfVçB’ÀÐ¢çVÔ6W'46†ævVB„çVÔ6W'46†ævVDWfVçB’ÀÐ¢Æ–W%&VvVæW&FR…Æ–W%&VvVæW&FTWfVçB’ÀÐ¢WFFU7FGW4—FVÒ…WFFU7FGW4—FVÔWfVçB’ÀÐ¢7FG5&W6WE&÷VæB…7FG5&W6WE&÷VæDWfVçB’ÀÐ¢66÷&U7FG467V×VÆFVEWFFR…66÷&U7FG467V×VÆFVEWFFTWfVçB’ÀÐ¢66÷&U7FG467V×VÆFVE&W6WB…66÷&U7FG467V×VÆFVE&W6WDWfVçB’ÀÐ¢6†–WfVÖVçDV&æVDÆö6Â„6†–WfVÖVçDV&æVDÆö6ÄWfVçB’ÀÐ¢Æ–W$†VÆVB…Æ–W$†VÆVDWfVçB’ÀÐ¢'V–ÆF–æt†VÆVB„'V–ÆF–æt†VÆVDWfVçB’ÀÐ¢—FVÕ–6·W„—FVÕ–6·WWfVçB’ÀÐ¢GVVÅ7FGW2„GVVÅ7FGW4WfVçB’ÀÐ¢f—6„æ÷F–6R„&÷ƒÄf—6„æ÷F–6TWfVçCâ’ÀÐ¢f—6„æ÷F–6T&Ò„&÷ƒÄf—6„æ÷F–6T&ÔWfVçCâ’ÀÐ¢6Ææ÷F–6R„&÷ƒÅ6Ææ÷F–6TWfVçCâ’ÀÐ¢F‡&÷v&ÆT†—B„&÷ƒÅF‡&÷v&ÆT†—DWfVçCâ’ÀÐ¢V×¶–äÆ÷&E7VÖÖöæVB…V×¶–äÆ÷&E7VÖÖöæVDWfVçB’ÀÐ¢V×¶–äÆ÷&D¶–ÆÆVB…V×¶–äÆ÷&D¶–ÆÆVDWfVçB’ÀÐ¢ÖW&6×W57VÖÖöæVB„ÖW&6×W57VÖÖöæVDWfVçB’ÀÐ¢ÖW&6×W4¶–ÆÆVB„ÖW&6×W4¶–ÆÆVDWfVçB’ÀÐ¢ÖW&6×W4W66Uv&æ–ær„ÖW&6×W4W66Uv&æ–ætWfVçB’ÀÐ¢ÖW&6×W4W66VB„ÖW&6×W4W66VDWfVçB’ÀÐ¢W–V&ÆÄ&÷757VÖÖöæVB„W–V&ÆÄ&÷757VÖÖöæVDWfVçB’ÀÐ¢W–V&ÆÄ&÷757GVææVB„W–V&ÆÄ&÷757GVææVDWfVçB’ÀÐ¢W–V&ÆÄ&÷74¶–ÆÆVB„W–V&ÆÄ&÷74¶–ÆÆVDWfVçB’ÀÐ¢W–V&ÆÄ&÷74¶–ÆÆW"„W–V&ÆÄ&÷74¶–ÆÆW$WfVçB’ÀÐ¢W–V&ÆÄ&÷74W66T–ÖÖ–æVçB„W–V&ÆÄ&÷74W66T–ÖÖ–æVçDWfVçB’ÀÐ¢W–V&ÆÄ&÷74W66VB„W–V&ÆÄ&÷74W66VDWfVçB’ÀÐ¢ç4‡W'B„ç4‡W'DWfVçB’ÀÐ¢6öçG&öÅö–çEF–ÖW%WFFVB„6öçG&öÅö–çEF–ÖW%WFFVDWfVçB’ÀÐ¢Æ–W$†–v„f—fU7F'B…Æ–W$†–v„f—fU7F'DWfVçB’ÀÐ¢Æ–W$†–v„f—fT6æ6VÂ…Æ–W$†–v„f—fT6æ6VÄWfVçB’ÀÐ¢Æ–W$†–v„f—fU7V66W72…Æ–W$†–v„f—fU7V66W74WfVçB’ÀÐ¢Æ–W$&öçW5ö–çG2…Æ–W$&öçW5ö–çG4WfVçB’ÀÐ¢Æ–W%Ww&FVB…Æ–W%Ww&FVDWfVçB’ÀÐ¢Æ–W$'W–&6²…Æ–W$'W–&6´WfVçB’ÀÐ¢Æ–W%W6VE÷vW%W&÷GFÆR…Æ–W%W6VE÷vW%W&÷GFÆTWfVçB’ÀÐ¢6‡&—7FÖ4v–gDw&"„6‡&—7FÖ4v–gDw&$WfVçB’ÀÐ¢Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæR…Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæTWfVçB’ÀÐ¢'G•WFFVB…'G•WFFVDWfVçB’ÀÐ¢'G•&Vd6†ævVB…'G•&Vd6†ævVDWfVçB’ÀÐ¢'G”7&—FW&–6†ævVB…'G”7&—FW&–6†ævVDWfVçB’ÀÐ¢'G”–çf—FW46†ævVB…'G”–çf—FW46†ævVDWfVçB’ÀÐ¢'G•VWVU7FFT6†ævVB…'G•VWVU7FFT6†ævVDWfVçB’ÀÐ¢'G”6†B…'G”6†DWfVçB’ÀÐ¢'G”ÖVÖ&W$¦ö–â…'G”ÖVÖ&W$¦ö–äWfVçB’ÀÐ¢'G”ÖVÖ&W$ÆVfR…'G”ÖVÖ&W$ÆVfTWfVçB’ÀÐ¢ÖF6„–çf—FW5WFFVB„ÖF6„–çf—FW5WFFVDWfVçB’ÀÐ¢Æö&'•WFFVB„Æö&'•WFFVDWfVçB’ÀÐ¢×fÔÖ—76–öåWFFR„×fÔÖ—76–öåWFFTWfVçB’ÀÐ¢&V6Æ7VÆFT†öÆ–F—2…&V6Æ7VÆFT†öÆ–F—4WfVçB’ÀÐ¢Æ–W$7W'&Væ7”6†ævVB…Æ–W$7W'&Væ7”6†ævVDWfVçB’ÀÐ¢Föö×6F•&ö6¶WD÷Vâ„Föö×6F•&ö6¶WD÷VäWfVçB’ÀÐ¢&VÖ÷fTæVÖW6—5&VÆF–öç6†—2…&VÖ÷fTæVÖW6—5&VÆF–öç6†—4WfVçB’ÀÐ¢×fÔ7&VF—D&öçW5vfR„×fÔ7&VF—D&öçW5vfTWfVçB’ÀÐ¢×fÔ7&VF—D&öçW4ÆÂ„×fÔ7&VF—D&öçW4ÆÄWfVçB’ÀÐ¢×fÔ7&VF—D&öçW4ÆÄGfæ6VB„×fÔ7&VF—D&öçW4ÆÄGfæ6VDWfVçB’ÀÐ¢×fÕV–6µ6VçG'•Ww&FR„×fÕV–6µ6VçG'•Ww&FTWfVçB’ÀÐ¢×fÕFæ´FW7G&÷–VD'•Æ–W'2„×fÕFæ´FW7G&÷–VD'•Æ–W'4WfVçB’ÀÐ¢×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ"„×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ$WfVçB’ÀÐ¢×fÕ–6·W7W'&Væ7’„×fÕ–6·W7W'&Væ7”WfVçB’ÀÐ¢×fÔ&öÖ$6'&–W$¶–ÆÆVB„×fÔ&öÖ$6'&–W$¶–ÆÆVDWfVçB’ÀÐ¢×fÕ6VçG'”'W7FW$FWFöæFR„×fÕ6VçG'”'W7FW$FWFöæFTWfVçB’ÀÐ¢×fÕ66÷WDÖ&¶VDf÷$FVF‚„×fÕ66÷WDÖ&¶VDf÷$FVF„WfVçB’ÀÐ¢×fÔÖVF–5÷vW%W6†&VB„×fÔÖVF–5÷vW%W6†&VDWfVçB’ÀÐ¢×fÔ&Vv–åvfR„×fÔ&Vv–åvfTWfVçB’ÀÐ¢×fÕvfT6ö×ÆWFR„×fÕvfT6ö×ÆWFTWfVçB’ÀÐ¢×fÔÖ—76–öä6ö×ÆWFR„×fÔÖ—76–öä6ö×ÆWFTWfVçB’ÀÐ¢×fÔ&öÖ%&W6WD'•Æ–W"„×fÔ&öÖ%&W6WD'•Æ–W$WfVçB’ÀÐ¢×fÔ&öÖ$Æ&ÕG&–vvW&VB„×fÔ&öÖ$Æ&ÕG&–vvW&VDWfVçB’ÀÐ¢×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W"„×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W$WfVçB’ÀÐ¢×fÕvfTf–ÆVB„×fÕvfTf–ÆVDWfVçB’ÀÐ¢×fÕ&W6WE7FG2„×fÕ&W6WE7FG4WfVçB’ÀÐ¢FÖvU&W6—7FVB„FÖvU&W6—7FVDWfVçB’ÀÐ¢&Wf—fUÆ–W$æ÷F–g’…&Wf—fUÆ–W$æ÷F–g”WfVçB’ÀÐ¢&Wf—fUÆ–W%7F÷VB…&Wf—fUÆ–W%7F÷VDWfVçB’ÀÐ¢&Wf—fUÆ–W$6ö×ÆWFR…&Wf—fUÆ–W$6ö×ÆWFTWfVçB’ÀÐ¢Æ–W%GW&æVEFôv†÷7B…Æ–W%GW&æVEFôv†÷7DWfVçB’ÀÐ¢ÖVF–wVå6†–VÆD&Æö6¶VDFÖvR„ÖVF–wVå6†–VÆD&Æö6¶VDFÖvTWfVçB’ÀÐ¢×fÔGevfT6ö×ÆWFTæôvFW2„×fÔGevfT6ö×ÆWFTæôvFW4WfVçB’ÀÐ¢×fÕ6æ—W$†VG6†÷D7W'&Væ7’„×fÕ6æ—W$†VG6†÷D7W'&Væ7”WfVçB’ÀÐ¢×fÔÖææ†GFå—B„×fÔÖææ†GFå—DWfVçB’ÀÐ¢fÆt6'&–VD–äFWFV7F–öå¦öæR„fÆt6'&–VD–äFWFV7F–öå¦öæTWfVçB’ÀÐ¢×fÔGevfT¶–ÆÆVE7GVå&F–ò„×fÔGevfT¶–ÆÆVE7GVå&F–ôWfVçB’ÀÐ¢Æ–W$F—&V7D†—E7GVâ…Æ–W$F—&V7D†—E7GVäWfVçB’ÀÐ¢×fÕ6VçG'”'W7FW$¶–ÆÆVB„×fÕ6VçG'”'W7FW$¶–ÆÆVDWfVçB’ÀÐ¢Ww&FW4f–ÆT6†ævVB…Ww&FW4f–ÆT6†ævVDWfVçB’ÀÐ¢&EFVÕö–çG46†ævVB…&EFVÕö–çG46†ævVDWfVçB’ÀÐ¢&E'VÆW57FFT6†ævVB…&E'VÆW57FFT6†ævVDWfVçB’ÀÐ¢&E&ö&÷D¶–ÆÆVB…&E&ö&÷D¶–ÆÆVDWfVçB’ÀÐ¢&E&ö&÷D–×7B…&E&ö&÷D–×7DWfVçB’ÀÐ¢FVÕÆ•&U&÷VæEF–ÖTÆVgB…FVÕÆ•&U&÷VæEF–ÖTÆVgDWfVçB’ÀÐ¢&6‡WFTFWÆ÷’…&6‡WFTFWÆ÷”WfVçB’ÀÐ¢&6‡WFT†öÇ7FW"…&6‡WFT†öÇ7FW$WfVçB’ÀÐ¢¶–ÆÅ&Vf–ÆÇ4ÖWFW"„¶–ÆÅ&Vf–ÆÇ4ÖWFW$WfVçB’ÀÐ¢'5FVçDWfVçB…'5FVçDWfVçDWfVçB’ÀÐ¢6öæv¶–ÆÂ„6öæv¶–ÆÄWfVçB’ÀÐ¢Æ–W$–æ—F–Å7vâ…Æ–W$–æ—F–Å7väWfVçB’ÀÐ¢6ö×WF—F—fUf–7F÷'’„6ö×WF—F—fUf–7F÷'”WfVçB’ÀÐ¢6ö×WF—F—fU7FG5WFFR„6ö×WF—F—fU7FG5WFFTWfVçB’ÀÐ¢Ö–æ”vÖUv–â„Ö–æ”vÖUv–äWfVçB’ÀÐ¢6VçG'”öävô7F—fR…6VçG'”öävô7F—fTWfVçB’ÀÐ¢GV6µ‡ÆWfVÅW„GV6µ‡ÆWfVÅWWfVçB’ÀÐ¢VW7DÆöt÷VæVB…VW7DÆöt÷VæVDWfVçB’ÀÐ¢66†VÖWFFVB…66†VÖWFFVDWfVçB’ÀÐ¢Æö6ÅÆ–W%–6·WvVöâ„Æö6ÅÆ–W%–6·WvVöäWfVçB’ÀÐ¢&EÆ–W%66÷&Uö–çG2…&EÆ–W%66÷&Uö–çG4WfVçB’ÀÐ¢FVÖöÖäFWE7F–6¶–W2„FVÖöÖäFWE7F–6¶–W4WfVçB’ÀÐ¢VW7Dö&¦V7F—fT6ö×ÆWFVB…VW7Dö&¦V7F—fT6ö×ÆWFVDWfVçB’ÀÐ¢Æ–W%66÷&T6†ævVB…Æ–W%66÷&T6†ævVDWfVçB’ÀÐ¢¶–ÆÆVD6–æuÆ–W"„¶–ÆÆVD6–æuÆ–W$WfVçB’ÀÐ¢Vçf—&öæÖVçFÄFVF‚„Vçf—&öæÖVçFÄFVF„WfVçB’ÀÐ¢&ö¦V7F–ÆTF—&V7D†—B…&ö¦V7F–ÆTF—&V7D†—DWfVçB’ÀÐ¢74vWB…74vWDWfVçB’ÀÐ¢7566÷&R…7566÷&TWfVçB’ÀÐ¢74g&VR…74g&VTWfVçB’ÀÐ¢75746Vv‡B…75746Vv‡DWfVçB’ÀÐ¢74&ÆÅ7FöÆVâ…74&ÆÅ7FöÆVäWfVçB’ÀÐ¢74&ÆÄ&Æö6¶VB…74&ÆÄ&Æö6¶VDWfVçB’ÀÐ¢FÖvU&WfVçFVB„FÖvU&WfVçFVDWfVçB’ÀÐ¢†ÆÆ÷vVVä&÷74¶–ÆÆVB„†ÆÆ÷vVVä&÷74¶–ÆÆVDWfVçB’ÀÐ¢W66VDÆö÷D—6ÆæB„W66VDÆö÷D—6ÆæDWfVçB’ÀÐ¢FvvVEÆ–W$4—B…FvvVEÆ–W$4—DWfVçB’ÀÐ¢ÖW&6×W57GVææVB„ÖW&6×W57GVææVDWfVçB’ÀÐ¢ÖW&6×W5&÷f÷VæB„ÖW&6×W5&÷f÷VæDWfVçB’ÀÐ¢†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVB„†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVDWfVçB’ÀÐ¢6¶VÆWFöä¶–ÆÆVEVW7B…6¶VÆWFöä¶–ÆÆVEVW7DWfVçB’ÀÐ¢6¶VÆWFöä¶–æt¶–ÆÆVEVW7B…6¶VÆWFöä¶–æt¶–ÆÆVEVW7DWfVçB’ÀÐ¢W66T†VÆÂ„W66T†VÆÄWfVçB’ÀÐ¢7&÷757V7G&Ä'&–FvR„7&÷757V7G&Ä'&–FvTWfVçB’ÀÐ¢Ö–æ”vÖUvöâ„Ö–æ”vÖUvöäWfVçB’ÀÐ¢&W7väv†÷7B…&W7väv†÷7DWfVçB’ÀÐ¢¶–ÆÄ–ä†VÆÂ„¶–ÆÄ–ä†VÆÄWfVçB’ÀÐ¢†ÆÆ÷vVVäGV6´6öÆÆV7FVB„†ÆÆ÷vVVäGV6´6öÆÆV7FVDWfVçB’ÀÐ¢7V6–Å66÷&R…7V6–Å66÷&TWfVçB’ÀÐ¢FVÔÆVFW$¶–ÆÆVB…FVÔÆVFW$¶–ÆÆVDWfVçB’ÀÐ¢†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVB„†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVDWfVçB’ÀÐ¢&V6Æ7VÆFUG'V6R…&V6Æ7VÆFUG'V6TWfVçB’ÀÐ¢FVE&–ævW$6†VDFVF‚„FVE&–ævW$6†VDFVF„WfVçB’ÀÐ¢7&÷76&÷t†VÂ„7&÷76&÷t†VÄWfVçB’ÀÐ¢FÖvTÖ—F–vFVB„FÖvTÖ—F–vFVDWfVçB’ÀÐ¢–ÆöEW6†VB…–ÆöEW6†VDWfVçB’ÀÐ¢Æ–W$&æFöæVDÖF6‚…Æ–W$&æFöæVDÖF6„WfVçB’ÀÐ¢6ÄG&vÆ–æR„6ÄG&vÆ–æTWfVçB’ÀÐ¢&W7F'EF–ÖW%F–ÖR…&W7F'EF–ÖW%F–ÖTWfVçB’ÀÐ¢v–äÆ–Ö—D6†ævVB…v–äÆ–Ö—D6†ævVDWfVçB’ÀÐ¢v–åæVÅ6†÷u66÷&W2…v–åæVÅ6†÷u66÷&W4WfVçB’ÀÐ¢F÷7G&V×5&WVW7Df–æ—6†VB…F÷7G&V×5&WVW7Df–æ—6†VDWfVçB’ÀÐ¢6ö×WF—F—fU7FFT6†ævVB„6ö×WF—F—fU7FFT6†ævVDWfVçB’ÀÐ¢vÆö&Åv$FFWFFVB„vÆö&Åv$FFWFFVDWfVçB’ÀÐ¢7F÷vF6„6†ævVB…7F÷vF6„6†ævVDWfVçB’ÀÐ¢G57F÷„G57F÷WfVçB’ÀÐ¢G567&VVç6†÷B„G567&VVç6†÷DWfVçB’ÀÐ¢6†÷tÖF6…7VÖÖ'’…6†÷tÖF6…7VÖÖ'”WfVçB’ÀÐ¢W‡W&–Væ6T6†ævVB„W‡W&–Væ6T6†ævVDWfVçB’ÀÐ¢&Vv–å‡ÆW'„&Vv–å‡ÆW'WfVçB’ÀÐ¢ÖF6†Ö¶W%7FG5WFFVB„ÖF6†Ö¶W%7FG5WFFVDWfVçB’ÀÐ¢&VÖF6…f÷FUW&–öD÷fW"…&VÖF6…f÷FUW&–öD÷fW$WfVçB’ÀÐ¢&VÖF6„f–ÆVEFô7&VFR…&VÖF6„f–ÆVEFô7&VFTWfVçB’ÀÐ¢Æ–W%&VÖF6„6†ævR…Æ–W%&VÖF6„6†ævTWfVçB’ÀÐ¢–æuWFFVB…–æuWFFVDWfVçB’ÀÐ¢ÔÕ7FG5WFFVB„ÔÕ7FG5WFFVDWfVçB’ÀÐ¢Æ–W$æW‡DÖf÷FT6†ævR…Æ–W$æW‡DÖf÷FT6†ævTWfVçB’ÀÐ¢f÷FTÖ46†ævVB…f÷FTÖ46†ævVDWfVçB’ÀÐ¢&÷FôFVd6†ævVB…&÷FôFVd6†ævVDWfVçB’ÀÐ¢Æ–W$FöÖ–æF–öâ…Æ–W$FöÖ–æF–öäWfVçB’ÀÐ¢Æ–W%&ö6¶WE6µW6†VB…Æ–W%&ö6¶WE6µW6†VDWfVçB’ÀÐ¢VW7E&WVW7B…VW7E&WVW7DWfVçB’ÀÐ¢VW7E&W7öç6R…VW7E&W7öç6TWfVçB’ÀÐ¢VW7E&öw&W72…VW7E&öw&W74WfVçB’ÀÐ¢&ö¦V7F–ÆU&VÖ÷fVB…&ö¦V7F–ÆU&VÖ÷fVDWfVçB’ÀÐ¢VW7DÖFF6†ævVB…VW7DÖFF6†ævVDWfVçB’ÀÐ¢v4F÷W6VEÆ–W$–væ—FVB„v4F÷W6VEÆ–W$–væ—FVDWfVçB’ÀÐ¢VW7EGW&ä–å7FFR…VW7EGW&ä–å7FFTWfVçB’ÀÐ¢—FV×46¶æ÷vÆVFvVB„—FV×46¶æ÷vÆVFvVDWfVçB’ÀÐ¢6W$¶–ÆÆVB„6W$¶–ÆÆVDWfVçB’ÀÐ¢Ö–äÖVçU7F&–Æ—¦VB„Ö–äÖVçU7F&–Æ—¦VDWfVçB’ÀÐ¢v÷&ÆE7FGW46†ævVB…v÷&ÆE7FGW46†ævVDWfVçB’ÀÐ¢„ÅEe7FGW2„„ÅEe7FGW4WfVçB’ÀÐ¢„ÅEd6ÖW&Öâ„„ÅEd6ÖW&ÖäWfVçB’ÀÐ¢„ÅEe&æ´6ÖW&„„ÅEe&æ´6ÖW&WfVçB’ÀÐ¢„ÅEe&æ´VçF—G’„„ÅEe&æ´VçF—G”WfVçB’ÀÐ¢„ÅEdf—†VB„„ÅEdf—†VDWfVçB’ÀÐ¢„ÅEd6†6R„„ÅEd6†6TWfVçB’ÀÐ¢„ÅEdÖW76vR„„ÅEdÖW76vTWfVçB’ÀÐ¢„ÅEeF—FÆR„„ÅEeF—FÆTWfVçB’ÀÐ¢„ÅEd6†B„„ÅEd6†DWfVçB’ÀÐ¢&WÆ•7F'E&V6÷&B…&WÆ•7F'E&V6÷&DWfVçB’ÀÐ¢&WÆ•6W76–öä–æfò…&WÆ•6W76–öä–æfôWfVçB’ÀÐ¢&WÆ”VæE&V6÷&B…&WÆ”VæE&V6÷&DWfVçB’ÀÐ¢&WÆ•&WÆ—4f–Æ&ÆR…&WÆ•&WÆ—4f–Æ&ÆTWfVçB’ÀÐ¢&WÆ•6W'fW$W'&÷"…&WÆ•6W'fW$W'&÷$WfVçB’ÀÐ¢Væ¶æ÷vâ…&tvÖTWfVçB’ÀÐ§ÐÐ¢5¶6fuöGG"†fVGW&RÒ'66†VÖ"ÂFW&—fR‡66†VÖ'3£¤§6öå66†VÖ’•ÐÐ¢5¶FW&—fR„6ÆöæRÂFV'VrÂ'F–ÄWÂWÂ†6‚Â6W&–Æ—¦RÂFW6W&–Æ—¦R•ÐÐ§V"VçVÒvÖTWfVçEG—R°Ð¢5·6W&FR‡&VæÖRÒ'6W'fW%÷7vâ"•ÐÐ¢6W'fW%7vâÀÐ¢5·6W&FR‡&VæÖRÒ'6W'fW%ö6†ævVÆWfVÅöf–ÆVB"•ÐÐ¢6W'fW$6†ævTÆWfVÄf–ÆVBÀÐ¢5·6W&FR‡&VæÖRÒ'6W'fW%÷6‡WFF÷vâ"•ÐÐ¢6W'fW%6‡WFF÷vâÀÐ¢5·6W&FR‡&VæÖRÒ'6W'fW%ö7f""•ÐÐ¢6W'fW$7f"ÀÐ¢5·6W&FR‡&VæÖRÒ'6W'fW%öÖW76vR"•ÐÐ¢6W'fW$ÖW76vRÀÐ¢5·6W&FR‡&VæÖRÒ'6W'fW%öFF&â"•ÐÐ¢6W'fW$FD&âÀÐ¢5·6W&FR‡&VæÖRÒ'6W'fW%÷&VÖ÷fV&â"•ÐÐ¢6W'fW%&VÖ÷fT&âÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6öææV7B"•ÐÐ¢Æ–W$6öææV7BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6öææV7Eö6Æ–VçB"•ÐÐ¢Æ–W$6öææV7D6Æ–VçBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö–æfò"•ÐÐ¢Æ–W$–æfòÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öF—66öææV7B"•ÐÐ¢Æ–W$F—66öææV7BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö7F—fFR"•ÐÐ¢Æ–W$7F—fFRÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷6’"•ÐÐ¢Æ–W%6’ÀÐ¢5·6W&FR‡&VæÖRÒ&6Æ–VçEöF—66öææV7B"•ÐÐ¢6Æ–VçDF—66öææV7BÀÐ¢5·6W&FR‡&VæÖRÒ&6Æ–VçEö&Vv–æ6öææV7B"•ÐÐ¢6Æ–VçD&Vv–ä6öææV7BÀÐ¢5·6W&FR‡&VæÖRÒ&6Æ–VçEö6öææV7FVB"•ÐÐ¢6Æ–VçD6öææV7FVBÀÐ¢5·6W&FR‡&VæÖRÒ&6Æ–VçEögVÆÆ6öææV7B"•ÐÐ¢6Æ–VçDgVÆÄ6öææV7BÀÐ¢5·6W&FR‡&VæÖRÒ&†÷7E÷V—B"•ÐÐ¢†÷7EV—BÀÐ¢5·6W&FR‡&VæÖRÒ'FVÕö–æfò"•ÐÐ¢FVÔ–æfòÀÐ¢5·6W&FR‡&VæÖRÒ'FVÕ÷66÷&R"•ÐÐ¢FVÕ66÷&RÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•ö'&öF67EöVF–ò"•ÐÐ¢FVÕÆ”'&öF67DVF–òÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷FVÒ"•ÐÐ¢Æ–W%FVÒÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6Æ72"•ÐÐ¢Æ–W$6Æ72ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öFVF‚"•ÐÐ¢Æ–W$FVF‚ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö‡W'B"•ÐÐ¢Æ–W$‡W'BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6†B"•ÐÐ¢Æ–W$6†BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷66÷&R"•ÐÐ¢Æ–W%66÷&RÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷7vâ"•ÐÐ¢Æ–W%7vâÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷6†ö÷B"•ÐÐ¢Æ–W%6†ö÷BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷W6R"•ÐÐ¢Æ–W%W6RÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6†ævVæÖR"•ÐÐ¢Æ–W$6†ævTæÖRÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö†–çFÖW76vR"•ÐÐ¢Æ–W$†–çDÖW76vRÀÐ¢5·6W&FR‡&VæÖRÒ&&6U÷Æ–W%÷FVÆW÷'FVB"•ÐÐ¢&6UÆ–W%FVÆW÷'FVBÀÐ¢5·6W&FR‡&VæÖRÒ&vÖUö–æ—B"•ÐÐ¢vÖT–æ—BÀÐ¢5·6W&FR‡&VæÖRÒ&vÖUöæWvÖ"•ÐÐ¢vÖTæWtÖÀÐ¢5·6W&FR‡&VæÖRÒ&vÖU÷7F'B"•ÐÐ¢vÖU7F'BÀÐ¢5·6W&FR‡&VæÖRÒ&vÖUöVæB"•ÐÐ¢vÖTVæBÀÐ¢5·6W&FR‡&VæÖRÒ'&÷VæE÷7F'B"•ÐÐ¢&÷VæE7F'BÀÐ¢5·6W&FR‡&VæÖRÒ'&÷VæEöVæB"•ÐÐ¢&÷VæDVæBÀÐ¢5·6W&FR‡&VæÖRÒ&vÖUöÖW76vR"•ÐÐ¢vÖTÖW76vRÀÐ¢5·6W&FR‡&VæÖRÒ&'&Vµö'&V¶&ÆR"•ÐÐ¢'&V´'&V¶&ÆRÀÐ¢5·6W&FR‡&VæÖRÒ&'&Vµ÷&÷"•ÐÐ¢'&Vµ&÷ÀÐ¢5·6W&FR‡&VæÖRÒ&VçF—G•ö¶–ÆÆVB"•ÐÐ¢VçF—G”¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&&öçW5÷WFFVB"•ÐÐ¢&öçW5WFFVBÀÐ¢5·6W&FR‡&VæÖRÒ&6†–WfVÖVçEöWfVçB"•ÐÐ¢6†–WfVÖVçDWfVçBÀÐ¢5·6W&FR‡&VæÖRÒ&6†–WfVÖVçEö–æ7&VÖVçB"•ÐÐ¢6†–WfVÖVçD–æ7&VÖVçBÀÐ¢5·6W&FR‡&VæÖRÒ'‡—6wVå÷–6·W"•ÐÐ¢‡—6wVå–6·WÀÐ¢5·6W&FR‡&VæÖRÒ&fÆ&Uö–væ—FUöç2"•ÐÐ¢fÆ&T–væ—FTç2ÀÐ¢5·6W&FR‡&VæÖRÒ&†VÆ–6÷FW%öw&VæFU÷VçEöÖ—72"•ÐÐ¢†VÆ–6÷FW$w&VæFUVçDÖ—72ÀÐ¢5·6W&FR‡&VæÖRÒ'W6W%öFFöF÷væÆöFVB"•ÐÐ¢W6W$FFF÷væÆöFVBÀÐ¢5·6W&FR‡&VæÖRÒ'&vFöÆÅöF—76öÇfVB"•ÐÐ¢&vFöÆÄF—76öÇfVBÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGeö6†ævVEöÖöFR"•ÐÐ¢„ÅEd6†ævVDÖöFRÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGeö6†ævVE÷F&vWB"•ÐÐ¢„ÅEd6†ævVEF&vWBÀÐ¢5·6W&FR‡&VæÖRÒ'f÷FUöVæFVB"•ÐÐ¢f÷FTVæFVBÀÐ¢5·6W&FR‡&VæÖRÒ'f÷FU÷7F'FVB"•ÐÐ¢f÷FU7F'FVBÀÐ¢5·6W&FR‡&VæÖRÒ'f÷FUö6†ævVB"•ÐÐ¢f÷FT6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ'f÷FU÷76VB"•ÐÐ¢f÷FU76VBÀÐ¢5·6W&FR‡&VæÖRÒ'f÷FUöf–ÆVB"•ÐÐ¢f÷FTf–ÆVBÀÐ¢5·6W&FR‡&VæÖRÒ'f÷FUö67B"•ÐÐ¢f÷FT67BÀÐ¢5·6W&FR‡&VæÖRÒ'f÷FUö÷F–öç2"•ÐÐ¢f÷FT÷F–öç2ÀÐ¢5·6W&FR‡&VæÖRÒ'&WÆ•÷6fVB"•ÐÐ¢&WÆ•6fVBÀÐ¢5·6W&FR‡&VæÖRÒ&VçFW&VE÷W&f÷&Öæ6UöÖöFR"•ÐÐ¢VçFW&VEW&f÷&Öæ6TÖöFRÀÐ¢5·6W&FR‡&VæÖRÒ&'&÷w6U÷&WÆ—2"•ÐÐ¢'&÷w6U&WÆ—2ÀÐ¢5·6W&FR‡&VæÖRÒ'&WÆ•÷–÷WGV&U÷7FG2"•ÐÐ¢&WÆ•–÷WGV&U7FG2ÀÐ¢5·6W&FR‡&VæÖRÒ&–çfVçF÷'•÷WFFVB"•ÐÐ¢–çfVçF÷'•WFFVBÀÐ¢5·6W&FR‡&VæÖRÒ&6'E÷WFFVB"•ÐÐ¢6'EWFFVBÀÐ¢5·6W&FR‡&VæÖRÒ'7F÷&U÷&–6W6†VWE÷WFFVB"•ÐÐ¢7F÷&U&–6U6†VWEWFFVBÀÐ¢5·6W&FR‡&VæÖRÒ&V6öåö–çfVçF÷'•ö6öææV7FVB"•ÐÐ¢V6öä–çfVçF÷'”6öææV7FVBÀÐ¢5·6W&FR‡&VæÖRÒ&—FVÕ÷66†VÖö–æ—F–Æ—¦VB"•ÐÐ¢—FVÕ66†VÖ–æ—F–Æ—¦VBÀÐ¢5·6W&FR‡&VæÖRÒ&v5öæWu÷6W76–öâ"•ÐÐ¢v4æWu6W76–öâÀÐ¢5·6W&FR‡&VæÖRÒ&v5öÆ÷7E÷6W76–öâ"•ÐÐ¢v4Æ÷7E6W76–öâÀÐ¢5·6W&FR‡&VæÖRÒ&–çG&õöf–æ—6‚"•ÐÐ¢–çG&ôf–æ—6‚ÀÐ¢5·6W&FR‡&VæÖRÒ&–çG&õöæW‡F6ÖW&"•ÐÐ¢–çG&ôæW‡D6ÖW&ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6†ævV6Æ72"•ÐÐ¢Æ–W$6†ævT6Æ72ÀÐ¢5·6W&FR‡&VæÖRÒ'FeöÖ÷F–ÖU÷&VÖ–æ–ær"•ÐÐ¢FdÖF–ÖU&VÖ–æ–ærÀÐ¢5·6W&FR‡&VæÖRÒ'FeövÖUö÷fW""•ÐÐ¢FdvÖT÷fW"ÀÐ¢5·6W&FR‡&VæÖRÒ&7FeöfÆuö6GW&VB"•ÐÐ¢7FdfÆt6GW&VBÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çEö–æ—F–Æ—¦VB"•ÐÐ¢6öçG&öÅö–çD–æ—F–Æ—¦VBÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çE÷WFFV–ÖvW2"•ÐÐ¢6öçG&öÅö–çEWFFT–ÖvW2ÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çE÷WFFVÆ–÷WB"•ÐÐ¢6öçG&öÅö–çEWFFTÆ–÷WBÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çE÷WFFV6–ær"•ÐÐ¢6öçG&öÅö–çEWFFT6–ærÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çE÷WFFV÷væW""•ÐÐ¢6öçG&öÅö–çEWFFT÷væW"ÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çE÷7F'GF÷V6‚"•ÐÐ¢6öçG&öÅö–çE7F'EF÷V6‚ÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çEöVæGF÷V6‚"•ÐÐ¢6öçG&öÅö–çDVæEF÷V6‚ÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çE÷VÇ6UöVÆVÖVçB"•ÐÐ¢6öçG&öÅö–çEVÇ6TVÆVÖVçBÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çEöf¶Uö6GW&R"•ÐÐ¢6öçG&öÅö–çDf¶T6GW&RÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çEöf¶Uö6GW&Uö×VÇB"•ÐÐ¢6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W"ÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷&÷VæE÷6VÆV7FVB"•ÐÐ¢FVÕÆ•&÷VæE6VÆV7FVBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷&÷VæE÷7F'B"•ÐÐ¢FVÕÆ•&÷VæE7F'BÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷&÷VæEö7F—fR"•ÐÐ¢FVÕÆ•&÷VæD7F—fRÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷v—F–æuö&Vv–ç2"•ÐÐ¢FVÕÆ•v—F–æt&Vv–ç2ÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷v—F–æuöVæG2"•ÐÐ¢FVÕÆ•v—F–ætVæG2ÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷v—F–æuö&÷WGFöVæB"•ÐÐ¢FVÕÆ•v—F–æt&÷WEFôVæBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷&W7F'E÷&÷VæB"•ÐÐ¢FVÕÆ•&W7F'E&÷VæBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷&VG•÷&W7F'B"•ÐÐ¢FVÕÆ•&VG•&W7F'BÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷&÷VæE÷&W7F'E÷6V6öæG2"•ÐÐ¢FVÕÆ•&÷VæE&W7F'E6V6öæG2ÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷FVÕ÷&VG’"•ÐÐ¢FVÕÆ•FVÕ&VG’ÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷&÷VæE÷v–â"•ÐÐ¢FVÕÆ•&÷VæEv–âÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷WFFU÷F–ÖW""•ÐÐ¢FVÕÆ•WFFUF–ÖW"ÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷&÷VæE÷7FÆVÖFR"•ÐÐ¢FVÕÆ•&÷VæE7FÆVÖFRÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•ö÷fW'F–ÖUö&Vv–â"•ÐÐ¢FVÕÆ”÷fW'F–ÖT&Vv–âÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•ö÷fW'F–ÖUöVæB"•ÐÐ¢FVÕÆ”÷fW'F–ÖTVæBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷7VFFVæFVF…ö&Vv–â"•ÐÐ¢FVÕÆ•7VFFVäFVF„&Vv–âÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷7VFFVæFVF…öVæB"•ÐÐ¢FVÕÆ•7VFFVäFVF„VæBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•övÖUö÷fW""•ÐÐ¢FVÕÆ”vÖT÷fW"ÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•öÖ÷F–ÖU÷&VÖ–æ–ær"•ÐÐ¢FVÕÆ”ÖF–ÖU&VÖ–æ–ærÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷F–ÖW%öfÆ6‚"•ÐÐ¢FVÕÆ•F–ÖW$fÆ6‚ÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷F–ÖW%÷F–ÖUöFFVB"•ÐÐ¢FVÕÆ•F–ÖW%F–ÖTFFVBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷ö–çE÷7F'F6GW&R"•ÐÐ¢FVÕÆ•ö–çE7F'D6GW&RÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷ö–çEö6GW&VB"•ÐÐ¢FVÕÆ•ö–çD6GW&VBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷ö–çEöÆö6¶VB"•ÐÐ¢FVÕÆ•ö–çDÆö6¶VBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷ö–çE÷VæÆö6¶VB"•ÐÐ¢FVÕÆ•ö–çEVæÆö6¶VBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•ö6GW&Uö'&ö¶Vâ"•ÐÐ¢FVÕÆ”6GW&T'&ö¶VâÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•ö6GW&Uö&Æö6¶VB"•ÐÐ¢FVÕÆ”6GW&T&Æö6¶VBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•öfÆuöWfVçB"•ÐÐ¢FVÕÆ”fÆtWfVçBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷v–å÷æVÂ"•ÐÐ¢FVÕÆ•v–åæVÂÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷FVÖ&Ææ6VE÷Æ–W""•ÐÐ¢FVÕÆ•FVÔ&Ææ6VEÆ–W"ÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷6WGWöf–æ—6†VB"•ÐÐ¢FVÕÆ•6WGWf–æ—6†VBÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•öÆW'B"•ÐÐ¢FVÕÆ”ÆW'BÀÐ¢5·6W&FR‡&VæÖRÒ'G&–æ–æuö6ö×ÆWFR"•ÐÐ¢G&–æ–æt6ö×ÆWFRÀÐ¢5·6W&FR‡&VæÖRÒ'6†÷uög&VW¦WæVÂ"•ÐÐ¢6†÷tg&VW¦UæVÂÀÐ¢5·6W&FR‡&VæÖRÒ&†–FUög&VW¦WæVÂ"•ÐÐ¢†–FTg&VW¦UæVÂÀÐ¢5·6W&FR‡&VæÖRÒ&g&VW¦V6Õ÷7F'FVB"•ÐÐ¢g&VW¦T6Õ7F'FVBÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%ö6†ævWFVÒ"•ÐÐ¢Æö6ÅÆ–W$6†ævUFVÒÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%÷66÷&Uö6†ævVB"•ÐÐ¢Æö6ÅÆ–W%66÷&T6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%ö6†ævV6Æ72"•ÐÐ¢Æö6ÅÆ–W$6†ævT6Æ72ÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%÷&W7vâ"•ÐÐ¢Æö6ÅÆ–W%&W7vâÀÐ¢5·6W&FR‡&VæÖRÒ&'V–ÆF–æuö–æfõö6†ævVB"•ÐÐ¢'V–ÆF–æt–æfô6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%ö6†ævVF—6wV—6R"•ÐÐ¢Æö6ÅÆ–W$6†ævTF—6wV—6RÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö66÷VçEö6†ævVB"•ÐÐ¢Æ–W$66÷VçD6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ'7•÷F÷&W6WB"•ÐÐ¢7•F&W6WBÀÐ¢5·6W&FR‡&VæÖRÒ&fÆw7FGW5÷WFFR"•ÐÐ¢fÆu7FGW5WFFRÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷7FG5÷WFFVB"•ÐÐ¢Æ–W%7FG5WFFVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ––æuö6öÖÖVçF'’"•ÐÐ¢Æ––æt6öÖÖVçF'’ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6†&vVFWÆ÷–VB"•ÐÐ¢Æ–W$6†&vTFWÆ÷–VBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö'V–ÇFö&¦V7B"•ÐÐ¢Æ–W$'V–ÇDö&¦V7BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷Ww&FVFö&¦V7B"•ÐÐ¢Æ–W%Ww&FVDö&¦V7BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6''–ö&¦V7B"•ÐÐ¢Æ–W$6''”ö&¦V7BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öG&÷ö&¦V7B"•ÐÐ¢Æ–W$G&÷ö&¦V7BÀÐ¢5·6W&FR‡&VæÖRÒ&ö&¦V7E÷&VÖ÷fVB"•ÐÐ¢ö&¦V7E&VÖ÷fVBÀÐ¢5·6W&FR‡&VæÖRÒ&ö&¦V7EöFW7G&÷–VB"•ÐÐ¢ö&¦V7DFW7G&÷–VBÀÐ¢5·6W&FR‡&VæÖRÒ&ö&¦V7EöFWFöæFVB"•ÐÐ¢ö&¦V7DFWFöæFVBÀÐ¢5·6W&FR‡&VæÖRÒ&6†–WfVÖVçEöV&æVB"•ÐÐ¢6†–WfVÖVçDV&æVBÀÐ¢5·6W&FR‡&VæÖRÒ'7V5÷F&vWE÷WFFVB"•ÐÐ¢7V5F&vWEWFFVBÀÐ¢5·6W&FR‡&VæÖRÒ'F÷W&æÖVçE÷7FFWWFFR"•ÐÐ¢F÷W&æÖVçE7FFUWFFRÀÐ¢5·6W&FR‡&VæÖRÒ'F÷W&æÖVçEöVæ&ÆV6÷VçFF÷vâ"•ÐÐ¢F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6ÆÆVFf÷&ÖVF–2"•ÐÐ¢Æ–W$6ÆÆVDf÷$ÖVF–2ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö6¶VFf÷&&ÆÂ"•ÐÐ¢Æ–W$6¶VDf÷$&ÆÂÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%ö&V6ÖVö'6W'fW""•ÐÐ¢Æö6ÅÆ–W$&V6ÖTö'6W'fW"ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö–væ—FVEö–çb"•ÐÐ¢Æ–W$–væ—FVD–çbÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö–væ—FVB"•ÐÐ¢Æ–W$–væ—FVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öW‡F–æwV—6†VB"•ÐÐ¢Æ–W$W‡F–æwV—6†VBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷FVÆW÷'FVB"•ÐÐ¢Æ–W%FVÆW÷'FVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö†VÆVFÖVF–66ÆÂ"•ÐÐ¢Æ–W$†VÆVDÖVF–46ÆÂÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%ö6†&vW&VG’"•ÐÐ¢Æö6ÅÆ–W$6†&vU&VG’ÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%÷v–æFF÷vâ"•ÐÐ¢Æö6ÅÆ–W%v–æDF÷vâÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö–çgVÆæVB"•ÐÐ¢Æ–W$–çgVÆæVBÀÐ¢5·6W&FR‡&VæÖRÒ&W66÷'E÷7VVB"•ÐÐ¢W66÷'E7VVBÀÐ¢5·6W&FR‡&VæÖRÒ&W66÷'E÷&öw&W72"•ÐÐ¢W66÷'E&öw&W72ÀÐ¢5·6W&FR‡&VæÖRÒ&W66÷'E÷&V6VFR"•ÐÐ¢W66÷'E&V6VFRÀÐ¢5·6W&FR‡&VæÖRÒ&vÖWV•ö7F—fFVB"•ÐÐ¢vÖUT”7F—fFVBÀÐ¢5·6W&FR‡&VæÖRÒ&vÖWV•ö†–FFVâ"•ÐÐ¢vÖUT”†–FFVâÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öW66÷'E÷66÷&R"•ÐÐ¢Æ–W$W66÷'E66÷&RÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö†VÆöæ†—B"•ÐÐ¢Æ–W$†VÄöä†—BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷7FVÇ6æGf–6‚"•ÐÐ¢Æ–W%7FVÅ6æGf–6‚ÀÐ¢5·6W&FR‡&VæÖRÒ'6†÷uö6Æ75öÆ–÷WB"•ÐÐ¢6†÷t6Æ74Æ–÷WBÀÐ¢5·6W&FR‡&VæÖRÒ'6†÷u÷g5÷æVÂ"•ÐÐ¢6†÷ug5æVÂÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öFÖvVB"•ÐÐ¢Æ–W$FÖvVBÀÐ¢5·6W&FR‡&VæÖRÒ&&Væ÷Æ–W%öæ÷F–f–6F–öâ"•ÐÐ¢&VæÆ–W$æ÷F–f–6F–öâÀÐ¢5·6W&FR‡&VæÖRÒ&&VæöÖF6…öÖ‡7G&V²"•ÐÐ¢&VæÖF6„Ö…7G&V²ÀÐ¢5·6W&FR‡&VæÖRÒ&&Væ÷&÷VæE÷7F'B"•ÐÐ¢&Væ&÷VæE7F'BÀÐ¢5·6W&FR‡&VæÖRÒ&&Væ÷v–å÷æVÂ"•ÐÐ¢&Væv–åæVÂÀÐ¢5·6W&FR‡&VæÖRÒ'fU÷v–å÷æVÂ"•ÐÐ¢fUv–åæVÂÀÐ¢5·6W&FR‡&VæÖRÒ&—%öF6‚"•ÐÐ¢—$F6‚ÀÐ¢5·6W&FR‡&VæÖRÒ&ÆæFVB"•ÐÐ¢ÆæFVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öFÖvUöFöFvVB"•ÐÐ¢Æ–W$FÖvTFöFvVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷7GVææVB"•ÐÐ¢Æ–W%7GVææVBÀÐ¢5·6W&FR‡&VæÖRÒ'66÷WEöw&æE÷6ÆÒ"•ÐÐ¢66÷WDw&æE6ÆÒÀÐ¢5·6W&FR‡&VæÖRÒ'66÷WE÷6ÆÖFöÆÅöÆæFVB"•ÐÐ¢66÷WE6ÆÖFöÆÄÆæFVBÀÐ¢5·6W&FR‡&VæÖRÒ&'&÷uö–×7B"•ÐÐ¢'&÷t–×7BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö¦&FVB"•ÐÐ¢Æ–W$¦&FVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö¦&FVEöfFR"•ÐÐ¢Æ–W$¦&FVDfFRÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷6†–VÆEö&Æö6¶VB"•ÐÐ¢Æ–W%6†–VÆD&Æö6¶VBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷–ææVB"•ÐÐ¢Æ–W%–ææVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö†VÆVF'–ÖVF–2"•ÐÐ¢Æ–W$†VÆVD'”ÖVF–2ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷6VEöö&¦V7B"•ÐÐ¢Æ–W%6VDö&¦V7BÀÐ¢5·6W&FR‡&VæÖRÒ&—FVÕöf÷VæB"•ÐÐ¢—FVÔf÷VæBÀÐ¢5·6W&FR‡&VæÖRÒ'6†÷uöææ÷FF–öâ"•ÐÐ¢6†÷tææ÷FF–öâÀÐ¢5·6W&FR‡&VæÖRÒ&†–FUöææ÷FF–öâ"•ÐÐ¢†–FTææ÷FF–öâÀÐ¢5·6W&FR‡&VæÖRÒ'÷7Eö–çfVçF÷'•öÆ–6F–öâ"•ÐÐ¢÷7D–çfVçF÷'”Æ–6F–öâÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çE÷VæÆö6µ÷WFFVB"•ÐÐ¢6öçG&öÅö–çEVæÆö6µWFFVBÀÐ¢5·6W&FR‡&VæÖRÒ&FWÆ÷•ö'Vfeö&ææW""•ÐÐ¢FWÆ÷”'Vfd&ææW"ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö'Vfb"•ÐÐ¢Æ–W$'VfbÀÐ¢5·6W&FR‡&VæÖRÒ&ÖVF–5öFVF‚"•ÐÐ¢ÖVF–4FVF‚ÀÐ¢5·6W&FR‡&VæÖRÒ&÷fW'F–ÖUöær"•ÐÐ¢÷fW'F–ÖTærÀÐ¢5·6W&FR‡&VæÖRÒ'FV×5ö6†ævVB"•ÐÐ¢FV×46†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&†ÆÆ÷vVVå÷V×¶–åöw&""•ÐÐ¢†ÆÆ÷vVVåV×¶–äw&"ÀÐ¢5·6W&FR‡&VæÖRÒ'&ö6¶WEö§V×"•ÐÐ¢&ö6¶WD§V×ÀÐ¢5·6W&FR‡&VæÖRÒ'&ö6¶WEö§V×öÆæFVB"•ÐÐ¢&ö6¶WD§V×ÆæFVBÀÐ¢5·6W&FR‡&VæÖRÒ'7F–6·•ö§V×"•ÐÐ¢7F–6·”§V×ÀÐ¢5·6W&FR‡&VæÖRÒ'7F–6·•ö§V×öÆæFVB"•ÐÐ¢7F–6·”§V×ÆæFVBÀÐ¢5·6W&FR‡&VæÖRÒ'&ö6¶WG6µöÆVæ6‚"•ÐÐ¢&ö6¶WE6´ÆVæ6‚ÀÐ¢5·6W&FR‡&VæÖRÒ'&ö6¶WG6µöÆæFVB"•ÐÐ¢&ö6¶WE6´ÆæFVBÀÐ¢5·6W&FR‡&VæÖRÒ&ÖVF–5öFVfVæFVB"•ÐÐ¢ÖVF–4FVfVæFVBÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%ö†VÆVB"•ÐÐ¢Æö6ÅÆ–W$†VÆVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öFW7G&÷–VE÷—V&öÖ""•ÐÐ¢Æ–W$FW7G&÷–VE—T&öÖ"ÀÐ¢5·6W&FR‡&VæÖRÒ&ö&¦V7EöFVfÆV7FVB"•ÐÐ¢ö&¦V7DFVfÆV7FVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö×g"•ÐÐ¢Æ–W$×gÀÐ¢5·6W&FR‡&VæÖRÒ'&–E÷7våöÖö""•ÐÐ¢&–E7väÖö"ÀÐ¢5·6W&FR‡&VæÖRÒ'&–E÷7vå÷7VB"•ÐÐ¢&–E7vå7VBÀÐ¢5·6W&FR‡&VæÖRÒ&æeö&Æö6¶VB"•ÐÐ¢æd&Æö6¶VBÀÐ¢5·6W&FR‡&VæÖRÒ'F…÷G&6µ÷76VB"•ÐÐ¢F…G&6µ76VBÀÐ¢5·6W&FR‡&VæÖRÒ&çVÕö6W'5ö6†ævVB"•ÐÐ¢çVÔ6W'46†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷&VvVæW&FR"•ÐÐ¢Æ–W%&VvVæW&FRÀÐ¢5·6W&FR‡&VæÖRÒ'WFFU÷7FGW5ö—FVÒ"•ÐÐ¢WFFU7FGW4—FVÒÀÐ¢5·6W&FR‡&VæÖRÒ'7FG5÷&W6WG&÷VæB"•ÐÐ¢7FG5&W6WE&÷VæBÀÐ¢5·6W&FR‡&VæÖRÒ'66÷&W7FG5ö67V×VÆFVE÷WFFR"•ÐÐ¢66÷&U7FG467V×VÆFVEWFFRÀÐ¢5·6W&FR‡&VæÖRÒ'66÷&W7FG5ö67V×VÆFVE÷&W6WB"•ÐÐ¢66÷&U7FG467V×VÆFVE&W6WBÀÐ¢5·6W&FR‡&VæÖRÒ&6†–WfVÖVçEöV&æVEöÆö6Â"•ÐÐ¢6†–WfVÖVçDV&æVDÆö6ÂÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö†VÆVB"•ÐÐ¢Æ–W$†VÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&'V–ÆF–æuö†VÆVB"•ÐÐ¢'V–ÆF–æt†VÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&—FVÕ÷–6·W"•ÐÐ¢—FVÕ–6·WÀÐ¢5·6W&FR‡&VæÖRÒ&GVVÅ÷7FGW2"•ÐÐ¢GVVÅ7FGW2ÀÐ¢5·6W&FR‡&VæÖRÒ&f—6…öæ÷F–6R"•ÐÐ¢f—6„æ÷F–6RÀÐ¢5·6W&FR‡&VæÖRÒ&f—6…öæ÷F–6Uõö&Ò"•ÐÐ¢f—6„æ÷F–6T&ÒÀÐ¢5·6W&FR‡&VæÖRÒ'6Æöæ÷F–6R"•ÐÐ¢6Ææ÷F–6RÀÐ¢5·6W&FR‡&VæÖRÒ'F‡&÷v&ÆUö†—B"•ÐÐ¢F‡&÷v&ÆT†—BÀÐ¢5·6W&FR‡&VæÖRÒ'V×¶–åöÆ÷&E÷7VÖÖöæVB"•ÐÐ¢V×¶–äÆ÷&E7VÖÖöæVBÀÐ¢5·6W&FR‡&VæÖRÒ'V×¶–åöÆ÷&Eö¶–ÆÆVB"•ÐÐ¢V×¶–äÆ÷&D¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&ÖW&6×W5÷7VÖÖöæVB"•ÐÐ¢ÖW&6×W57VÖÖöæVBÀÐ¢5·6W&FR‡&VæÖRÒ&ÖW&6×W5ö¶–ÆÆVB"•ÐÐ¢ÖW&6×W4¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&ÖW&6×W5öW66U÷v&æ–ær"•ÐÐ¢ÖW&6×W4W66Uv&æ–ærÀÐ¢5·6W&FR‡&VæÖRÒ&ÖW&6×W5öW66VB"•ÐÐ¢ÖW&6×W4W66VBÀÐ¢5·6W&FR‡&VæÖRÒ&W–V&ÆÅö&÷75÷7VÖÖöæVB"•ÐÐ¢W–V&ÆÄ&÷757VÖÖöæVBÀÐ¢5·6W&FR‡&VæÖRÒ&W–V&ÆÅö&÷75÷7GVææVB"•ÐÐ¢W–V&ÆÄ&÷757GVææVBÀÐ¢5·6W&FR‡&VæÖRÒ&W–V&ÆÅö&÷75ö¶–ÆÆVB"•ÐÐ¢W–V&ÆÄ&÷74¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&W–V&ÆÅö&÷75ö¶–ÆÆW""•ÐÐ¢W–V&ÆÄ&÷74¶–ÆÆW"ÀÐ¢5·6W&FR‡&VæÖRÒ&W–V&ÆÅö&÷75öW66Uö–ÖÖ–æVçB"•ÐÐ¢W–V&ÆÄ&÷74W66T–ÖÖ–æVçBÀÐ¢5·6W&FR‡&VæÖRÒ&W–V&ÆÅö&÷75öW66VB"•ÐÐ¢W–V&ÆÄ&÷74W66VBÀÐ¢5·6W&FR‡&VæÖRÒ&ç5ö‡W'B"•ÐÐ¢ç4‡W'BÀÐ¢5·6W&FR‡&VæÖRÒ&6öçG&öÇö–çE÷F–ÖW%÷WFFVB"•ÐÐ¢6öçG&öÅö–çEF–ÖW%WFFVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö†–v†f—fU÷7F'B"•ÐÐ¢Æ–W$†–v„f—fU7F'BÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö†–v†f—fUö6æ6VÂ"•ÐÐ¢Æ–W$†–v„f—fT6æ6VÂÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö†–v†f—fU÷7V66W72"•ÐÐ¢Æ–W$†–v„f—fU7V66W72ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö&öçW7ö–çG2"•ÐÐ¢Æ–W$&öçW5ö–çG2ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷Ww&FVB"•ÐÐ¢Æ–W%Ww&FVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö'W–&6²"•ÐÐ¢Æ–W$'W–&6²ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷W6VE÷÷vW'Wö&÷GFÆR"•ÐÐ¢Æ–W%W6VE÷vW%W&÷GFÆRÀÐ¢5·6W&FR‡&VæÖRÒ&6‡&—7FÖ5öv–gEöw&""•ÐÐ¢6‡&—7FÖ4v–gDw&"ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö¶–ÆÆVEö6†–WfVÖVçE÷¦öæR"•ÐÐ¢Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæRÀÐ¢5·6W&FR‡&VæÖRÒ''G•÷WFFVB"•ÐÐ¢'G•WFFVBÀÐ¢5·6W&FR‡&VæÖRÒ''G•÷&Veö6†ævVB"•ÐÐ¢'G•&Vd6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ''G•ö7&—FW&–ö6†ævVB"•ÐÐ¢'G”7&—FW&–6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ''G•ö–çf—FW5ö6†ævVB"•ÐÐ¢'G”–çf—FW46†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ''G•÷VWVU÷7FFUö6†ævVB"•ÐÐ¢'G•VWVU7FFT6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ''G•ö6†B"•ÐÐ¢'G”6†BÀÐ¢5·6W&FR‡&VæÖRÒ''G•öÖVÖ&W%ö¦ö–â"•ÐÐ¢'G”ÖVÖ&W$¦ö–âÀÐ¢5·6W&FR‡&VæÖRÒ''G•öÖVÖ&W%öÆVfR"•ÐÐ¢'G”ÖVÖ&W$ÆVfRÀÐ¢5·6W&FR‡&VæÖRÒ&ÖF6…ö–çf—FW5÷WFFVB"•ÐÐ¢ÖF6„–çf—FW5WFFVBÀÐ¢5·6W&FR‡&VæÖRÒ&Æö&'•÷WFFVB"•ÐÐ¢Æö&'•WFFVBÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕöÖ—76–öå÷WFFR"•ÐÐ¢×fÔÖ—76–öåWFFRÀÐ¢5·6W&FR‡&VæÖRÒ'&V6Æ7VÆFUö†öÆ–F—2"•ÐÐ¢&V6Æ7VÆFT†öÆ–F—2ÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö7W'&Væ7•ö6†ævVB"•ÐÐ¢Æ–W$7W'&Væ7”6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&Föö×6F•÷&ö6¶WEö÷Vâ"•ÐÐ¢Föö×6F•&ö6¶WD÷VâÀÐ¢5·6W&FR‡&VæÖRÒ'&VÖ÷fUöæVÖW6—5÷&VÆF–öç6†—2"•ÐÐ¢&VÖ÷fTæVÖW6—5&VÆF–öç6†—2ÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕö7&VF—F&öçW5÷vfR"•ÐÐ¢×fÔ7&VF—D&öçW5vfRÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕö7&VF—F&öçW5öÆÂ"•ÐÐ¢×fÔ7&VF—D&öçW4ÆÂÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕö7&VF—F&öçW5öÆÅöGfæ6VB"•ÐÐ¢×fÔ7&VF—D&öçW4ÆÄGfæ6VBÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷V–6µ÷6VçG'•÷Ww&FR"•ÐÐ¢×fÕV–6µ6VçG'•Ww&FRÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷FæµöFW7G&÷–VEö'•÷Æ–W'2"•ÐÐ¢×fÕFæ´FW7G&÷–VD'•Æ–W'2ÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕö¶–ÆÅ÷&ö&÷EöFVÆ—fW&–æuö&öÖ""•ÐÐ¢×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ"ÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷–6·Wö7W'&Væ7’"•ÐÐ¢×fÕ–6·W7W'&Væ7’ÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕö&öÖ%ö6'&–W%ö¶–ÆÆVB"•ÐÐ¢×fÔ&öÖ$6'&–W$¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷6VçG'–'W7FW%öFWFöæFR"•ÐÐ¢×fÕ6VçG'”'W7FW$FWFöæFRÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷66÷WEöÖ&¶VEöf÷%öFVF‚"•ÐÐ¢×fÕ66÷WDÖ&¶VDf÷$FVF‚ÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕöÖVF–5÷÷vW'W÷6†&VB"•ÐÐ¢×fÔÖVF–5÷vW%W6†&VBÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕö&Vv–å÷vfR"•ÐÐ¢×fÔ&Vv–åvfRÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷vfUö6ö×ÆWFR"•ÐÐ¢×fÕvfT6ö×ÆWFRÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕöÖ—76–öåö6ö×ÆWFR"•ÐÐ¢×fÔÖ—76–öä6ö×ÆWFRÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕö&öÖ%÷&W6WEö'•÷Æ–W""•ÐÐ¢×fÔ&öÖ%&W6WD'•Æ–W"ÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕö&öÖ%öÆ&Õ÷G&–vvW&VB"•ÐÐ¢×fÔ&öÖ$Æ&ÕG&–vvW&VBÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕö&öÖ%öFWÆ÷•÷&W6WEö'•÷Æ–W""•ÐÐ¢×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W"ÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷vfUöf–ÆVB"•ÐÐ¢×fÕvfTf–ÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷&W6WE÷7FG2"•ÐÐ¢×fÕ&W6WE7FG2ÀÐ¢5·6W&FR‡&VæÖRÒ&FÖvU÷&W6—7FVB"•ÐÐ¢FÖvU&W6—7FVBÀÐ¢5·6W&FR‡&VæÖRÒ'&Wf—fU÷Æ–W%öæ÷F–g’"•ÐÐ¢&Wf—fUÆ–W$æ÷F–g’ÀÐ¢5·6W&FR‡&VæÖRÒ'&Wf—fU÷Æ–W%÷7F÷VB"•ÐÐ¢&Wf—fUÆ–W%7F÷VBÀÐ¢5·6W&FR‡&VæÖRÒ'&Wf—fU÷Æ–W%ö6ö×ÆWFR"•ÐÐ¢&Wf—fUÆ–W$6ö×ÆWFRÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷GW&æVE÷Fõöv†÷7B"•ÐÐ¢Æ–W%GW&æVEFôv†÷7BÀÐ¢5·6W&FR‡&VæÖRÒ&ÖVF–wVå÷6†–VÆEö&Æö6¶VEöFÖvR"•ÐÐ¢ÖVF–wVå6†–VÆD&Æö6¶VDFÖvRÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕöGe÷vfUö6ö×ÆWFUöæõövFW2"•ÐÐ¢×fÔGevfT6ö×ÆWFTæôvFW2ÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷6æ—W%ö†VG6†÷Eö7W'&Væ7’"•ÐÐ¢×fÕ6æ—W$†VG6†÷D7W'&Væ7’ÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕöÖææ†GFå÷—B"•ÐÐ¢×fÔÖææ†GFå—BÀÐ¢5·6W&FR‡&VæÖRÒ&fÆuö6'&–VEö–åöFWFV7F–öå÷¦öæR"•ÐÐ¢fÆt6'&–VD–äFWFV7F–öå¦öæRÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕöGe÷vfUö¶–ÆÆVE÷7GVå÷&F–ò"•ÐÐ¢×fÔGevfT¶–ÆÆVE7GVå&F–òÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öF—&V7F†—E÷7GVâ"•ÐÐ¢Æ–W$F—&V7D†—E7GVâÀÐ¢5·6W&FR‡&VæÖRÒ&×fÕ÷6VçG'–'W7FW%ö¶–ÆÆVB"•ÐÐ¢×fÕ6VçG'”'W7FW$¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ'Ww&FW5öf–ÆUö6†ævVB"•ÐÐ¢Ww&FW4f–ÆT6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ'&E÷FVÕ÷ö–çG5ö6†ævVB"•ÐÐ¢&EFVÕö–çG46†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ'&E÷'VÆW5÷7FFUö6†ævVB"•ÐÐ¢&E'VÆW57FFT6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ'&E÷&ö&÷Eö¶–ÆÆVB"•ÐÐ¢&E&ö&÷D¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ'&E÷&ö&÷Eö–×7B"•ÐÐ¢&E&ö&÷D–×7BÀÐ¢5·6W&FR‡&VæÖRÒ'FV×Æ•÷&U÷&÷VæE÷F–ÖUöÆVgB"•ÐÐ¢FVÕÆ•&U&÷VæEF–ÖTÆVgBÀÐ¢5·6W&FR‡&VæÖRÒ'&6‡WFUöFWÆ÷’"•ÐÐ¢&6‡WFTFWÆ÷’ÀÐ¢5·6W&FR‡&VæÖRÒ'&6‡WFUö†öÇ7FW""•ÐÐ¢&6‡WFT†öÇ7FW"ÀÐ¢5·6W&FR‡&VæÖRÒ&¶–ÆÅ÷&Vf–ÆÇ5öÖWFW""•ÐÐ¢¶–ÆÅ&Vf–ÆÇ4ÖWFW"ÀÐ¢5·6W&FR‡&VæÖRÒ''5÷FVçEöWfVçB"•ÐÐ¢'5FVçDWfVçBÀÐ¢5·6W&FR‡&VæÖRÒ&6öævö¶–ÆÂ"•ÐÐ¢6öæv¶–ÆÂÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö–æ—F–Å÷7vâ"•ÐÐ¢Æ–W$–æ—F–Å7vâÀÐ¢5·6W&FR‡&VæÖRÒ&6ö×WF—F—fU÷f–7F÷'’"•ÐÐ¢6ö×WF—F—fUf–7F÷'’ÀÐ¢5·6W&FR‡&VæÖRÒ&6ö×WF—F—fU÷7FG5÷WFFR"•ÐÐ¢6ö×WF—F—fU7FG5WFFRÀÐ¢5·6W&FR‡&VæÖRÒ&Ö–æ–vÖU÷v–â"•ÐÐ¢Ö–æ”vÖUv–âÀÐ¢5·6W&FR‡&VæÖRÒ'6VçG'•ööåövõö7F—fR"•ÐÐ¢6VçG'”öävô7F—fRÀÐ¢5·6W&FR‡&VæÖRÒ&GV6µ÷‡öÆWfVÅ÷W"•ÐÐ¢GV6µ‡ÆWfVÅWÀÐ¢5·6W&FR‡&VæÖRÒ'VW7FÆöuö÷VæVB"•ÐÐ¢VW7DÆöt÷VæVBÀÐ¢5·6W&FR‡&VæÖRÒ'66†VÖ÷WFFVB"•ÐÐ¢66†VÖWFFVBÀÐ¢5·6W&FR‡&VæÖRÒ&Æö6ÇÆ–W%÷–6·W÷vVöâ"•ÐÐ¢Æö6ÅÆ–W%–6·WvVöâÀÐ¢5·6W&FR‡&VæÖRÒ'&E÷Æ–W%÷66÷&U÷ö–çG2"•ÐÐ¢&EÆ–W%66÷&Uö–çG2ÀÐ¢5·6W&FR‡&VæÖRÒ&FVÖöÖåöFWE÷7F–6¶–W2"•ÐÐ¢FVÖöÖäFWE7F–6¶–W2ÀÐ¢5·6W&FR‡&VæÖRÒ'VW7Eöö&¦V7F—fUö6ö×ÆWFVB"•ÐÐ¢VW7Dö&¦V7F—fT6ö×ÆWFVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷66÷&Uö6†ævVB"•ÐÐ¢Æ–W%66÷&T6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&¶–ÆÆVEö6–æu÷Æ–W""•ÐÐ¢¶–ÆÆVD6–æuÆ–W"ÀÐ¢5·6W&FR‡&VæÖRÒ&Vçf—&öæÖVçFÅöFVF‚"•ÐÐ¢Vçf—&öæÖVçFÄFVF‚ÀÐ¢5·6W&FR‡&VæÖRÒ'&ö¦V7F–ÆUöF—&V7Eö†—B"•ÐÐ¢&ö¦V7F–ÆTF—&V7D†—BÀÐ¢5·6W&FR‡&VæÖRÒ'75övWB"•ÐÐ¢74vWBÀÐ¢5·6W&FR‡&VæÖRÒ'75÷66÷&R"•ÐÐ¢7566÷&RÀÐ¢5·6W&FR‡&VæÖRÒ'75ög&VR"•ÐÐ¢74g&VRÀÐ¢5·6W&FR‡&VæÖRÒ'75÷75ö6Vv‡B"•ÐÐ¢75746Vv‡BÀÐ¢5·6W&FR‡&VæÖRÒ'75ö&ÆÅ÷7FöÆVâ"•ÐÐ¢74&ÆÅ7FöÆVâÀÐ¢5·6W&FR‡&VæÖRÒ'75ö&ÆÅö&Æö6¶VB"•ÐÐ¢74&ÆÄ&Æö6¶VBÀÐ¢5·6W&FR‡&VæÖRÒ&FÖvU÷&WfVçFVB"•ÐÐ¢FÖvU&WfVçFVBÀÐ¢5·6W&FR‡&VæÖRÒ&†ÆÆ÷vVVåö&÷75ö¶–ÆÆVB"•ÐÐ¢†ÆÆ÷vVVä&÷74¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&W66VEöÆö÷Eö—6ÆæB"•ÐÐ¢W66VDÆö÷D—6ÆæBÀÐ¢5·6W&FR‡&VæÖRÒ'FvvVE÷Æ–W%ö5ö—B"•ÐÐ¢FvvVEÆ–W$4—BÀÐ¢5·6W&FR‡&VæÖRÒ&ÖW&6×W5÷7GVææVB"•ÐÐ¢ÖW&6×W57GVææVBÀÐ¢5·6W&FR‡&VæÖRÒ&ÖW&6×W5÷&÷öf÷VæB"•ÐÐ¢ÖW&6×W5&÷f÷VæBÀÐ¢5·6W&FR‡&VæÖRÒ&†ÆÆ÷vVVå÷6¶VÆWFöåö¶–ÆÆVB"•ÐÐ¢†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ'6¶VÆWFöåö¶–ÆÆVE÷VW7B"•ÐÐ¢6¶VÆWFöä¶–ÆÆVEVW7BÀÐ¢5·6W&FR‡&VæÖRÒ'6¶VÆWFöåö¶–æuö¶–ÆÆVE÷VW7B"•ÐÐ¢6¶VÆWFöä¶–æt¶–ÆÆVEVW7BÀÐ¢5·6W&FR‡&VæÖRÒ&W66Uö†VÆÂ"•ÐÐ¢W66T†VÆÂÀÐ¢5·6W&FR‡&VæÖRÒ&7&÷75÷7V7G&Åö'&–FvR"•ÐÐ¢7&÷757V7G&Ä'&–FvRÀÐ¢5·6W&FR‡&VæÖRÒ&Ö–æ–vÖU÷vöâ"•ÐÐ¢Ö–æ”vÖUvöâÀÐ¢5·6W&FR‡&VæÖRÒ'&W7våöv†÷7B"•ÐÐ¢&W7väv†÷7BÀÐ¢5·6W&FR‡&VæÖRÒ&¶–ÆÅö–åö†VÆÂ"•ÐÐ¢¶–ÆÄ–ä†VÆÂÀÐ¢5·6W&FR‡&VæÖRÒ&†ÆÆ÷vVVåöGV6µö6öÆÆV7FVB"•ÐÐ¢†ÆÆ÷vVVäGV6´6öÆÆV7FVBÀÐ¢5·6W&FR‡&VæÖRÒ'7V6–Å÷66÷&R"•ÐÐ¢7V6–Å66÷&RÀÐ¢5·6W&FR‡&VæÖRÒ'FVÕöÆVFW%ö¶–ÆÆVB"•ÐÐ¢FVÔÆVFW$¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&†ÆÆ÷vVVå÷6÷VÅö6öÆÆV7FVB"•ÐÐ¢†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVBÀÐ¢5·6W&FR‡&VæÖRÒ'&V6Æ7VÆFU÷G'V6R"•ÐÐ¢&V6Æ7VÆFUG'V6RÀÐ¢5·6W&FR‡&VæÖRÒ&FVG&–ævW%ö6†VEöFVF‚"•ÐÐ¢FVE&–ævW$6†VDFVF‚ÀÐ¢5·6W&FR‡&VæÖRÒ&7&÷76&÷uö†VÂ"•ÐÐ¢7&÷76&÷t†VÂÀÐ¢5·6W&FR‡&VæÖRÒ&FÖvUöÖ—F–vFVB"•ÐÐ¢FÖvTÖ—F–vFVBÀÐ¢5·6W&FR‡&VæÖRÒ'–ÆöE÷W6†VB"•ÐÐ¢–ÆöEW6†VBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%ö&æFöæVEöÖF6‚"•ÐÐ¢Æ–W$&æFöæVDÖF6‚ÀÐ¢5·6W&FR‡&VæÖRÒ&6ÅöG&vÆ–æR"•ÐÐ¢6ÄG&vÆ–æRÀÐ¢5·6W&FR‡&VæÖRÒ'&W7F'E÷F–ÖW%÷F–ÖR"•ÐÐ¢&W7F'EF–ÖW%F–ÖRÀÐ¢5·6W&FR‡&VæÖRÒ'v–æÆ–Ö—Eö6†ævVB"•ÐÐ¢v–äÆ–Ö—D6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ'v–çæVÅ÷6†÷u÷66÷&W2"•ÐÐ¢v–åæVÅ6†÷u66÷&W2ÀÐ¢5·6W&FR‡&VæÖRÒ'F÷÷7G&V×5÷&WVW7Eöf–æ—6†VB"•ÐÐ¢F÷7G&V×5&WVW7Df–æ—6†VBÀÐ¢5·6W&FR‡&VæÖRÒ&6ö×WF—F—fU÷7FFUö6†ævVB"•ÐÐ¢6ö×WF—F—fU7FFT6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&vÆö&Å÷v%öFF÷WFFVB"•ÐÐ¢vÆö&Åv$FFWFFVBÀÐ¢5·6W&FR‡&VæÖRÒ'7F÷÷vF6…ö6†ævVB"•ÐÐ¢7F÷vF6„6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&G5÷7F÷"•ÐÐ¢G57F÷ÀÐ¢5·6W&FR‡&VæÖRÒ&G5÷67&VVç6†÷B"•ÐÐ¢G567&VVç6†÷BÀÐ¢5·6W&FR‡&VæÖRÒ'6†÷uöÖF6…÷7VÖÖ'’"•ÐÐ¢6†÷tÖF6…7VÖÖ'’ÀÐ¢5·6W&FR‡&VæÖRÒ&W‡W&–Væ6Uö6†ævVB"•ÐÐ¢W‡W&–Væ6T6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&&Vv–å÷‡öÆW'"•ÐÐ¢&Vv–å‡ÆW'ÀÐ¢5·6W&FR‡&VæÖRÒ&ÖF6†Ö¶W%÷7FG5÷WFFVB"•ÐÐ¢ÖF6†Ö¶W%7FG5WFFVBÀÐ¢5·6W&FR‡&VæÖRÒ'&VÖF6…÷f÷FU÷W&–öEö÷fW""•ÐÐ¢&VÖF6…f÷FUW&–öD÷fW"ÀÐ¢5·6W&FR‡&VæÖRÒ'&VÖF6…öf–ÆVE÷Fõö7&VFR"•ÐÐ¢&VÖF6„f–ÆVEFô7&VFRÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷&VÖF6…ö6†ævR"•ÐÐ¢Æ–W%&VÖF6„6†ævRÀÐ¢5·6W&FR‡&VæÖRÒ'–æu÷WFFVB"•ÐÐ¢–æuWFFVBÀÐ¢5·6W&FR‡&VæÖRÒ&Ö×7FG5÷WFFVB"•ÐÐ¢ÔÕ7FG5WFFVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öæW‡EöÖ÷f÷FUö6†ævR"•ÐÐ¢Æ–W$æW‡DÖf÷FT6†ævRÀÐ¢5·6W&FR‡&VæÖRÒ'f÷FUöÖ5ö6†ævVB"•ÐÐ¢f÷FTÖ46†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ'&÷FõöFVeö6†ævVB"•ÐÐ¢&÷FôFVd6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%öFöÖ–æF–öâ"•ÐÐ¢Æ–W$FöÖ–æF–öâÀÐ¢5·6W&FR‡&VæÖRÒ'Æ–W%÷&ö6¶WG6µ÷W6†VB"•ÐÐ¢Æ–W%&ö6¶WE6µW6†VBÀÐ¢5·6W&FR‡&VæÖRÒ'VW7E÷&WVW7B"•ÐÐ¢VW7E&WVW7BÀÐ¢5·6W&FR‡&VæÖRÒ'VW7E÷&W7öç6R"•ÐÐ¢VW7E&W7öç6RÀÐ¢5·6W&FR‡&VæÖRÒ'VW7E÷&öw&W72"•ÐÐ¢VW7E&öw&W72ÀÐ¢5·6W&FR‡&VæÖRÒ'&ö¦V7F–ÆU÷&VÖ÷fVB"•ÐÐ¢&ö¦V7F–ÆU&VÖ÷fVBÀÐ¢5·6W&FR‡&VæÖRÒ'VW7EöÖöFFö6†ævVB"•ÐÐ¢VW7DÖFF6†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&v5öF÷W6VE÷Æ–W%ö–væ—FVB"•ÐÐ¢v4F÷W6VEÆ–W$–væ—FVBÀÐ¢5·6W&FR‡&VæÖRÒ'VW7E÷GW&åö–å÷7FFR"•ÐÐ¢VW7EGW&ä–å7FFRÀÐ¢5·6W&FR‡&VæÖRÒ&—FV×5ö6¶æ÷vÆVFvVB"•ÐÐ¢—FV×46¶æ÷vÆVFvVBÀÐ¢5·6W&FR‡&VæÖRÒ&6W%ö¶–ÆÆVB"•ÐÐ¢6W$¶–ÆÆVBÀÐ¢5·6W&FR‡&VæÖRÒ&Ö–æÖVçU÷7F&–Æ—¦VB"•ÐÐ¢Ö–äÖVçU7F&–Æ—¦VBÀÐ¢5·6W&FR‡&VæÖRÒ'v÷&ÆE÷7FGW5ö6†ævVB"•ÐÐ¢v÷&ÆE7FGW46†ævVBÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGe÷7FGW2"•ÐÐ¢„ÅEe7FGW2ÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGeö6ÖW&Öâ"•ÐÐ¢„ÅEd6ÖW&ÖâÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGe÷&æµö6ÖW&"•ÐÐ¢„ÅEe&æ´6ÖW&ÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGe÷&æµöVçF—G’"•ÐÐ¢„ÅEe&æ´VçF—G’ÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGeöf—†VB"•ÐÐ¢„ÅEdf—†VBÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGeö6†6R"•ÐÐ¢„ÅEd6†6RÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGeöÖW76vR"•ÐÐ¢„ÅEdÖW76vRÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGe÷F—FÆR"•ÐÐ¢„ÅEeF—FÆRÀÐ¢5·6W&FR‡&VæÖRÒ&†ÇGeö6†B"•ÐÐ¢„ÅEd6†BÀÐ¢5·6W&FR‡&VæÖRÒ'&WÆ•÷7F'G&V6÷&B"•ÐÐ¢&WÆ•7F'E&V6÷&BÀÐ¢5·6W&FR‡&VæÖRÒ'&WÆ•÷6W76–öæ–æfò"•ÐÐ¢&WÆ•6W76–öä–æfòÀÐ¢5·6W&FR‡&VæÖRÒ'&WÆ•öVæG&V6÷&B"•ÐÐ¢&WÆ”VæE&V6÷&BÀÐ¢5·6W&FR‡&VæÖRÒ'&WÆ•÷&WÆ—6f–Æ&ÆR"•ÐÐ¢&WÆ•&WÆ—4f–Æ&ÆRÀÐ¢5·6W&FR‡&VæÖRÒ'&WÆ•÷6W'fW&W'&÷""•ÐÐ¢&WÆ•6W'fW$W'&÷"ÀÐ¢Væ¶æ÷vâ…7G&–ær’ÀÐ§ÐÐ¦–×ÂvÖTWfVçEG—R°Ð¢V"fâg&öÕ÷G—UöæÖR†æÖS¢g7G"’Óâ6VÆb°Ð¢ÖF6‚æÖR°Ð¢'6W'fW%÷7vâ"ÓâvÖTWfVçEG—S£¥6W'fW%7vâÀÐ¢'6W'fW%ö6†ævVÆWfVÅöf–ÆVB"ÓâvÖTWfVçEG—S£¥6W'fW$6†ævTÆWfVÄf–ÆVBÀÐ¢'6W'fW%÷6‡WFF÷vâ"ÓâvÖTWfVçEG—S£¥6W'fW%6‡WFF÷vâÀÐ¢'6W'fW%ö7f""ÓâvÖTWfVçEG—S£¥6W'fW$7f"ÀÐ¢'6W'fW%öÖW76vR"ÓâvÖTWfVçEG—S£¥6W'fW$ÖW76vRÀÐ¢'6W'fW%öFF&â"ÓâvÖTWfVçEG—S£¥6W'fW$FD&âÀÐ¢'6W'fW%÷&VÖ÷fV&â"ÓâvÖTWfVçEG—S£¥6W'fW%&VÖ÷fT&âÀÐ¢'Æ–W%ö6öææV7B"ÓâvÖTWfVçEG—S£¥Æ–W$6öææV7BÀÐ¢'Æ–W%ö6öææV7Eö6Æ–VçB"ÓâvÖTWfVçEG—S£¥Æ–W$6öææV7D6Æ–VçBÀÐ¢'Æ–W%ö–æfò"ÓâvÖTWfVçEG—S£¥Æ–W$–æfòÀÐ¢'Æ–W%öF—66öææV7B"ÓâvÖTWfVçEG—S£¥Æ–W$F—66öææV7BÀÐ¢'Æ–W%ö7F—fFR"ÓâvÖTWfVçEG—S£¥Æ–W$7F—fFRÀÐ¢'Æ–W%÷6’"ÓâvÖTWfVçEG—S£¥Æ–W%6’ÀÐ¢&6Æ–VçEöF—66öææV7B"ÓâvÖTWfVçEG—S£¤6Æ–VçDF—66öææV7BÀÐ¢&6Æ–VçEö&Vv–æ6öææV7B"ÓâvÖTWfVçEG—S£¤6Æ–VçD&Vv–ä6öææV7BÀÐ¢&6Æ–VçEö6öææV7FVB"ÓâvÖTWfVçEG—S£¤6Æ–VçD6öææV7FVBÀÐ¢&6Æ–VçEögVÆÆ6öææV7B"ÓâvÖTWfVçEG—S£¤6Æ–VçDgVÆÄ6öææV7BÀÐ¢&†÷7E÷V—B"ÓâvÖTWfVçEG—S£¤†÷7EV—BÀÐ¢'FVÕö–æfò"ÓâvÖTWfVçEG—S£¥FVÔ–æfòÀÐ¢'FVÕ÷66÷&R"ÓâvÖTWfVçEG—S£¥FVÕ66÷&RÀÐ¢'FV×Æ•ö'&öF67EöVF–ò"ÓâvÖTWfVçEG—S£¥FVÕÆ”'&öF67DVF–òÀÐ¢'Æ–W%÷FVÒ"ÓâvÖTWfVçEG—S£¥Æ–W%FVÒÀÐ¢'Æ–W%ö6Æ72"ÓâvÖTWfVçEG—S£¥Æ–W$6Æ72ÀÐ¢'Æ–W%öFVF‚"ÓâvÖTWfVçEG—S£¥Æ–W$FVF‚ÀÐ¢'Æ–W%ö‡W'B"ÓâvÖTWfVçEG—S£¥Æ–W$‡W'BÀÐ¢'Æ–W%ö6†B"ÓâvÖTWfVçEG—S£¥Æ–W$6†BÀÐ¢'Æ–W%÷66÷&R"ÓâvÖTWfVçEG—S£¥Æ–W%66÷&RÀÐ¢'Æ–W%÷7vâ"ÓâvÖTWfVçEG—S£¥Æ–W%7vâÀÐ¢'Æ–W%÷6†ö÷B"ÓâvÖTWfVçEG—S£¥Æ–W%6†ö÷BÀÐ¢'Æ–W%÷W6R"ÓâvÖTWfVçEG—S£¥Æ–W%W6RÀÐ¢'Æ–W%ö6†ævVæÖR"ÓâvÖTWfVçEG—S£¥Æ–W$6†ævTæÖRÀÐ¢'Æ–W%ö†–çFÖW76vR"ÓâvÖTWfVçEG—S£¥Æ–W$†–çDÖW76vRÀÐ¢&&6U÷Æ–W%÷FVÆW÷'FVB"ÓâvÖTWfVçEG—S£¤&6UÆ–W%FVÆW÷'FVBÀÐ¢&vÖUö–æ—B"ÓâvÖTWfVçEG—S£¤vÖT–æ—BÀÐ¢&vÖUöæWvÖ"ÓâvÖTWfVçEG—S£¤vÖTæWtÖÀÐ¢&vÖU÷7F'B"ÓâvÖTWfVçEG—S£¤vÖU7F'BÀÐ¢&vÖUöVæB"ÓâvÖTWfVçEG—S£¤vÖTVæBÀÐ¢'&÷VæE÷7F'B"ÓâvÖTWfVçEG—S£¥&÷VæE7F'BÀÐ¢'&÷VæEöVæB"ÓâvÖTWfVçEG—S£¥&÷VæDVæBÀÐ¢&vÖUöÖW76vR"ÓâvÖTWfVçEG—S£¤vÖTÖW76vRÀÐ¢&'&Vµö'&V¶&ÆR"ÓâvÖTWfVçEG—S£¤'&V´'&V¶&ÆRÀÐ¢&'&Vµ÷&÷"ÓâvÖTWfVçEG—S£¤'&Vµ&÷ÀÐ¢&VçF—G•ö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¤VçF—G”¶–ÆÆVBÀÐ¢&&öçW5÷WFFVB"ÓâvÖTWfVçEG—S£¤&öçW5WFFVBÀÐ¢&6†–WfVÖVçEöWfVçB"ÓâvÖTWfVçEG—S£¤6†–WfVÖVçDWfVçBÀÐ¢&6†–WfVÖVçEö–æ7&VÖVçB"ÓâvÖTWfVçEG—S£¤6†–WfVÖVçD–æ7&VÖVçBÀÐ¢'‡—6wVå÷–6·W"ÓâvÖTWfVçEG—S£¥‡—6wVå–6·WÀÐ¢&fÆ&Uö–væ—FUöç2"ÓâvÖTWfVçEG—S£¤fÆ&T–væ—FTç2ÀÐ¢&†VÆ–6÷FW%öw&VæFU÷VçEöÖ—72"ÓâvÖTWfVçEG—S£¤†VÆ–6÷FW$w&VæFUVçDÖ—72ÀÐ¢'W6W%öFFöF÷væÆöFVB"ÓâvÖTWfVçEG—S£¥W6W$FFF÷væÆöFVBÀÐ¢'&vFöÆÅöF—76öÇfVB"ÓâvÖTWfVçEG—S£¥&vFöÆÄF—76öÇfVBÀÐ¢&†ÇGeö6†ævVEöÖöFR"ÓâvÖTWfVçEG—S£¤„ÅEd6†ævVDÖöFRÀÐ¢&†ÇGeö6†ævVE÷F&vWB"ÓâvÖTWfVçEG—S£¤„ÅEd6†ævVEF&vWBÀÐ¢'f÷FUöVæFVB"ÓâvÖTWfVçEG—S£¥f÷FTVæFVBÀÐ¢'f÷FU÷7F'FVB"ÓâvÖTWfVçEG—S£¥f÷FU7F'FVBÀÐ¢'f÷FUö6†ævVB"ÓâvÖTWfVçEG—S£¥f÷FT6†ævVBÀÐ¢'f÷FU÷76VB"ÓâvÖTWfVçEG—S£¥f÷FU76VBÀÐ¢'f÷FUöf–ÆVB"ÓâvÖTWfVçEG—S£¥f÷FTf–ÆVBÀÐ¢'f÷FUö67B"ÓâvÖTWfVçEG—S£¥f÷FT67BÀÐ¢'f÷FUö÷F–öç2"ÓâvÖTWfVçEG—S£¥f÷FT÷F–öç2ÀÐ¢'&WÆ•÷6fVB"ÓâvÖTWfVçEG—S£¥&WÆ•6fVBÀÐ¢&VçFW&VE÷W&f÷&Öæ6UöÖöFR"ÓâvÖTWfVçEG—S£¤VçFW&VEW&f÷&Öæ6TÖöFRÀÐ¢&'&÷w6U÷&WÆ—2"ÓâvÖTWfVçEG—S£¤'&÷w6U&WÆ—2ÀÐ¢'&WÆ•÷–÷WGV&U÷7FG2"ÓâvÖTWfVçEG—S£¥&WÆ•–÷WGV&U7FG2ÀÐ¢&–çfVçF÷'•÷WFFVB"ÓâvÖTWfVçEG—S£¤–çfVçF÷'•WFFVBÀÐ¢&6'E÷WFFVB"ÓâvÖTWfVçEG—S£¤6'EWFFVBÀÐ¢'7F÷&U÷&–6W6†VWE÷WFFVB"ÓâvÖTWfVçEG—S£¥7F÷&U&–6U6†VWEWFFVBÀÐ¢&V6öåö–çfVçF÷'•ö6öææV7FVB"ÓâvÖTWfVçEG—S£¤V6öä–çfVçF÷'”6öææV7FVBÀÐ¢&—FVÕ÷66†VÖö–æ—F–Æ—¦VB"ÓâvÖTWfVçEG—S£¤—FVÕ66†VÖ–æ—F–Æ—¦VBÀÐ¢&v5öæWu÷6W76–öâ"ÓâvÖTWfVçEG—S£¤v4æWu6W76–öâÀÐ¢&v5öÆ÷7E÷6W76–öâ"ÓâvÖTWfVçEG—S£¤v4Æ÷7E6W76–öâÀÐ¢&–çG&õöf–æ—6‚"ÓâvÖTWfVçEG—S£¤–çG&ôf–æ—6‚ÀÐ¢&–çG&õöæW‡F6ÖW&"ÓâvÖTWfVçEG—S£¤–çG&ôæW‡D6ÖW&ÀÐ¢'Æ–W%ö6†ævV6Æ72"ÓâvÖTWfVçEG—S£¥Æ–W$6†ævT6Æ72ÀÐ¢'FeöÖ÷F–ÖU÷&VÖ–æ–ær"ÓâvÖTWfVçEG—S£¥FdÖF–ÖU&VÖ–æ–ærÀÐ¢'FeövÖUö÷fW""ÓâvÖTWfVçEG—S£¥FdvÖT÷fW"ÀÐ¢&7FeöfÆuö6GW&VB"ÓâvÖTWfVçEG—S£¤7FdfÆt6GW&VBÀÐ¢&6öçG&öÇö–çEö–æ—F–Æ—¦VB"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çD–æ—F–Æ—¦VBÀÐ¢&6öçG&öÇö–çE÷WFFV–ÖvW2"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT–ÖvW2ÀÐ¢&6öçG&öÇö–çE÷WFFVÆ–÷WB"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEWFFTÆ–÷WBÀÐ¢&6öçG&öÇö–çE÷WFFV6–ær"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT6–ærÀÐ¢&6öçG&öÇö–çE÷WFFV÷væW""ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT÷væW"ÀÐ¢&6öçG&öÇö–çE÷7F'GF÷V6‚"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çE7F'EF÷V6‚ÀÐ¢&6öçG&öÇö–çEöVæGF÷V6‚"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çDVæEF÷V6‚ÀÐ¢&6öçG&öÇö–çE÷VÇ6UöVÆVÖVçB"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEVÇ6TVÆVÖVçBÀÐ¢&6öçG&öÇö–çEöf¶Uö6GW&R"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çDf¶T6GW&RÀÐ¢&6öçG&öÇö–çEöf¶Uö6GW&Uö×VÇB"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W"ÀÐ¢'FV×Æ•÷&÷VæE÷6VÆV7FVB"ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæE6VÆV7FVBÀÐ¢'FV×Æ•÷&÷VæE÷7F'B"ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæE7F'BÀÐ¢'FV×Æ•÷&÷VæEö7F—fR"ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæD7F—fRÀÐ¢'FV×Æ•÷v—F–æuö&Vv–ç2"ÓâvÖTWfVçEG—S£¥FVÕÆ•v—F–æt&Vv–ç2ÀÐ¢'FV×Æ•÷v—F–æuöVæG2"ÓâvÖTWfVçEG—S£¥FVÕÆ•v—F–ætVæG2ÀÐ¢'FV×Æ•÷v—F–æuö&÷WGFöVæB"ÓâvÖTWfVçEG—S£¥FVÕÆ•v—F–æt&÷WEFôVæBÀÐ¢'FV×Æ•÷&W7F'E÷&÷VæB"ÓâvÖTWfVçEG—S£¥FVÕÆ•&W7F'E&÷VæBÀÐ¢'FV×Æ•÷&VG•÷&W7F'B"ÓâvÖTWfVçEG—S£¥FVÕÆ•&VG•&W7F'BÀÐ¢'FV×Æ•÷&÷VæE÷&W7F'E÷6V6öæG2"ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæE&W7F'E6V6öæG2ÀÐ¢'FV×Æ•÷FVÕ÷&VG’"ÓâvÖTWfVçEG—S£¥FVÕÆ•FVÕ&VG’ÀÐ¢'FV×Æ•÷&÷VæE÷v–â"ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæEv–âÀÐ¢'FV×Æ•÷WFFU÷F–ÖW""ÓâvÖTWfVçEG—S£¥FVÕÆ•WFFUF–ÖW"ÀÐ¢'FV×Æ•÷&÷VæE÷7FÆVÖFR"ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæE7FÆVÖFRÀÐ¢'FV×Æ•ö÷fW'F–ÖUö&Vv–â"ÓâvÖTWfVçEG—S£¥FVÕÆ”÷fW'F–ÖT&Vv–âÀÐ¢'FV×Æ•ö÷fW'F–ÖUöVæB"ÓâvÖTWfVçEG—S£¥FVÕÆ”÷fW'F–ÖTVæBÀÐ¢'FV×Æ•÷7VFFVæFVF…ö&Vv–â"ÓâvÖTWfVçEG—S£¥FVÕÆ•7VFFVäFVF„&Vv–âÀÐ¢'FV×Æ•÷7VFFVæFVF…öVæB"ÓâvÖTWfVçEG—S£¥FVÕÆ•7VFFVäFVF„VæBÀÐ¢'FV×Æ•övÖUö÷fW""ÓâvÖTWfVçEG—S£¥FVÕÆ”vÖT÷fW"ÀÐ¢'FV×Æ•öÖ÷F–ÖU÷&VÖ–æ–ær"ÓâvÖTWfVçEG—S£¥FVÕÆ”ÖF–ÖU&VÖ–æ–ærÀÐ¢'FV×Æ•÷F–ÖW%öfÆ6‚"ÓâvÖTWfVçEG—S£¥FVÕÆ•F–ÖW$fÆ6‚ÀÐ¢'FV×Æ•÷F–ÖW%÷F–ÖUöFFVB"ÓâvÖTWfVçEG—S£¥FVÕÆ•F–ÖW%F–ÖTFFVBÀÐ¢'FV×Æ•÷ö–çE÷7F'F6GW&R"ÓâvÖTWfVçEG—S£¥FVÕÆ•ö–çE7F'D6GW&RÀÐ¢'FV×Æ•÷ö–çEö6GW&VB"ÓâvÖTWfVçEG—S£¥FVÕÆ•ö–çD6GW&VBÀÐ¢'FV×Æ•÷ö–çEöÆö6¶VB"ÓâvÖTWfVçEG—S£¥FVÕÆ•ö–çDÆö6¶VBÀÐ¢'FV×Æ•÷ö–çE÷VæÆö6¶VB"ÓâvÖTWfVçEG—S£¥FVÕÆ•ö–çEVæÆö6¶VBÀÐ¢'FV×Æ•ö6GW&Uö'&ö¶Vâ"ÓâvÖTWfVçEG—S£¥FVÕÆ”6GW&T'&ö¶VâÀÐ¢'FV×Æ•ö6GW&Uö&Æö6¶VB"ÓâvÖTWfVçEG—S£¥FVÕÆ”6GW&T&Æö6¶VBÀÐ¢'FV×Æ•öfÆuöWfVçB"ÓâvÖTWfVçEG—S£¥FVÕÆ”fÆtWfVçBÀÐ¢'FV×Æ•÷v–å÷æVÂ"ÓâvÖTWfVçEG—S£¥FVÕÆ•v–åæVÂÀÐ¢'FV×Æ•÷FVÖ&Ææ6VE÷Æ–W""ÓâvÖTWfVçEG—S£¥FVÕÆ•FVÔ&Ææ6VEÆ–W"ÀÐ¢'FV×Æ•÷6WGWöf–æ—6†VB"ÓâvÖTWfVçEG—S£¥FVÕÆ•6WGWf–æ—6†VBÀÐ¢'FV×Æ•öÆW'B"ÓâvÖTWfVçEG—S£¥FVÕÆ”ÆW'BÀÐ¢'G&–æ–æuö6ö×ÆWFR"ÓâvÖTWfVçEG—S£¥G&–æ–æt6ö×ÆWFRÀÐ¢'6†÷uög&VW¦WæVÂ"ÓâvÖTWfVçEG—S£¥6†÷tg&VW¦UæVÂÀÐ¢&†–FUög&VW¦WæVÂ"ÓâvÖTWfVçEG—S£¤†–FTg&VW¦UæVÂÀÐ¢&g&VW¦V6Õ÷7F'FVB"ÓâvÖTWfVçEG—S£¤g&VW¦T6Õ7F'FVBÀÐ¢&Æö6ÇÆ–W%ö6†ævWFVÒ"ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævUFVÒÀÐ¢&Æö6ÇÆ–W%÷66÷&Uö6†ævVB"ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W%66÷&T6†ævVBÀÐ¢&Æö6ÇÆ–W%ö6†ævV6Æ72"ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævT6Æ72ÀÐ¢&Æö6ÇÆ–W%÷&W7vâ"ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W%&W7vâÀÐ¢&'V–ÆF–æuö–æfõö6†ævVB"ÓâvÖTWfVçEG—S£¤'V–ÆF–æt–æfô6†ævVBÀÐ¢&Æö6ÇÆ–W%ö6†ævVF—6wV—6R"ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævTF—6wV—6RÀÐ¢'Æ–W%ö66÷VçEö6†ævVB"ÓâvÖTWfVçEG—S£¥Æ–W$66÷VçD6†ævVBÀÐ¢'7•÷F÷&W6WB"ÓâvÖTWfVçEG—S£¥7•F&W6WBÀÐ¢&fÆw7FGW5÷WFFR"ÓâvÖTWfVçEG—S£¤fÆu7FGW5WFFRÀÐ¢'Æ–W%÷7FG5÷WFFVB"ÓâvÖTWfVçEG—S£¥Æ–W%7FG5WFFVBÀÐ¢'Æ––æuö6öÖÖVçF'’"ÓâvÖTWfVçEG—S£¥Æ––æt6öÖÖVçF'’ÀÐ¢'Æ–W%ö6†&vVFWÆ÷–VB"ÓâvÖTWfVçEG—S£¥Æ–W$6†&vTFWÆ÷–VBÀÐ¢'Æ–W%ö'V–ÇFö&¦V7B"ÓâvÖTWfVçEG—S£¥Æ–W$'V–ÇDö&¦V7BÀÐ¢'Æ–W%÷Ww&FVFö&¦V7B"ÓâvÖTWfVçEG—S£¥Æ–W%Ww&FVDö&¦V7BÀÐ¢'Æ–W%ö6''–ö&¦V7B"ÓâvÖTWfVçEG—S£¥Æ–W$6''”ö&¦V7BÀÐ¢'Æ–W%öG&÷ö&¦V7B"ÓâvÖTWfVçEG—S£¥Æ–W$G&÷ö&¦V7BÀÐ¢&ö&¦V7E÷&VÖ÷fVB"ÓâvÖTWfVçEG—S£¤ö&¦V7E&VÖ÷fVBÀÐ¢&ö&¦V7EöFW7G&÷–VB"ÓâvÖTWfVçEG—S£¤ö&¦V7DFW7G&÷–VBÀÐ¢&ö&¦V7EöFWFöæFVB"ÓâvÖTWfVçEG—S£¤ö&¦V7DFWFöæFVBÀÐ¢&6†–WfVÖVçEöV&æVB"ÓâvÖTWfVçEG—S£¤6†–WfVÖVçDV&æVBÀÐ¢'7V5÷F&vWE÷WFFVB"ÓâvÖTWfVçEG—S£¥7V5F&vWEWFFVBÀÐ¢'F÷W&æÖVçE÷7FFWWFFR"ÓâvÖTWfVçEG—S£¥F÷W&æÖVçE7FFUWFFRÀÐ¢'F÷W&æÖVçEöVæ&ÆV6÷VçFF÷vâ"ÓâvÖTWfVçEG—S£¥F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâÀÐ¢'Æ–W%ö6ÆÆVFf÷&ÖVF–2"ÓâvÖTWfVçEG—S£¥Æ–W$6ÆÆVDf÷$ÖVF–2ÀÐ¢'Æ–W%ö6¶VFf÷&&ÆÂ"ÓâvÖTWfVçEG—S£¥Æ–W$6¶VDf÷$&ÆÂÀÐ¢&Æö6ÇÆ–W%ö&V6ÖVö'6W'fW""ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$&V6ÖTö'6W'fW"ÀÐ¢'Æ–W%ö–væ—FVEö–çb"ÓâvÖTWfVçEG—S£¥Æ–W$–væ—FVD–çbÀÐ¢'Æ–W%ö–væ—FVB"ÓâvÖTWfVçEG—S£¥Æ–W$–væ—FVBÀÐ¢'Æ–W%öW‡F–æwV—6†VB"ÓâvÖTWfVçEG—S£¥Æ–W$W‡F–æwV—6†VBÀÐ¢'Æ–W%÷FVÆW÷'FVB"ÓâvÖTWfVçEG—S£¥Æ–W%FVÆW÷'FVBÀÐ¢'Æ–W%ö†VÆVFÖVF–66ÆÂ"ÓâvÖTWfVçEG—S£¥Æ–W$†VÆVDÖVF–46ÆÂÀÐ¢&Æö6ÇÆ–W%ö6†&vW&VG’"ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$6†&vU&VG’ÀÐ¢&Æö6ÇÆ–W%÷v–æFF÷vâ"ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W%v–æDF÷vâÀÐ¢'Æ–W%ö–çgVÆæVB"ÓâvÖTWfVçEG—S£¥Æ–W$–çgVÆæVBÀÐ¢&W66÷'E÷7VVB"ÓâvÖTWfVçEG—S£¤W66÷'E7VVBÀÐ¢&W66÷'E÷&öw&W72"ÓâvÖTWfVçEG—S£¤W66÷'E&öw&W72ÀÐ¢&W66÷'E÷&V6VFR"ÓâvÖTWfVçEG—S£¤W66÷'E&V6VFRÀÐ¢&vÖWV•ö7F—fFVB"ÓâvÖTWfVçEG—S£¤vÖUT”7F—fFVBÀÐ¢&vÖWV•ö†–FFVâ"ÓâvÖTWfVçEG—S£¤vÖUT”†–FFVâÀÐ¢'Æ–W%öW66÷'E÷66÷&R"ÓâvÖTWfVçEG—S£¥Æ–W$W66÷'E66÷&RÀÐ¢'Æ–W%ö†VÆöæ†—B"ÓâvÖTWfVçEG—S£¥Æ–W$†VÄöä†—BÀÐ¢'Æ–W%÷7FVÇ6æGf–6‚"ÓâvÖTWfVçEG—S£¥Æ–W%7FVÅ6æGf–6‚ÀÐ¢'6†÷uö6Æ75öÆ–÷WB"ÓâvÖTWfVçEG—S£¥6†÷t6Æ74Æ–÷WBÀÐ¢'6†÷u÷g5÷æVÂ"ÓâvÖTWfVçEG—S£¥6†÷ug5æVÂÀÐ¢'Æ–W%öFÖvVB"ÓâvÖTWfVçEG—S£¥Æ–W$FÖvVBÀÐ¢&&Væ÷Æ–W%öæ÷F–f–6F–öâ"ÓâvÖTWfVçEG—S£¤&VæÆ–W$æ÷F–f–6F–öâÀÐ¢&&VæöÖF6…öÖ‡7G&V²"ÓâvÖTWfVçEG—S£¤&VæÖF6„Ö…7G&V²ÀÐ¢&&Væ÷&÷VæE÷7F'B"ÓâvÖTWfVçEG—S£¤&Væ&÷VæE7F'BÀÐ¢&&Væ÷v–å÷æVÂ"ÓâvÖTWfVçEG—S£¤&Væv–åæVÂÀÐ¢'fU÷v–å÷æVÂ"ÓâvÖTWfVçEG—S£¥fUv–åæVÂÀÐ¢&—%öF6‚"ÓâvÖTWfVçEG—S£¤—$F6‚ÀÐ¢&ÆæFVB"ÓâvÖTWfVçEG—S£¤ÆæFVBÀÐ¢'Æ–W%öFÖvUöFöFvVB"ÓâvÖTWfVçEG—S£¥Æ–W$FÖvTFöFvVBÀÐ¢'Æ–W%÷7GVææVB"ÓâvÖTWfVçEG—S£¥Æ–W%7GVææVBÀÐ¢'66÷WEöw&æE÷6ÆÒ"ÓâvÖTWfVçEG—S£¥66÷WDw&æE6ÆÒÀÐ¢'66÷WE÷6ÆÖFöÆÅöÆæFVB"ÓâvÖTWfVçEG—S£¥66÷WE6ÆÖFöÆÄÆæFVBÀÐ¢&'&÷uö–×7B"ÓâvÖTWfVçEG—S£¤'&÷t–×7BÀÐ¢'Æ–W%ö¦&FVB"ÓâvÖTWfVçEG—S£¥Æ–W$¦&FVBÀÐ¢'Æ–W%ö¦&FVEöfFR"ÓâvÖTWfVçEG—S£¥Æ–W$¦&FVDfFRÀÐ¢'Æ–W%÷6†–VÆEö&Æö6¶VB"ÓâvÖTWfVçEG—S£¥Æ–W%6†–VÆD&Æö6¶VBÀÐ¢'Æ–W%÷–ææVB"ÓâvÖTWfVçEG—S£¥Æ–W%–ææVBÀÐ¢'Æ–W%ö†VÆVF'–ÖVF–2"ÓâvÖTWfVçEG—S£¥Æ–W$†VÆVD'”ÖVF–2ÀÐ¢'Æ–W%÷6VEöö&¦V7B"ÓâvÖTWfVçEG—S£¥Æ–W%6VDö&¦V7BÀÐ¢&—FVÕöf÷VæB"ÓâvÖTWfVçEG—S£¤—FVÔf÷VæBÀÐ¢'6†÷uöææ÷FF–öâ"ÓâvÖTWfVçEG—S£¥6†÷tææ÷FF–öâÀÐ¢&†–FUöææ÷FF–öâ"ÓâvÖTWfVçEG—S£¤†–FTææ÷FF–öâÀÐ¢'÷7Eö–çfVçF÷'•öÆ–6F–öâ"ÓâvÖTWfVçEG—S£¥÷7D–çfVçF÷'”Æ–6F–öâÀÐ¢&6öçG&öÇö–çE÷VæÆö6µ÷WFFVB"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEVæÆö6µWFFVBÀÐ¢&FWÆ÷•ö'Vfeö&ææW""ÓâvÖTWfVçEG—S£¤FWÆ÷”'Vfd&ææW"ÀÐ¢'Æ–W%ö'Vfb"ÓâvÖTWfVçEG—S£¥Æ–W$'VfbÀÐ¢&ÖVF–5öFVF‚"ÓâvÖTWfVçEG—S£¤ÖVF–4FVF‚ÀÐ¢&÷fW'F–ÖUöær"ÓâvÖTWfVçEG—S£¤÷fW'F–ÖTærÀÐ¢'FV×5ö6†ævVB"ÓâvÖTWfVçEG—S£¥FV×46†ævVBÀÐ¢&†ÆÆ÷vVVå÷V×¶–åöw&""ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVåV×¶–äw&"ÀÐ¢'&ö6¶WEö§V×"ÓâvÖTWfVçEG—S£¥&ö6¶WD§V×ÀÐ¢'&ö6¶WEö§V×öÆæFVB"ÓâvÖTWfVçEG—S£¥&ö6¶WD§V×ÆæFVBÀÐ¢'7F–6·•ö§V×"ÓâvÖTWfVçEG—S£¥7F–6·”§V×ÀÐ¢'7F–6·•ö§V×öÆæFVB"ÓâvÖTWfVçEG—S£¥7F–6·”§V×ÆæFVBÀÐ¢'&ö6¶WG6µöÆVæ6‚"ÓâvÖTWfVçEG—S£¥&ö6¶WE6´ÆVæ6‚ÀÐ¢'&ö6¶WG6µöÆæFVB"ÓâvÖTWfVçEG—S£¥&ö6¶WE6´ÆæFVBÀÐ¢&ÖVF–5öFVfVæFVB"ÓâvÖTWfVçEG—S£¤ÖVF–4FVfVæFVBÀÐ¢&Æö6ÇÆ–W%ö†VÆVB"ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$†VÆVBÀÐ¢'Æ–W%öFW7G&÷–VE÷—V&öÖ""ÓâvÖTWfVçEG—S£¥Æ–W$FW7G&÷–VE—T&öÖ"ÀÐ¢&ö&¦V7EöFVfÆV7FVB"ÓâvÖTWfVçEG—S£¤ö&¦V7DFVfÆV7FVBÀÐ¢'Æ–W%ö×g"ÓâvÖTWfVçEG—S£¥Æ–W$×gÀÐ¢'&–E÷7våöÖö""ÓâvÖTWfVçEG—S£¥&–E7väÖö"ÀÐ¢'&–E÷7vå÷7VB"ÓâvÖTWfVçEG—S£¥&–E7vå7VBÀÐ¢&æeö&Æö6¶VB"ÓâvÖTWfVçEG—S£¤æd&Æö6¶VBÀÐ¢'F…÷G&6µ÷76VB"ÓâvÖTWfVçEG—S£¥F…G&6µ76VBÀÐ¢&çVÕö6W'5ö6†ævVB"ÓâvÖTWfVçEG—S£¤çVÔ6W'46†ævVBÀÐ¢'Æ–W%÷&VvVæW&FR"ÓâvÖTWfVçEG—S£¥Æ–W%&VvVæW&FRÀÐ¢'WFFU÷7FGW5ö—FVÒ"ÓâvÖTWfVçEG—S£¥WFFU7FGW4—FVÒÀÐ¢'7FG5÷&W6WG&÷VæB"ÓâvÖTWfVçEG—S£¥7FG5&W6WE&÷VæBÀÐ¢'66÷&W7FG5ö67V×VÆFVE÷WFFR"ÓâvÖTWfVçEG—S£¥66÷&U7FG467V×VÆFVEWFFRÀÐ¢'66÷&W7FG5ö67V×VÆFVE÷&W6WB"ÓâvÖTWfVçEG—S£¥66÷&U7FG467V×VÆFVE&W6WBÀÐ¢&6†–WfVÖVçEöV&æVEöÆö6Â"ÓâvÖTWfVçEG—S£¤6†–WfVÖVçDV&æVDÆö6ÂÀÐ¢'Æ–W%ö†VÆVB"ÓâvÖTWfVçEG—S£¥Æ–W$†VÆVBÀÐ¢&'V–ÆF–æuö†VÆVB"ÓâvÖTWfVçEG—S£¤'V–ÆF–æt†VÆVBÀÐ¢&—FVÕ÷–6·W"ÓâvÖTWfVçEG—S£¤—FVÕ–6·WÀÐ¢&GVVÅ÷7FGW2"ÓâvÖTWfVçEG—S£¤GVVÅ7FGW2ÀÐ¢&f—6…öæ÷F–6R"ÓâvÖTWfVçEG—S£¤f—6„æ÷F–6RÀÐ¢&f—6…öæ÷F–6Uõö&Ò"ÓâvÖTWfVçEG—S£¤f—6„æ÷F–6T&ÒÀÐ¢'6Æöæ÷F–6R"ÓâvÖTWfVçEG—S£¥6Ææ÷F–6RÀÐ¢'F‡&÷v&ÆUö†—B"ÓâvÖTWfVçEG—S£¥F‡&÷v&ÆT†—BÀÐ¢'V×¶–åöÆ÷&E÷7VÖÖöæVB"ÓâvÖTWfVçEG—S£¥V×¶–äÆ÷&E7VÖÖöæVBÀÐ¢'V×¶–åöÆ÷&Eö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¥V×¶–äÆ÷&D¶–ÆÆVBÀÐ¢&ÖW&6×W5÷7VÖÖöæVB"ÓâvÖTWfVçEG—S£¤ÖW&6×W57VÖÖöæVBÀÐ¢&ÖW&6×W5ö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¤ÖW&6×W4¶–ÆÆVBÀÐ¢&ÖW&6×W5öW66U÷v&æ–ær"ÓâvÖTWfVçEG—S£¤ÖW&6×W4W66Uv&æ–ærÀÐ¢&ÖW&6×W5öW66VB"ÓâvÖTWfVçEG—S£¤ÖW&6×W4W66VBÀÐ¢&W–V&ÆÅö&÷75÷7VÖÖöæVB"ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷757VÖÖöæVBÀÐ¢&W–V&ÆÅö&÷75÷7GVææVB"ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷757GVææVBÀÐ¢&W–V&ÆÅö&÷75ö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷74¶–ÆÆVBÀÐ¢&W–V&ÆÅö&÷75ö¶–ÆÆW""ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷74¶–ÆÆW"ÀÐ¢&W–V&ÆÅö&÷75öW66Uö–ÖÖ–æVçB"ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷74W66T–ÖÖ–æVçBÀÐ¢&W–V&ÆÅö&÷75öW66VB"ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷74W66VBÀÐ¢&ç5ö‡W'B"ÓâvÖTWfVçEG—S£¤ç4‡W'BÀÐ¢&6öçG&öÇö–çE÷F–ÖW%÷WFFVB"ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEF–ÖW%WFFVBÀÐ¢'Æ–W%ö†–v†f—fU÷7F'B"ÓâvÖTWfVçEG—S£¥Æ–W$†–v„f—fU7F'BÀÐ¢'Æ–W%ö†–v†f—fUö6æ6VÂ"ÓâvÖTWfVçEG—S£¥Æ–W$†–v„f—fT6æ6VÂÀÐ¢'Æ–W%ö†–v†f—fU÷7V66W72"ÓâvÖTWfVçEG—S£¥Æ–W$†–v„f—fU7V66W72ÀÐ¢'Æ–W%ö&öçW7ö–çG2"ÓâvÖTWfVçEG—S£¥Æ–W$&öçW5ö–çG2ÀÐ¢'Æ–W%÷Ww&FVB"ÓâvÖTWfVçEG—S£¥Æ–W%Ww&FVBÀÐ¢'Æ–W%ö'W–&6²"ÓâvÖTWfVçEG—S£¥Æ–W$'W–&6²ÀÐ¢'Æ–W%÷W6VE÷÷vW'Wö&÷GFÆR"ÓâvÖTWfVçEG—S£¥Æ–W%W6VE÷vW%W&÷GFÆRÀÐ¢&6‡&—7FÖ5öv–gEöw&""ÓâvÖTWfVçEG—S£¤6‡&—7FÖ4v–gDw&"ÀÐ¢'Æ–W%ö¶–ÆÆVEö6†–WfVÖVçE÷¦öæR"ÓâvÖTWfVçEG—S£¥Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæRÀÐ¢''G•÷WFFVB"ÓâvÖTWfVçEG—S£¥'G•WFFVBÀÐ¢''G•÷&Veö6†ævVB"ÓâvÖTWfVçEG—S£¥'G•&Vd6†ævVBÀÐ¢''G•ö7&—FW&–ö6†ævVB"ÓâvÖTWfVçEG—S£¥'G”7&—FW&–6†ævVBÀÐ¢''G•ö–çf—FW5ö6†ævVB"ÓâvÖTWfVçEG—S£¥'G”–çf—FW46†ævVBÀÐ¢''G•÷VWVU÷7FFUö6†ævVB"ÓâvÖTWfVçEG—S£¥'G•VWVU7FFT6†ævVBÀÐ¢''G•ö6†B"ÓâvÖTWfVçEG—S£¥'G”6†BÀÐ¢''G•öÖVÖ&W%ö¦ö–â"ÓâvÖTWfVçEG—S£¥'G”ÖVÖ&W$¦ö–âÀÐ¢''G•öÖVÖ&W%öÆVfR"ÓâvÖTWfVçEG—S£¥'G”ÖVÖ&W$ÆVfRÀÐ¢&ÖF6…ö–çf—FW5÷WFFVB"ÓâvÖTWfVçEG—S£¤ÖF6„–çf—FW5WFFVBÀÐ¢&Æö&'•÷WFFVB"ÓâvÖTWfVçEG—S£¤Æö&'•WFFVBÀÐ¢&×fÕöÖ—76–öå÷WFFR"ÓâvÖTWfVçEG—S£¤×fÔÖ—76–öåWFFRÀÐ¢'&V6Æ7VÆFUö†öÆ–F—2"ÓâvÖTWfVçEG—S£¥&V6Æ7VÆFT†öÆ–F—2ÀÐ¢'Æ–W%ö7W'&Væ7•ö6†ævVB"ÓâvÖTWfVçEG—S£¥Æ–W$7W'&Væ7”6†ævVBÀÐ¢&Föö×6F•÷&ö6¶WEö÷Vâ"ÓâvÖTWfVçEG—S£¤Föö×6F•&ö6¶WD÷VâÀÐ¢'&VÖ÷fUöæVÖW6—5÷&VÆF–öç6†—2"ÓâvÖTWfVçEG—S£¥&VÖ÷fTæVÖW6—5&VÆF–öç6†—2ÀÐ¢&×fÕö7&VF—F&öçW5÷vfR"ÓâvÖTWfVçEG—S£¤×fÔ7&VF—D&öçW5vfRÀÐ¢&×fÕö7&VF—F&öçW5öÆÂ"ÓâvÖTWfVçEG—S£¤×fÔ7&VF—D&öçW4ÆÂÀÐ¢&×fÕö7&VF—F&öçW5öÆÅöGfæ6VB"ÓâvÖTWfVçEG—S£¤×fÔ7&VF—D&öçW4ÆÄGfæ6VBÀÐ¢&×fÕ÷V–6µ÷6VçG'•÷Ww&FR"ÓâvÖTWfVçEG—S£¤×fÕV–6µ6VçG'•Ww&FRÀÐ¢&×fÕ÷FæµöFW7G&÷–VEö'•÷Æ–W'2"ÓâvÖTWfVçEG—S£¤×fÕFæ´FW7G&÷–VD'•Æ–W'2ÀÐ¢&×fÕö¶–ÆÅ÷&ö&÷EöFVÆ—fW&–æuö&öÖ""ÓâvÖTWfVçEG—S£¤×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ"ÀÐ¢&×fÕ÷–6·Wö7W'&Væ7’"ÓâvÖTWfVçEG—S£¤×fÕ–6·W7W'&Væ7’ÀÐ¢&×fÕö&öÖ%ö6'&–W%ö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¤×fÔ&öÖ$6'&–W$¶–ÆÆVBÀÐ¢&×fÕ÷6VçG'–'W7FW%öFWFöæFR"ÓâvÖTWfVçEG—S£¤×fÕ6VçG'”'W7FW$FWFöæFRÀÐ¢&×fÕ÷66÷WEöÖ&¶VEöf÷%öFVF‚"ÓâvÖTWfVçEG—S£¤×fÕ66÷WDÖ&¶VDf÷$FVF‚ÀÐ¢&×fÕöÖVF–5÷÷vW'W÷6†&VB"ÓâvÖTWfVçEG—S£¤×fÔÖVF–5÷vW%W6†&VBÀÐ¢&×fÕö&Vv–å÷vfR"ÓâvÖTWfVçEG—S£¤×fÔ&Vv–åvfRÀÐ¢&×fÕ÷vfUö6ö×ÆWFR"ÓâvÖTWfVçEG—S£¤×fÕvfT6ö×ÆWFRÀÐ¢&×fÕöÖ—76–öåö6ö×ÆWFR"ÓâvÖTWfVçEG—S£¤×fÔÖ—76–öä6ö×ÆWFRÀÐ¢&×fÕö&öÖ%÷&W6WEö'•÷Æ–W""ÓâvÖTWfVçEG—S£¤×fÔ&öÖ%&W6WD'•Æ–W"ÀÐ¢&×fÕö&öÖ%öÆ&Õ÷G&–vvW&VB"ÓâvÖTWfVçEG—S£¤×fÔ&öÖ$Æ&ÕG&–vvW&VBÀÐ¢&×fÕö&öÖ%öFWÆ÷•÷&W6WEö'•÷Æ–W""ÓâvÖTWfVçEG—S£¤×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W"ÀÐ¢&×fÕ÷vfUöf–ÆVB"ÓâvÖTWfVçEG—S£¤×fÕvfTf–ÆVBÀÐ¢&×fÕ÷&W6WE÷7FG2"ÓâvÖTWfVçEG—S£¤×fÕ&W6WE7FG2ÀÐ¢&FÖvU÷&W6—7FVB"ÓâvÖTWfVçEG—S£¤FÖvU&W6—7FVBÀÐ¢'&Wf—fU÷Æ–W%öæ÷F–g’"ÓâvÖTWfVçEG—S£¥&Wf—fUÆ–W$æ÷F–g’ÀÐ¢'&Wf—fU÷Æ–W%÷7F÷VB"ÓâvÖTWfVçEG—S£¥&Wf—fUÆ–W%7F÷VBÀÐ¢'&Wf—fU÷Æ–W%ö6ö×ÆWFR"ÓâvÖTWfVçEG—S£¥&Wf—fUÆ–W$6ö×ÆWFRÀÐ¢'Æ–W%÷GW&æVE÷Fõöv†÷7B"ÓâvÖTWfVçEG—S£¥Æ–W%GW&æVEFôv†÷7BÀÐ¢&ÖVF–wVå÷6†–VÆEö&Æö6¶VEöFÖvR"ÓâvÖTWfVçEG—S£¤ÖVF–wVå6†–VÆD&Æö6¶VDFÖvRÀÐ¢&×fÕöGe÷vfUö6ö×ÆWFUöæõövFW2"ÓâvÖTWfVçEG—S£¤×fÔGevfT6ö×ÆWFTæôvFW2ÀÐ¢&×fÕ÷6æ—W%ö†VG6†÷Eö7W'&Væ7’"ÓâvÖTWfVçEG—S£¤×fÕ6æ—W$†VG6†÷D7W'&Væ7’ÀÐ¢&×fÕöÖææ†GFå÷—B"ÓâvÖTWfVçEG—S£¤×fÔÖææ†GFå—BÀÐ¢&fÆuö6'&–VEö–åöFWFV7F–öå÷¦öæR"ÓâvÖTWfVçEG—S£¤fÆt6'&–VD–äFWFV7F–öå¦öæRÀÐ¢&×fÕöGe÷vfUö¶–ÆÆVE÷7GVå÷&F–ò"ÓâvÖTWfVçEG—S£¤×fÔGevfT¶–ÆÆVE7GVå&F–òÀÐ¢'Æ–W%öF—&V7F†—E÷7GVâ"ÓâvÖTWfVçEG—S£¥Æ–W$F—&V7D†—E7GVâÀÐ¢&×fÕ÷6VçG'–'W7FW%ö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¤×fÕ6VçG'”'W7FW$¶–ÆÆVBÀÐ¢'Ww&FW5öf–ÆUö6†ævVB"ÓâvÖTWfVçEG—S£¥Ww&FW4f–ÆT6†ævVBÀÐ¢'&E÷FVÕ÷ö–çG5ö6†ævVB"ÓâvÖTWfVçEG—S£¥&EFVÕö–çG46†ævVBÀÐ¢'&E÷'VÆW5÷7FFUö6†ævVB"ÓâvÖTWfVçEG—S£¥&E'VÆW57FFT6†ævVBÀÐ¢'&E÷&ö&÷Eö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¥&E&ö&÷D¶–ÆÆVBÀÐ¢'&E÷&ö&÷Eö–×7B"ÓâvÖTWfVçEG—S£¥&E&ö&÷D–×7BÀÐ¢'FV×Æ•÷&U÷&÷VæE÷F–ÖUöÆVgB"ÓâvÖTWfVçEG—S£¥FVÕÆ•&U&÷VæEF–ÖTÆVgBÀÐ¢'&6‡WFUöFWÆ÷’"ÓâvÖTWfVçEG—S£¥&6‡WFTFWÆ÷’ÀÐ¢'&6‡WFUö†öÇ7FW""ÓâvÖTWfVçEG—S£¥&6‡WFT†öÇ7FW"ÀÐ¢&¶–ÆÅ÷&Vf–ÆÇ5öÖWFW""ÓâvÖTWfVçEG—S£¤¶–ÆÅ&Vf–ÆÇ4ÖWFW"ÀÐ¢''5÷FVçEöWfVçB"ÓâvÖTWfVçEG—S£¥'5FVçDWfVçBÀÐ¢&6öævö¶–ÆÂ"ÓâvÖTWfVçEG—S£¤6öæv¶–ÆÂÀÐ¢'Æ–W%ö–æ—F–Å÷7vâ"ÓâvÖTWfVçEG—S£¥Æ–W$–æ—F–Å7vâÀÐ¢&6ö×WF—F—fU÷f–7F÷'’"ÓâvÖTWfVçEG—S£¤6ö×WF—F—fUf–7F÷'’ÀÐ¢&6ö×WF—F—fU÷7FG5÷WFFR"ÓâvÖTWfVçEG—S£¤6ö×WF—F—fU7FG5WFFRÀÐ¢&Ö–æ–vÖU÷v–â"ÓâvÖTWfVçEG—S£¤Ö–æ”vÖUv–âÀÐ¢'6VçG'•ööåövõö7F—fR"ÓâvÖTWfVçEG—S£¥6VçG'”öävô7F—fRÀÐ¢&GV6µ÷‡öÆWfVÅ÷W"ÓâvÖTWfVçEG—S£¤GV6µ‡ÆWfVÅWÀÐ¢'VW7FÆöuö÷VæVB"ÓâvÖTWfVçEG—S£¥VW7DÆöt÷VæVBÀÐ¢'66†VÖ÷WFFVB"ÓâvÖTWfVçEG—S£¥66†VÖWFFVBÀÐ¢&Æö6ÇÆ–W%÷–6·W÷vVöâ"ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W%–6·WvVöâÀÐ¢'&E÷Æ–W%÷66÷&U÷ö–çG2"ÓâvÖTWfVçEG—S£¥&EÆ–W%66÷&Uö–çG2ÀÐ¢&FVÖöÖåöFWE÷7F–6¶–W2"ÓâvÖTWfVçEG—S£¤FVÖöÖäFWE7F–6¶–W2ÀÐ¢'VW7Eöö&¦V7F—fUö6ö×ÆWFVB"ÓâvÖTWfVçEG—S£¥VW7Dö&¦V7F—fT6ö×ÆWFVBÀÐ¢'Æ–W%÷66÷&Uö6†ævVB"ÓâvÖTWfVçEG—S£¥Æ–W%66÷&T6†ævVBÀÐ¢&¶–ÆÆVEö6–æu÷Æ–W""ÓâvÖTWfVçEG—S£¤¶–ÆÆVD6–æuÆ–W"ÀÐ¢&Vçf—&öæÖVçFÅöFVF‚"ÓâvÖTWfVçEG—S£¤Vçf—&öæÖVçFÄFVF‚ÀÐ¢'&ö¦V7F–ÆUöF—&V7Eö†—B"ÓâvÖTWfVçEG—S£¥&ö¦V7F–ÆTF—&V7D†—BÀÐ¢'75övWB"ÓâvÖTWfVçEG—S£¥74vWBÀÐ¢'75÷66÷&R"ÓâvÖTWfVçEG—S£¥7566÷&RÀÐ¢'75ög&VR"ÓâvÖTWfVçEG—S£¥74g&VRÀÐ¢'75÷75ö6Vv‡B"ÓâvÖTWfVçEG—S£¥75746Vv‡BÀÐ¢'75ö&ÆÅ÷7FöÆVâ"ÓâvÖTWfVçEG—S£¥74&ÆÅ7FöÆVâÀÐ¢'75ö&ÆÅö&Æö6¶VB"ÓâvÖTWfVçEG—S£¥74&ÆÄ&Æö6¶VBÀÐ¢&FÖvU÷&WfVçFVB"ÓâvÖTWfVçEG—S£¤FÖvU&WfVçFVBÀÐ¢&†ÆÆ÷vVVåö&÷75ö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVä&÷74¶–ÆÆVBÀÐ¢&W66VEöÆö÷Eö—6ÆæB"ÓâvÖTWfVçEG—S£¤W66VDÆö÷D—6ÆæBÀÐ¢'FvvVE÷Æ–W%ö5ö—B"ÓâvÖTWfVçEG—S£¥FvvVEÆ–W$4—BÀÐ¢&ÖW&6×W5÷7GVææVB"ÓâvÖTWfVçEG—S£¤ÖW&6×W57GVææVBÀÐ¢&ÖW&6×W5÷&÷öf÷VæB"ÓâvÖTWfVçEG—S£¤ÖW&6×W5&÷f÷VæBÀÐ¢&†ÆÆ÷vVVå÷6¶VÆWFöåö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVBÀÐ¢'6¶VÆWFöåö¶–ÆÆVE÷VW7B"ÓâvÖTWfVçEG—S£¥6¶VÆWFöä¶–ÆÆVEVW7BÀÐ¢'6¶VÆWFöåö¶–æuö¶–ÆÆVE÷VW7B"ÓâvÖTWfVçEG—S£¥6¶VÆWFöä¶–æt¶–ÆÆVEVW7BÀÐ¢&W66Uö†VÆÂ"ÓâvÖTWfVçEG—S£¤W66T†VÆÂÀÐ¢&7&÷75÷7V7G&Åö'&–FvR"ÓâvÖTWfVçEG—S£¤7&÷757V7G&Ä'&–FvRÀÐ¢&Ö–æ–vÖU÷vöâ"ÓâvÖTWfVçEG—S£¤Ö–æ”vÖUvöâÀÐ¢'&W7våöv†÷7B"ÓâvÖTWfVçEG—S£¥&W7väv†÷7BÀÐ¢&¶–ÆÅö–åö†VÆÂ"ÓâvÖTWfVçEG—S£¤¶–ÆÄ–ä†VÆÂÀÐ¢&†ÆÆ÷vVVåöGV6µö6öÆÆV7FVB"ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVäGV6´6öÆÆV7FVBÀÐ¢'7V6–Å÷66÷&R"ÓâvÖTWfVçEG—S£¥7V6–Å66÷&RÀÐ¢'FVÕöÆVFW%ö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¥FVÔÆVFW$¶–ÆÆVBÀÐ¢&†ÆÆ÷vVVå÷6÷VÅö6öÆÆV7FVB"ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVBÀÐ¢'&V6Æ7VÆFU÷G'V6R"ÓâvÖTWfVçEG—S£¥&V6Æ7VÆFUG'V6RÀÐ¢&FVG&–ævW%ö6†VEöFVF‚"ÓâvÖTWfVçEG—S£¤FVE&–ævW$6†VDFVF‚ÀÐ¢&7&÷76&÷uö†VÂ"ÓâvÖTWfVçEG—S£¤7&÷76&÷t†VÂÀÐ¢&FÖvUöÖ—F–vFVB"ÓâvÖTWfVçEG—S£¤FÖvTÖ—F–vFVBÀÐ¢'–ÆöE÷W6†VB"ÓâvÖTWfVçEG—S£¥–ÆöEW6†VBÀÐ¢'Æ–W%ö&æFöæVEöÖF6‚"ÓâvÖTWfVçEG—S£¥Æ–W$&æFöæVDÖF6‚ÀÐ¢&6ÅöG&vÆ–æR"ÓâvÖTWfVçEG—S£¤6ÄG&vÆ–æRÀÐ¢'&W7F'E÷F–ÖW%÷F–ÖR"ÓâvÖTWfVçEG—S£¥&W7F'EF–ÖW%F–ÖRÀÐ¢'v–æÆ–Ö—Eö6†ævVB"ÓâvÖTWfVçEG—S£¥v–äÆ–Ö—D6†ævVBÀÐ¢'v–çæVÅ÷6†÷u÷66÷&W2"ÓâvÖTWfVçEG—S£¥v–åæVÅ6†÷u66÷&W2ÀÐ¢'F÷÷7G&V×5÷&WVW7Eöf–æ—6†VB"ÓâvÖTWfVçEG—S£¥F÷7G&V×5&WVW7Df–æ—6†VBÀÐ¢&6ö×WF—F—fU÷7FFUö6†ævVB"ÓâvÖTWfVçEG—S£¤6ö×WF—F—fU7FFT6†ævVBÀÐ¢&vÆö&Å÷v%öFF÷WFFVB"ÓâvÖTWfVçEG—S£¤vÆö&Åv$FFWFFVBÀÐ¢'7F÷÷vF6…ö6†ævVB"ÓâvÖTWfVçEG—S£¥7F÷vF6„6†ævVBÀÐ¢&G5÷7F÷"ÓâvÖTWfVçEG—S£¤G57F÷ÀÐ¢&G5÷67&VVç6†÷B"ÓâvÖTWfVçEG—S£¤G567&VVç6†÷BÀÐ¢'6†÷uöÖF6…÷7VÖÖ'’"ÓâvÖTWfVçEG—S£¥6†÷tÖF6…7VÖÖ'’ÀÐ¢&W‡W&–Væ6Uö6†ævVB"ÓâvÖTWfVçEG—S£¤W‡W&–Væ6T6†ævVBÀÐ¢&&Vv–å÷‡öÆW'"ÓâvÖTWfVçEG—S£¤&Vv–å‡ÆW'ÀÐ¢&ÖF6†Ö¶W%÷7FG5÷WFFVB"ÓâvÖTWfVçEG—S£¤ÖF6†Ö¶W%7FG5WFFVBÀÐ¢'&VÖF6…÷f÷FU÷W&–öEö÷fW""ÓâvÖTWfVçEG—S£¥&VÖF6…f÷FUW&–öD÷fW"ÀÐ¢'&VÖF6…öf–ÆVE÷Fõö7&VFR"ÓâvÖTWfVçEG—S£¥&VÖF6„f–ÆVEFô7&VFRÀÐ¢'Æ–W%÷&VÖF6…ö6†ævR"ÓâvÖTWfVçEG—S£¥Æ–W%&VÖF6„6†ævRÀÐ¢'–æu÷WFFVB"ÓâvÖTWfVçEG—S£¥–æuWFFVBÀÐ¢&Ö×7FG5÷WFFVB"ÓâvÖTWfVçEG—S£¤ÔÕ7FG5WFFVBÀÐ¢'Æ–W%öæW‡EöÖ÷f÷FUö6†ævR"ÓâvÖTWfVçEG—S£¥Æ–W$æW‡DÖf÷FT6†ævRÀÐ¢'f÷FUöÖ5ö6†ævVB"ÓâvÖTWfVçEG—S£¥f÷FTÖ46†ævVBÀÐ¢'&÷FõöFVeö6†ævVB"ÓâvÖTWfVçEG—S£¥&÷FôFVd6†ævVBÀÐ¢'Æ–W%öFöÖ–æF–öâ"ÓâvÖTWfVçEG—S£¥Æ–W$FöÖ–æF–öâÀÐ¢'Æ–W%÷&ö6¶WG6µ÷W6†VB"ÓâvÖTWfVçEG—S£¥Æ–W%&ö6¶WE6µW6†VBÀÐ¢'VW7E÷&WVW7B"ÓâvÖTWfVçEG—S£¥VW7E&WVW7BÀÐ¢'VW7E÷&W7öç6R"ÓâvÖTWfVçEG—S£¥VW7E&W7öç6RÀÐ¢'VW7E÷&öw&W72"ÓâvÖTWfVçEG—S£¥VW7E&öw&W72ÀÐ¢'&ö¦V7F–ÆU÷&VÖ÷fVB"ÓâvÖTWfVçEG—S£¥&ö¦V7F–ÆU&VÖ÷fVBÀÐ¢'VW7EöÖöFFö6†ævVB"ÓâvÖTWfVçEG—S£¥VW7DÖFF6†ævVBÀÐ¢&v5öF÷W6VE÷Æ–W%ö–væ—FVB"ÓâvÖTWfVçEG—S£¤v4F÷W6VEÆ–W$–væ—FVBÀÐ¢'VW7E÷GW&åö–å÷7FFR"ÓâvÖTWfVçEG—S£¥VW7EGW&ä–å7FFRÀÐ¢&—FV×5ö6¶æ÷vÆVFvVB"ÓâvÖTWfVçEG—S£¤—FV×46¶æ÷vÆVFvVBÀÐ¢&6W%ö¶–ÆÆVB"ÓâvÖTWfVçEG—S£¤6W$¶–ÆÆVBÀÐ¢&Ö–æÖVçU÷7F&–Æ—¦VB"ÓâvÖTWfVçEG—S£¤Ö–äÖVçU7F&–Æ—¦VBÀÐ¢'v÷&ÆE÷7FGW5ö6†ævVB"ÓâvÖTWfVçEG—S£¥v÷&ÆE7FGW46†ævVBÀÐ¢&†ÇGe÷7FGW2"ÓâvÖTWfVçEG—S£¤„ÅEe7FGW2ÀÐ¢&†ÇGeö6ÖW&Öâ"ÓâvÖTWfVçEG—S£¤„ÅEd6ÖW&ÖâÀÐ¢&†ÇGe÷&æµö6ÖW&"ÓâvÖTWfVçEG—S£¤„ÅEe&æ´6ÖW&ÀÐ¢&†ÇGe÷&æµöVçF—G’"ÓâvÖTWfVçEG—S£¤„ÅEe&æ´VçF—G’ÀÐ¢&†ÇGeöf—†VB"ÓâvÖTWfVçEG—S£¤„ÅEdf—†VBÀÐ¢&†ÇGeö6†6R"ÓâvÖTWfVçEG—S£¤„ÅEd6†6RÀÐ¢&†ÇGeöÖW76vR"ÓâvÖTWfVçEG—S£¤„ÅEdÖW76vRÀÐ¢&†ÇGe÷F—FÆR"ÓâvÖTWfVçEG—S£¤„ÅEeF—FÆRÀÐ¢&†ÇGeö6†B"ÓâvÖTWfVçEG—S£¤„ÅEd6†BÀÐ¢'&WÆ•÷7F'G&V6÷&B"ÓâvÖTWfVçEG—S£¥&WÆ•7F'E&V6÷&BÀÐ¢'&WÆ•÷6W76–öæ–æfò"ÓâvÖTWfVçEG—S£¥&WÆ•6W76–öä–æfòÀÐ¢'&WÆ•öVæG&V6÷&B"ÓâvÖTWfVçEG—S£¥&WÆ”VæE&V6÷&BÀÐ¢'&WÆ•÷&WÆ—6f–Æ&ÆR"ÓâvÖTWfVçEG—S£¥&WÆ•&WÆ—4f–Æ&ÆRÀÐ¢'&WÆ•÷6W'fW&W'&÷""ÓâvÖTWfVçEG—S£¥&WÆ•6W'fW$W'&÷"ÀÐ¢G’ÓâvÖTWfVçEG—S£¥Væ¶æ÷vâ‡G’æ–çFò‚’’ÀÐ¢ÐÐ¢ÐÐ¢V"fâ5÷7G"‚g6VÆb’Óâg7G"°Ð¢ÖF6‚6VÆb°Ð¢vÖTWfVçEG—S£¥6W'fW%7vâÓâ'6W'fW%÷7vâ"ÀÐ¢vÖTWfVçEG—S£¥6W'fW$6†ævTÆWfVÄf–ÆVBÓâ'6W'fW%ö6†ævVÆWfVÅöf–ÆVB"ÀÐ¢vÖTWfVçEG—S£¥6W'fW%6‡WFF÷vâÓâ'6W'fW%÷6‡WFF÷vâ"ÀÐ¢vÖTWfVçEG—S£¥6W'fW$7f"Óâ'6W'fW%ö7f""ÀÐ¢vÖTWfVçEG—S£¥6W'fW$ÖW76vRÓâ'6W'fW%öÖW76vR"ÀÐ¢vÖTWfVçEG—S£¥6W'fW$FD&âÓâ'6W'fW%öFF&â"ÀÐ¢vÖTWfVçEG—S£¥6W'fW%&VÖ÷fT&âÓâ'6W'fW%÷&VÖ÷fV&â"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6öææV7BÓâ'Æ–W%ö6öææV7B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6öææV7D6Æ–VçBÓâ'Æ–W%ö6öææV7Eö6Æ–VçB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$–æfòÓâ'Æ–W%ö–æfò"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$F—66öææV7BÓâ'Æ–W%öF—66öææV7B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$7F—fFRÓâ'Æ–W%ö7F—fFR"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%6’Óâ'Æ–W%÷6’"ÀÐ¢vÖTWfVçEG—S£¤6Æ–VçDF—66öææV7BÓâ&6Æ–VçEöF—66öææV7B"ÀÐ¢vÖTWfVçEG—S£¤6Æ–VçD&Vv–ä6öææV7BÓâ&6Æ–VçEö&Vv–æ6öææV7B"ÀÐ¢vÖTWfVçEG—S£¤6Æ–VçD6öææV7FVBÓâ&6Æ–VçEö6öææV7FVB"ÀÐ¢vÖTWfVçEG—S£¤6Æ–VçDgVÆÄ6öææV7BÓâ&6Æ–VçEögVÆÆ6öææV7B"ÀÐ¢vÖTWfVçEG—S£¤†÷7EV—BÓâ&†÷7E÷V—B"ÀÐ¢vÖTWfVçEG—S£¥FVÔ–æfòÓâ'FVÕö–æfò"ÀÐ¢vÖTWfVçEG—S£¥FVÕ66÷&RÓâ'FVÕ÷66÷&R"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”'&öF67DVF–òÓâ'FV×Æ•ö'&öF67EöVF–ò"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%FVÒÓâ'Æ–W%÷FVÒ"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6Æ72Óâ'Æ–W%ö6Æ72"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$FVF‚Óâ'Æ–W%öFVF‚"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$‡W'BÓâ'Æ–W%ö‡W'B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6†BÓâ'Æ–W%ö6†B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%66÷&RÓâ'Æ–W%÷66÷&R"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%7vâÓâ'Æ–W%÷7vâ"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%6†ö÷BÓâ'Æ–W%÷6†ö÷B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%W6RÓâ'Æ–W%÷W6R"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6†ævTæÖRÓâ'Æ–W%ö6†ævVæÖR"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†–çDÖW76vRÓâ'Æ–W%ö†–çFÖW76vR"ÀÐ¢vÖTWfVçEG—S£¤&6UÆ–W%FVÆW÷'FVBÓâ&&6U÷Æ–W%÷FVÆW÷'FVB"ÀÐ¢vÖTWfVçEG—S£¤vÖT–æ—BÓâ&vÖUö–æ—B"ÀÐ¢vÖTWfVçEG—S£¤vÖTæWtÖÓâ&vÖUöæWvÖ"ÀÐ¢vÖTWfVçEG—S£¤vÖU7F'BÓâ&vÖU÷7F'B"ÀÐ¢vÖTWfVçEG—S£¤vÖTVæBÓâ&vÖUöVæB"ÀÐ¢vÖTWfVçEG—S£¥&÷VæE7F'BÓâ'&÷VæE÷7F'B"ÀÐ¢vÖTWfVçEG—S£¥&÷VæDVæBÓâ'&÷VæEöVæB"ÀÐ¢vÖTWfVçEG—S£¤vÖTÖW76vRÓâ&vÖUöÖW76vR"ÀÐ¢vÖTWfVçEG—S£¤'&V´'&V¶&ÆRÓâ&'&Vµö'&V¶&ÆR"ÀÐ¢vÖTWfVçEG—S£¤'&Vµ&÷Óâ&'&Vµ÷&÷"ÀÐ¢vÖTWfVçEG—S£¤VçF—G”¶–ÆÆVBÓâ&VçF—G•ö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¤&öçW5WFFVBÓâ&&öçW5÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¤6†–WfVÖVçDWfVçBÓâ&6†–WfVÖVçEöWfVçB"ÀÐ¢vÖTWfVçEG—S£¤6†–WfVÖVçD–æ7&VÖVçBÓâ&6†–WfVÖVçEö–æ7&VÖVçB"ÀÐ¢vÖTWfVçEG—S£¥‡—6wVå–6·WÓâ'‡—6wVå÷–6·W"ÀÐ¢vÖTWfVçEG—S£¤fÆ&T–væ—FTç2Óâ&fÆ&Uö–væ—FUöç2"ÀÐ¢vÖTWfVçEG—S£¤†VÆ–6÷FW$w&VæFUVçDÖ—72Óâ&†VÆ–6÷FW%öw&VæFU÷VçEöÖ—72"ÀÐ¢vÖTWfVçEG—S£¥W6W$FFF÷væÆöFVBÓâ'W6W%öFFöF÷væÆöFVB"ÀÐ¢vÖTWfVçEG—S£¥&vFöÆÄF—76öÇfVBÓâ'&vFöÆÅöF—76öÇfVB"ÀÐ¢vÖTWfVçEG—S£¤„ÅEd6†ævVDÖöFRÓâ&†ÇGeö6†ævVEöÖöFR"ÀÐ¢vÖTWfVçEG—S£¤„ÅEd6†ævVEF&vWBÓâ&†ÇGeö6†ævVE÷F&vWB"ÀÐ¢vÖTWfVçEG—S£¥f÷FTVæFVBÓâ'f÷FUöVæFVB"ÀÐ¢vÖTWfVçEG—S£¥f÷FU7F'FVBÓâ'f÷FU÷7F'FVB"ÀÐ¢vÖTWfVçEG—S£¥f÷FT6†ævVBÓâ'f÷FUö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥f÷FU76VBÓâ'f÷FU÷76VB"ÀÐ¢vÖTWfVçEG—S£¥f÷FTf–ÆVBÓâ'f÷FUöf–ÆVB"ÀÐ¢vÖTWfVçEG—S£¥f÷FT67BÓâ'f÷FUö67B"ÀÐ¢vÖTWfVçEG—S£¥f÷FT÷F–öç2Óâ'f÷FUö÷F–öç2"ÀÐ¢vÖTWfVçEG—S£¥&WÆ•6fVBÓâ'&WÆ•÷6fVB"ÀÐ¢vÖTWfVçEG—S£¤VçFW&VEW&f÷&Öæ6TÖöFRÓâ&VçFW&VE÷W&f÷&Öæ6UöÖöFR"ÀÐ¢vÖTWfVçEG—S£¤'&÷w6U&WÆ—2Óâ&'&÷w6U÷&WÆ—2"ÀÐ¢vÖTWfVçEG—S£¥&WÆ•–÷WGV&U7FG2Óâ'&WÆ•÷–÷WGV&U÷7FG2"ÀÐ¢vÖTWfVçEG—S£¤–çfVçF÷'•WFFVBÓâ&–çfVçF÷'•÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¤6'EWFFVBÓâ&6'E÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¥7F÷&U&–6U6†VWEWFFVBÓâ'7F÷&U÷&–6W6†VWE÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¤V6öä–çfVçF÷'”6öææV7FVBÓâ&V6öåö–çfVçF÷'•ö6öææV7FVB"ÀÐ¢vÖTWfVçEG—S£¤—FVÕ66†VÖ–æ—F–Æ—¦VBÓâ&—FVÕ÷66†VÖö–æ—F–Æ—¦VB"ÀÐ¢vÖTWfVçEG—S£¤v4æWu6W76–öâÓâ&v5öæWu÷6W76–öâ"ÀÐ¢vÖTWfVçEG—S£¤v4Æ÷7E6W76–öâÓâ&v5öÆ÷7E÷6W76–öâ"ÀÐ¢vÖTWfVçEG—S£¤–çG&ôf–æ—6‚Óâ&–çG&õöf–æ—6‚"ÀÐ¢vÖTWfVçEG—S£¤–çG&ôæW‡D6ÖW&Óâ&–çG&õöæW‡F6ÖW&"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6†ævT6Æ72Óâ'Æ–W%ö6†ævV6Æ72"ÀÐ¢vÖTWfVçEG—S£¥FdÖF–ÖU&VÖ–æ–ærÓâ'FeöÖ÷F–ÖU÷&VÖ–æ–ær"ÀÐ¢vÖTWfVçEG—S£¥FdvÖT÷fW"Óâ'FeövÖUö÷fW""ÀÐ¢vÖTWfVçEG—S£¤7FdfÆt6GW&VBÓâ&7FeöfÆuö6GW&VB"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çD–æ—F–Æ—¦VBÓâ&6öçG&öÇö–çEö–æ—F–Æ—¦VB"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT–ÖvW2Óâ&6öçG&öÇö–çE÷WFFV–ÖvW2"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEWFFTÆ–÷WBÓâ&6öçG&öÇö–çE÷WFFVÆ–÷WB"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT6–ærÓâ&6öçG&öÇö–çE÷WFFV6–ær"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT÷væW"Óâ&6öçG&öÇö–çE÷WFFV÷væW""ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çE7F'EF÷V6‚Óâ&6öçG&öÇö–çE÷7F'GF÷V6‚"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çDVæEF÷V6‚Óâ&6öçG&öÇö–çEöVæGF÷V6‚"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEVÇ6TVÆVÖVçBÓâ&6öçG&öÇö–çE÷VÇ6UöVÆVÖVçB"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çDf¶T6GW&RÓâ&6öçG&öÇö–çEöf¶Uö6GW&R"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W"Óâ&6öçG&öÇö–çEöf¶Uö6GW&Uö×VÇB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæE6VÆV7FVBÓâ'FV×Æ•÷&÷VæE÷6VÆV7FVB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæE7F'BÓâ'FV×Æ•÷&÷VæE÷7F'B"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæD7F—fRÓâ'FV×Æ•÷&÷VæEö7F—fR"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•v—F–æt&Vv–ç2Óâ'FV×Æ•÷v—F–æuö&Vv–ç2"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•v—F–ætVæG2Óâ'FV×Æ•÷v—F–æuöVæG2"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•v—F–æt&÷WEFôVæBÓâ'FV×Æ•÷v—F–æuö&÷WGFöVæB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&W7F'E&÷VæBÓâ'FV×Æ•÷&W7F'E÷&÷VæB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&VG•&W7F'BÓâ'FV×Æ•÷&VG•÷&W7F'B"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæE&W7F'E6V6öæG2Óâ'FV×Æ•÷&÷VæE÷&W7F'E÷6V6öæG2"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•FVÕ&VG’Óâ'FV×Æ•÷FVÕ÷&VG’"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæEv–âÓâ'FV×Æ•÷&÷VæE÷v–â"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•WFFUF–ÖW"Óâ'FV×Æ•÷WFFU÷F–ÖW""ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæE7FÆVÖFRÓâ'FV×Æ•÷&÷VæE÷7FÆVÖFR"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”÷fW'F–ÖT&Vv–âÓâ'FV×Æ•ö÷fW'F–ÖUö&Vv–â"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”÷fW'F–ÖTVæBÓâ'FV×Æ•ö÷fW'F–ÖUöVæB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•7VFFVäFVF„&Vv–âÓâ'FV×Æ•÷7VFFVæFVF…ö&Vv–â"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•7VFFVäFVF„VæBÓâ'FV×Æ•÷7VFFVæFVF…öVæB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”vÖT÷fW"Óâ'FV×Æ•övÖUö÷fW""ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”ÖF–ÖU&VÖ–æ–ærÓâ'FV×Æ•öÖ÷F–ÖU÷&VÖ–æ–ær"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•F–ÖW$fÆ6‚Óâ'FV×Æ•÷F–ÖW%öfÆ6‚"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•F–ÖW%F–ÖTFFVBÓâ'FV×Æ•÷F–ÖW%÷F–ÖUöFFVB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•ö–çE7F'D6GW&RÓâ'FV×Æ•÷ö–çE÷7F'F6GW&R"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•ö–çD6GW&VBÓâ'FV×Æ•÷ö–çEö6GW&VB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•ö–çDÆö6¶VBÓâ'FV×Æ•÷ö–çEöÆö6¶VB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•ö–çEVæÆö6¶VBÓâ'FV×Æ•÷ö–çE÷VæÆö6¶VB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”6GW&T'&ö¶VâÓâ'FV×Æ•ö6GW&Uö'&ö¶Vâ"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”6GW&T&Æö6¶VBÓâ'FV×Æ•ö6GW&Uö&Æö6¶VB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”fÆtWfVçBÓâ'FV×Æ•öfÆuöWfVçB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•v–åæVÂÓâ'FV×Æ•÷v–å÷æVÂ"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•FVÔ&Ææ6VEÆ–W"Óâ'FV×Æ•÷FVÖ&Ææ6VE÷Æ–W""ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•6WGWf–æ—6†VBÓâ'FV×Æ•÷6WGWöf–æ—6†VB"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”ÆW'BÓâ'FV×Æ•öÆW'B"ÀÐ¢vÖTWfVçEG—S£¥G&–æ–æt6ö×ÆWFRÓâ'G&–æ–æuö6ö×ÆWFR"ÀÐ¢vÖTWfVçEG—S£¥6†÷tg&VW¦UæVÂÓâ'6†÷uög&VW¦WæVÂ"ÀÐ¢vÖTWfVçEG—S£¤†–FTg&VW¦UæVÂÓâ&†–FUög&VW¦WæVÂ"ÀÐ¢vÖTWfVçEG—S£¤g&VW¦T6Õ7F'FVBÓâ&g&VW¦V6Õ÷7F'FVB"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævUFVÒÓâ&Æö6ÇÆ–W%ö6†ævWFVÒ"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W%66÷&T6†ævVBÓâ&Æö6ÇÆ–W%÷66÷&Uö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævT6Æ72Óâ&Æö6ÇÆ–W%ö6†ævV6Æ72"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W%&W7vâÓâ&Æö6ÇÆ–W%÷&W7vâ"ÀÐ¢vÖTWfVçEG—S£¤'V–ÆF–æt–æfô6†ævVBÓâ&'V–ÆF–æuö–æfõö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævTF—6wV—6RÓâ&Æö6ÇÆ–W%ö6†ævVF—6wV—6R"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$66÷VçD6†ævVBÓâ'Æ–W%ö66÷VçEö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥7•F&W6WBÓâ'7•÷F÷&W6WB"ÀÐ¢vÖTWfVçEG—S£¤fÆu7FGW5WFFRÓâ&fÆw7FGW5÷WFFR"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%7FG5WFFVBÓâ'Æ–W%÷7FG5÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¥Æ––æt6öÖÖVçF'’Óâ'Æ––æuö6öÖÖVçF'’"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6†&vTFWÆ÷–VBÓâ'Æ–W%ö6†&vVFWÆ÷–VB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$'V–ÇDö&¦V7BÓâ'Æ–W%ö'V–ÇFö&¦V7B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%Ww&FVDö&¦V7BÓâ'Æ–W%÷Ww&FVFö&¦V7B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6''”ö&¦V7BÓâ'Æ–W%ö6''–ö&¦V7B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$G&÷ö&¦V7BÓâ'Æ–W%öG&÷ö&¦V7B"ÀÐ¢vÖTWfVçEG—S£¤ö&¦V7E&VÖ÷fVBÓâ&ö&¦V7E÷&VÖ÷fVB"ÀÐ¢vÖTWfVçEG—S£¤ö&¦V7DFW7G&÷–VBÓâ&ö&¦V7EöFW7G&÷–VB"ÀÐ¢vÖTWfVçEG—S£¤ö&¦V7DFWFöæFVBÓâ&ö&¦V7EöFWFöæFVB"ÀÐ¢vÖTWfVçEG—S£¤6†–WfVÖVçDV&æVBÓâ&6†–WfVÖVçEöV&æVB"ÀÐ¢vÖTWfVçEG—S£¥7V5F&vWEWFFVBÓâ'7V5÷F&vWE÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¥F÷W&æÖVçE7FFUWFFRÓâ'F÷W&æÖVçE÷7FFWWFFR"ÀÐ¢vÖTWfVçEG—S£¥F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâÓâ'F÷W&æÖVçEöVæ&ÆV6÷VçFF÷vâ"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6ÆÆVDf÷$ÖVF–2Óâ'Æ–W%ö6ÆÆVFf÷&ÖVF–2"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6¶VDf÷$&ÆÂÓâ'Æ–W%ö6¶VFf÷&&ÆÂ"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$&V6ÖTö'6W'fW"Óâ&Æö6ÇÆ–W%ö&V6ÖVö'6W'fW""ÀÐ¢vÖTWfVçEG—S£¥Æ–W$–væ—FVD–çbÓâ'Æ–W%ö–væ—FVEö–çb"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$–væ—FVBÓâ'Æ–W%ö–væ—FVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$W‡F–æwV—6†VBÓâ'Æ–W%öW‡F–æwV—6†VB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%FVÆW÷'FVBÓâ'Æ–W%÷FVÆW÷'FVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†VÆVDÖVF–46ÆÂÓâ'Æ–W%ö†VÆVFÖVF–66ÆÂ"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$6†&vU&VG’Óâ&Æö6ÇÆ–W%ö6†&vW&VG’"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W%v–æDF÷vâÓâ&Æö6ÇÆ–W%÷v–æFF÷vâ"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$–çgVÆæVBÓâ'Æ–W%ö–çgVÆæVB"ÀÐ¢vÖTWfVçEG—S£¤W66÷'E7VVBÓâ&W66÷'E÷7VVB"ÀÐ¢vÖTWfVçEG—S£¤W66÷'E&öw&W72Óâ&W66÷'E÷&öw&W72"ÀÐ¢vÖTWfVçEG—S£¤W66÷'E&V6VFRÓâ&W66÷'E÷&V6VFR"ÀÐ¢vÖTWfVçEG—S£¤vÖUT”7F—fFVBÓâ&vÖWV•ö7F—fFVB"ÀÐ¢vÖTWfVçEG—S£¤vÖUT”†–FFVâÓâ&vÖWV•ö†–FFVâ"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$W66÷'E66÷&RÓâ'Æ–W%öW66÷'E÷66÷&R"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†VÄöä†—BÓâ'Æ–W%ö†VÆöæ†—B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%7FVÅ6æGf–6‚Óâ'Æ–W%÷7FVÇ6æGf–6‚"ÀÐ¢vÖTWfVçEG—S£¥6†÷t6Æ74Æ–÷WBÓâ'6†÷uö6Æ75öÆ–÷WB"ÀÐ¢vÖTWfVçEG—S£¥6†÷ug5æVÂÓâ'6†÷u÷g5÷æVÂ"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$FÖvVBÓâ'Æ–W%öFÖvVB"ÀÐ¢vÖTWfVçEG—S£¤&VæÆ–W$æ÷F–f–6F–öâÓâ&&Væ÷Æ–W%öæ÷F–f–6F–öâ"ÀÐ¢vÖTWfVçEG—S£¤&VæÖF6„Ö…7G&V²Óâ&&VæöÖF6…öÖ‡7G&V²"ÀÐ¢vÖTWfVçEG—S£¤&Væ&÷VæE7F'BÓâ&&Væ÷&÷VæE÷7F'B"ÀÐ¢vÖTWfVçEG—S£¤&Væv–åæVÂÓâ&&Væ÷v–å÷æVÂ"ÀÐ¢vÖTWfVçEG—S£¥fUv–åæVÂÓâ'fU÷v–å÷æVÂ"ÀÐ¢vÖTWfVçEG—S£¤—$F6‚Óâ&—%öF6‚"ÀÐ¢vÖTWfVçEG—S£¤ÆæFVBÓâ&ÆæFVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$FÖvTFöFvVBÓâ'Æ–W%öFÖvUöFöFvVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%7GVææVBÓâ'Æ–W%÷7GVææVB"ÀÐ¢vÖTWfVçEG—S£¥66÷WDw&æE6ÆÒÓâ'66÷WEöw&æE÷6ÆÒ"ÀÐ¢vÖTWfVçEG—S£¥66÷WE6ÆÖFöÆÄÆæFVBÓâ'66÷WE÷6ÆÖFöÆÅöÆæFVB"ÀÐ¢vÖTWfVçEG—S£¤'&÷t–×7BÓâ&'&÷uö–×7B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$¦&FVBÓâ'Æ–W%ö¦&FVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$¦&FVDfFRÓâ'Æ–W%ö¦&FVEöfFR"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%6†–VÆD&Æö6¶VBÓâ'Æ–W%÷6†–VÆEö&Æö6¶VB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%–ææVBÓâ'Æ–W%÷–ææVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†VÆVD'”ÖVF–2Óâ'Æ–W%ö†VÆVF'–ÖVF–2"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%6VDö&¦V7BÓâ'Æ–W%÷6VEöö&¦V7B"ÀÐ¢vÖTWfVçEG—S£¤—FVÔf÷VæBÓâ&—FVÕöf÷VæB"ÀÐ¢vÖTWfVçEG—S£¥6†÷tææ÷FF–öâÓâ'6†÷uöææ÷FF–öâ"ÀÐ¢vÖTWfVçEG—S£¤†–FTææ÷FF–öâÓâ&†–FUöææ÷FF–öâ"ÀÐ¢vÖTWfVçEG—S£¥÷7D–çfVçF÷'”Æ–6F–öâÓâ'÷7Eö–çfVçF÷'•öÆ–6F–öâ"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEVæÆö6µWFFVBÓâ&6öçG&öÇö–çE÷VæÆö6µ÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¤FWÆ÷”'Vfd&ææW"Óâ&FWÆ÷•ö'Vfeö&ææW""ÀÐ¢vÖTWfVçEG—S£¥Æ–W$'VfbÓâ'Æ–W%ö'Vfb"ÀÐ¢vÖTWfVçEG—S£¤ÖVF–4FVF‚Óâ&ÖVF–5öFVF‚"ÀÐ¢vÖTWfVçEG—S£¤÷fW'F–ÖTærÓâ&÷fW'F–ÖUöær"ÀÐ¢vÖTWfVçEG—S£¥FV×46†ævVBÓâ'FV×5ö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVåV×¶–äw&"Óâ&†ÆÆ÷vVVå÷V×¶–åöw&""ÀÐ¢vÖTWfVçEG—S£¥&ö6¶WD§V×Óâ'&ö6¶WEö§V×"ÀÐ¢vÖTWfVçEG—S£¥&ö6¶WD§V×ÆæFVBÓâ'&ö6¶WEö§V×öÆæFVB"ÀÐ¢vÖTWfVçEG—S£¥7F–6·”§V×Óâ'7F–6·•ö§V×"ÀÐ¢vÖTWfVçEG—S£¥7F–6·”§V×ÆæFVBÓâ'7F–6·•ö§V×öÆæFVB"ÀÐ¢vÖTWfVçEG—S£¥&ö6¶WE6´ÆVæ6‚Óâ'&ö6¶WG6µöÆVæ6‚"ÀÐ¢vÖTWfVçEG—S£¥&ö6¶WE6´ÆæFVBÓâ'&ö6¶WG6µöÆæFVB"ÀÐ¢vÖTWfVçEG—S£¤ÖVF–4FVfVæFVBÓâ&ÖVF–5öFVfVæFVB"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$†VÆVBÓâ&Æö6ÇÆ–W%ö†VÆVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$FW7G&÷–VE—T&öÖ"Óâ'Æ–W%öFW7G&÷–VE÷—V&öÖ""ÀÐ¢vÖTWfVçEG—S£¤ö&¦V7DFVfÆV7FVBÓâ&ö&¦V7EöFVfÆV7FVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$×gÓâ'Æ–W%ö×g"ÀÐ¢vÖTWfVçEG—S£¥&–E7väÖö"Óâ'&–E÷7våöÖö""ÀÐ¢vÖTWfVçEG—S£¥&–E7vå7VBÓâ'&–E÷7vå÷7VB"ÀÐ¢vÖTWfVçEG—S£¤æd&Æö6¶VBÓâ&æeö&Æö6¶VB"ÀÐ¢vÖTWfVçEG—S£¥F…G&6µ76VBÓâ'F…÷G&6µ÷76VB"ÀÐ¢vÖTWfVçEG—S£¤çVÔ6W'46†ævVBÓâ&çVÕö6W'5ö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%&VvVæW&FRÓâ'Æ–W%÷&VvVæW&FR"ÀÐ¢vÖTWfVçEG—S£¥WFFU7FGW4—FVÒÓâ'WFFU÷7FGW5ö—FVÒ"ÀÐ¢vÖTWfVçEG—S£¥7FG5&W6WE&÷VæBÓâ'7FG5÷&W6WG&÷VæB"ÀÐ¢vÖTWfVçEG—S£¥66÷&U7FG467V×VÆFVEWFFRÓâ'66÷&W7FG5ö67V×VÆFVE÷WFFR"ÀÐ¢vÖTWfVçEG—S£¥66÷&U7FG467V×VÆFVE&W6WBÓâ'66÷&W7FG5ö67V×VÆFVE÷&W6WB"ÀÐ¢vÖTWfVçEG—S£¤6†–WfVÖVçDV&æVDÆö6ÂÓâ&6†–WfVÖVçEöV&æVEöÆö6Â"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†VÆVBÓâ'Æ–W%ö†VÆVB"ÀÐ¢vÖTWfVçEG—S£¤'V–ÆF–æt†VÆVBÓâ&'V–ÆF–æuö†VÆVB"ÀÐ¢vÖTWfVçEG—S£¤—FVÕ–6·WÓâ&—FVÕ÷–6·W"ÀÐ¢vÖTWfVçEG—S£¤GVVÅ7FGW2Óâ&GVVÅ÷7FGW2"ÀÐ¢vÖTWfVçEG—S£¤f—6„æ÷F–6RÓâ&f—6…öæ÷F–6R"ÀÐ¢vÖTWfVçEG—S£¤f—6„æ÷F–6T&ÒÓâ&f—6…öæ÷F–6Uõö&Ò"ÀÐ¢vÖTWfVçEG—S£¥6Ææ÷F–6RÓâ'6Æöæ÷F–6R"ÀÐ¢vÖTWfVçEG—S£¥F‡&÷v&ÆT†—BÓâ'F‡&÷v&ÆUö†—B"ÀÐ¢vÖTWfVçEG—S£¥V×¶–äÆ÷&E7VÖÖöæVBÓâ'V×¶–åöÆ÷&E÷7VÖÖöæVB"ÀÐ¢vÖTWfVçEG—S£¥V×¶–äÆ÷&D¶–ÆÆVBÓâ'V×¶–åöÆ÷&Eö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¤ÖW&6×W57VÖÖöæVBÓâ&ÖW&6×W5÷7VÖÖöæVB"ÀÐ¢vÖTWfVçEG—S£¤ÖW&6×W4¶–ÆÆVBÓâ&ÖW&6×W5ö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¤ÖW&6×W4W66Uv&æ–ærÓâ&ÖW&6×W5öW66U÷v&æ–ær"ÀÐ¢vÖTWfVçEG—S£¤ÖW&6×W4W66VBÓâ&ÖW&6×W5öW66VB"ÀÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷757VÖÖöæVBÓâ&W–V&ÆÅö&÷75÷7VÖÖöæVB"ÀÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷757GVææVBÓâ&W–V&ÆÅö&÷75÷7GVææVB"ÀÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷74¶–ÆÆVBÓâ&W–V&ÆÅö&÷75ö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷74¶–ÆÆW"Óâ&W–V&ÆÅö&÷75ö¶–ÆÆW""ÀÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷74W66T–ÖÖ–æVçBÓâ&W–V&ÆÅö&÷75öW66Uö–ÖÖ–æVçB"ÀÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷74W66VBÓâ&W–V&ÆÅö&÷75öW66VB"ÀÐ¢vÖTWfVçEG—S£¤ç4‡W'BÓâ&ç5ö‡W'B"ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEF–ÖW%WFFVBÓâ&6öçG&öÇö–çE÷F–ÖW%÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†–v„f—fU7F'BÓâ'Æ–W%ö†–v†f—fU÷7F'B"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†–v„f—fT6æ6VÂÓâ'Æ–W%ö†–v†f—fUö6æ6VÂ"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†–v„f—fU7V66W72Óâ'Æ–W%ö†–v†f—fU÷7V66W72"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$&öçW5ö–çG2Óâ'Æ–W%ö&öçW7ö–çG2"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%Ww&FVBÓâ'Æ–W%÷Ww&FVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$'W–&6²Óâ'Æ–W%ö'W–&6²"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%W6VE÷vW%W&÷GFÆRÓâ'Æ–W%÷W6VE÷÷vW'Wö&÷GFÆR"ÀÐ¢vÖTWfVçEG—S£¤6‡&—7FÖ4v–gDw&"Óâ&6‡&—7FÖ5öv–gEöw&""ÀÐ¢vÖTWfVçEG—S£¥Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæRÓâ'Æ–W%ö¶–ÆÆVEö6†–WfVÖVçE÷¦öæR"ÀÐ¢vÖTWfVçEG—S£¥'G•WFFVBÓâ''G•÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¥'G•&Vd6†ævVBÓâ''G•÷&Veö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥'G”7&—FW&–6†ævVBÓâ''G•ö7&—FW&–ö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥'G”–çf—FW46†ævVBÓâ''G•ö–çf—FW5ö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥'G•VWVU7FFT6†ævVBÓâ''G•÷VWVU÷7FFUö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥'G”6†BÓâ''G•ö6†B"ÀÐ¢vÖTWfVçEG—S£¥'G”ÖVÖ&W$¦ö–âÓâ''G•öÖVÖ&W%ö¦ö–â"ÀÐ¢vÖTWfVçEG—S£¥'G”ÖVÖ&W$ÆVfRÓâ''G•öÖVÖ&W%öÆVfR"ÀÐ¢vÖTWfVçEG—S£¤ÖF6„–çf—FW5WFFVBÓâ&ÖF6…ö–çf—FW5÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¤Æö&'•WFFVBÓâ&Æö&'•÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¤×fÔÖ—76–öåWFFRÓâ&×fÕöÖ—76–öå÷WFFR"ÀÐ¢vÖTWfVçEG—S£¥&V6Æ7VÆFT†öÆ–F—2Óâ'&V6Æ7VÆFUö†öÆ–F—2"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$7W'&Væ7”6†ævVBÓâ'Æ–W%ö7W'&Væ7•ö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤Föö×6F•&ö6¶WD÷VâÓâ&Föö×6F•÷&ö6¶WEö÷Vâ"ÀÐ¢vÖTWfVçEG—S£¥&VÖ÷fTæVÖW6—5&VÆF–öç6†—2Óâ'&VÖ÷fUöæVÖW6—5÷&VÆF–öç6†—2"ÀÐ¢vÖTWfVçEG—S£¤×fÔ7&VF—D&öçW5vfRÓâ&×fÕö7&VF—F&öçW5÷vfR"ÀÐ¢vÖTWfVçEG—S£¤×fÔ7&VF—D&öçW4ÆÂÓâ&×fÕö7&VF—F&öçW5öÆÂ"ÀÐ¢vÖTWfVçEG—S£¤×fÔ7&VF—D&öçW4ÆÄGfæ6VBÓâ&×fÕö7&VF—F&öçW5öÆÅöGfæ6VB"ÀÐ¢vÖTWfVçEG—S£¤×fÕV–6µ6VçG'•Ww&FRÓâ&×fÕ÷V–6µ÷6VçG'•÷Ww&FR"ÀÐ¢vÖTWfVçEG—S£¤×fÕFæ´FW7G&÷–VD'•Æ–W'2Óâ&×fÕ÷FæµöFW7G&÷–VEö'•÷Æ–W'2"ÀÐ¢vÖTWfVçEG—S£¤×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ"Óâ&×fÕö¶–ÆÅ÷&ö&÷EöFVÆ—fW&–æuö&öÖ""ÀÐ¢vÖTWfVçEG—S£¤×fÕ–6·W7W'&Væ7’Óâ&×fÕ÷–6·Wö7W'&Væ7’"ÀÐ¢vÖTWfVçEG—S£¤×fÔ&öÖ$6'&–W$¶–ÆÆVBÓâ&×fÕö&öÖ%ö6'&–W%ö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¤×fÕ6VçG'”'W7FW$FWFöæFRÓâ&×fÕ÷6VçG'–'W7FW%öFWFöæFR"ÀÐ¢vÖTWfVçEG—S£¤×fÕ66÷WDÖ&¶VDf÷$FVF‚Óâ&×fÕ÷66÷WEöÖ&¶VEöf÷%öFVF‚"ÀÐ¢vÖTWfVçEG—S£¤×fÔÖVF–5÷vW%W6†&VBÓâ&×fÕöÖVF–5÷÷vW'W÷6†&VB"ÀÐ¢vÖTWfVçEG—S£¤×fÔ&Vv–åvfRÓâ&×fÕö&Vv–å÷vfR"ÀÐ¢vÖTWfVçEG—S£¤×fÕvfT6ö×ÆWFRÓâ&×fÕ÷vfUö6ö×ÆWFR"ÀÐ¢vÖTWfVçEG—S£¤×fÔÖ—76–öä6ö×ÆWFRÓâ&×fÕöÖ—76–öåö6ö×ÆWFR"ÀÐ¢vÖTWfVçEG—S£¤×fÔ&öÖ%&W6WD'•Æ–W"Óâ&×fÕö&öÖ%÷&W6WEö'•÷Æ–W""ÀÐ¢vÖTWfVçEG—S£¤×fÔ&öÖ$Æ&ÕG&–vvW&VBÓâ&×fÕö&öÖ%öÆ&Õ÷G&–vvW&VB"ÀÐ¢vÖTWfVçEG—S£¤×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W"Óâ&×fÕö&öÖ%öFWÆ÷•÷&W6WEö'•÷Æ–W""ÀÐ¢vÖTWfVçEG—S£¤×fÕvfTf–ÆVBÓâ&×fÕ÷vfUöf–ÆVB"ÀÐ¢vÖTWfVçEG—S£¤×fÕ&W6WE7FG2Óâ&×fÕ÷&W6WE÷7FG2"ÀÐ¢vÖTWfVçEG—S£¤FÖvU&W6—7FVBÓâ&FÖvU÷&W6—7FVB"ÀÐ¢vÖTWfVçEG—S£¥&Wf—fUÆ–W$æ÷F–g’Óâ'&Wf—fU÷Æ–W%öæ÷F–g’"ÀÐ¢vÖTWfVçEG—S£¥&Wf—fUÆ–W%7F÷VBÓâ'&Wf—fU÷Æ–W%÷7F÷VB"ÀÐ¢vÖTWfVçEG—S£¥&Wf—fUÆ–W$6ö×ÆWFRÓâ'&Wf—fU÷Æ–W%ö6ö×ÆWFR"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%GW&æVEFôv†÷7BÓâ'Æ–W%÷GW&æVE÷Fõöv†÷7B"ÀÐ¢vÖTWfVçEG—S£¤ÖVF–wVå6†–VÆD&Æö6¶VDFÖvRÓâ&ÖVF–wVå÷6†–VÆEö&Æö6¶VEöFÖvR"ÀÐ¢vÖTWfVçEG—S£¤×fÔGevfT6ö×ÆWFTæôvFW2Óâ&×fÕöGe÷vfUö6ö×ÆWFUöæõövFW2"ÀÐ¢vÖTWfVçEG—S£¤×fÕ6æ—W$†VG6†÷D7W'&Væ7’Óâ&×fÕ÷6æ—W%ö†VG6†÷Eö7W'&Væ7’"ÀÐ¢vÖTWfVçEG—S£¤×fÔÖææ†GFå—BÓâ&×fÕöÖææ†GFå÷—B"ÀÐ¢vÖTWfVçEG—S£¤fÆt6'&–VD–äFWFV7F–öå¦öæRÓâ&fÆuö6'&–VEö–åöFWFV7F–öå÷¦öæR"ÀÐ¢vÖTWfVçEG—S£¤×fÔGevfT¶–ÆÆVE7GVå&F–òÓâ&×fÕöGe÷vfUö¶–ÆÆVE÷7GVå÷&F–ò"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$F—&V7D†—E7GVâÓâ'Æ–W%öF—&V7F†—E÷7GVâ"ÀÐ¢vÖTWfVçEG—S£¤×fÕ6VçG'”'W7FW$¶–ÆÆVBÓâ&×fÕ÷6VçG'–'W7FW%ö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¥Ww&FW4f–ÆT6†ævVBÓâ'Ww&FW5öf–ÆUö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥&EFVÕö–çG46†ævVBÓâ'&E÷FVÕ÷ö–çG5ö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥&E'VÆW57FFT6†ævVBÓâ'&E÷'VÆW5÷7FFUö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥&E&ö&÷D¶–ÆÆVBÓâ'&E÷&ö&÷Eö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¥&E&ö&÷D–×7BÓâ'&E÷&ö&÷Eö–×7B"ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&U&÷VæEF–ÖTÆVgBÓâ'FV×Æ•÷&U÷&÷VæE÷F–ÖUöÆVgB"ÀÐ¢vÖTWfVçEG—S£¥&6‡WFTFWÆ÷’Óâ'&6‡WFUöFWÆ÷’"ÀÐ¢vÖTWfVçEG—S£¥&6‡WFT†öÇ7FW"Óâ'&6‡WFUö†öÇ7FW""ÀÐ¢vÖTWfVçEG—S£¤¶–ÆÅ&Vf–ÆÇ4ÖWFW"Óâ&¶–ÆÅ÷&Vf–ÆÇ5öÖWFW""ÀÐ¢vÖTWfVçEG—S£¥'5FVçDWfVçBÓâ''5÷FVçEöWfVçB"ÀÐ¢vÖTWfVçEG—S£¤6öæv¶–ÆÂÓâ&6öævö¶–ÆÂ"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$–æ—F–Å7vâÓâ'Æ–W%ö–æ—F–Å÷7vâ"ÀÐ¢vÖTWfVçEG—S£¤6ö×WF—F—fUf–7F÷'’Óâ&6ö×WF—F—fU÷f–7F÷'’"ÀÐ¢vÖTWfVçEG—S£¤6ö×WF—F—fU7FG5WFFRÓâ&6ö×WF—F—fU÷7FG5÷WFFR"ÀÐ¢vÖTWfVçEG—S£¤Ö–æ”vÖUv–âÓâ&Ö–æ–vÖU÷v–â"ÀÐ¢vÖTWfVçEG—S£¥6VçG'”öävô7F—fRÓâ'6VçG'•ööåövõö7F—fR"ÀÐ¢vÖTWfVçEG—S£¤GV6µ‡ÆWfVÅWÓâ&GV6µ÷‡öÆWfVÅ÷W"ÀÐ¢vÖTWfVçEG—S£¥VW7DÆöt÷VæVBÓâ'VW7FÆöuö÷VæVB"ÀÐ¢vÖTWfVçEG—S£¥66†VÖWFFVBÓâ'66†VÖ÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W%–6·WvVöâÓâ&Æö6ÇÆ–W%÷–6·W÷vVöâ"ÀÐ¢vÖTWfVçEG—S£¥&EÆ–W%66÷&Uö–çG2Óâ'&E÷Æ–W%÷66÷&U÷ö–çG2"ÀÐ¢vÖTWfVçEG—S£¤FVÖöÖäFWE7F–6¶–W2Óâ&FVÖöÖåöFWE÷7F–6¶–W2"ÀÐ¢vÖTWfVçEG—S£¥VW7Dö&¦V7F—fT6ö×ÆWFVBÓâ'VW7Eöö&¦V7F—fUö6ö×ÆWFVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%66÷&T6†ævVBÓâ'Æ–W%÷66÷&Uö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤¶–ÆÆVD6–æuÆ–W"Óâ&¶–ÆÆVEö6–æu÷Æ–W""ÀÐ¢vÖTWfVçEG—S£¤Vçf—&öæÖVçFÄFVF‚Óâ&Vçf—&öæÖVçFÅöFVF‚"ÀÐ¢vÖTWfVçEG—S£¥&ö¦V7F–ÆTF—&V7D†—BÓâ'&ö¦V7F–ÆUöF—&V7Eö†—B"ÀÐ¢vÖTWfVçEG—S£¥74vWBÓâ'75övWB"ÀÐ¢vÖTWfVçEG—S£¥7566÷&RÓâ'75÷66÷&R"ÀÐ¢vÖTWfVçEG—S£¥74g&VRÓâ'75ög&VR"ÀÐ¢vÖTWfVçEG—S£¥75746Vv‡BÓâ'75÷75ö6Vv‡B"ÀÐ¢vÖTWfVçEG—S£¥74&ÆÅ7FöÆVâÓâ'75ö&ÆÅ÷7FöÆVâ"ÀÐ¢vÖTWfVçEG—S£¥74&ÆÄ&Æö6¶VBÓâ'75ö&ÆÅö&Æö6¶VB"ÀÐ¢vÖTWfVçEG—S£¤FÖvU&WfVçFVBÓâ&FÖvU÷&WfVçFVB"ÀÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVä&÷74¶–ÆÆVBÓâ&†ÆÆ÷vVVåö&÷75ö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¤W66VDÆö÷D—6ÆæBÓâ&W66VEöÆö÷Eö—6ÆæB"ÀÐ¢vÖTWfVçEG—S£¥FvvVEÆ–W$4—BÓâ'FvvVE÷Æ–W%ö5ö—B"ÀÐ¢vÖTWfVçEG—S£¤ÖW&6×W57GVææVBÓâ&ÖW&6×W5÷7GVææVB"ÀÐ¢vÖTWfVçEG—S£¤ÖW&6×W5&÷f÷VæBÓâ&ÖW&6×W5÷&÷öf÷VæB"ÀÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVBÓâ&†ÆÆ÷vVVå÷6¶VÆWFöåö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¥6¶VÆWFöä¶–ÆÆVEVW7BÓâ'6¶VÆWFöåö¶–ÆÆVE÷VW7B"ÀÐ¢vÖTWfVçEG—S£¥6¶VÆWFöä¶–æt¶–ÆÆVEVW7BÓâ'6¶VÆWFöåö¶–æuö¶–ÆÆVE÷VW7B"ÀÐ¢vÖTWfVçEG—S£¤W66T†VÆÂÓâ&W66Uö†VÆÂ"ÀÐ¢vÖTWfVçEG—S£¤7&÷757V7G&Ä'&–FvRÓâ&7&÷75÷7V7G&Åö'&–FvR"ÀÐ¢vÖTWfVçEG—S£¤Ö–æ”vÖUvöâÓâ&Ö–æ–vÖU÷vöâ"ÀÐ¢vÖTWfVçEG—S£¥&W7väv†÷7BÓâ'&W7våöv†÷7B"ÀÐ¢vÖTWfVçEG—S£¤¶–ÆÄ–ä†VÆÂÓâ&¶–ÆÅö–åö†VÆÂ"ÀÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVäGV6´6öÆÆV7FVBÓâ&†ÆÆ÷vVVåöGV6µö6öÆÆV7FVB"ÀÐ¢vÖTWfVçEG—S£¥7V6–Å66÷&RÓâ'7V6–Å÷66÷&R"ÀÐ¢vÖTWfVçEG—S£¥FVÔÆVFW$¶–ÆÆVBÓâ'FVÕöÆVFW%ö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVBÓâ&†ÆÆ÷vVVå÷6÷VÅö6öÆÆV7FVB"ÀÐ¢vÖTWfVçEG—S£¥&V6Æ7VÆFUG'V6RÓâ'&V6Æ7VÆFU÷G'V6R"ÀÐ¢vÖTWfVçEG—S£¤FVE&–ævW$6†VDFVF‚Óâ&FVG&–ævW%ö6†VEöFVF‚"ÀÐ¢vÖTWfVçEG—S£¤7&÷76&÷t†VÂÓâ&7&÷76&÷uö†VÂ"ÀÐ¢vÖTWfVçEG—S£¤FÖvTÖ—F–vFVBÓâ&FÖvUöÖ—F–vFVB"ÀÐ¢vÖTWfVçEG—S£¥–ÆöEW6†VBÓâ'–ÆöE÷W6†VB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$&æFöæVDÖF6‚Óâ'Æ–W%ö&æFöæVEöÖF6‚"ÀÐ¢vÖTWfVçEG—S£¤6ÄG&vÆ–æRÓâ&6ÅöG&vÆ–æR"ÀÐ¢vÖTWfVçEG—S£¥&W7F'EF–ÖW%F–ÖRÓâ'&W7F'E÷F–ÖW%÷F–ÖR"ÀÐ¢vÖTWfVçEG—S£¥v–äÆ–Ö—D6†ævVBÓâ'v–æÆ–Ö—Eö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥v–åæVÅ6†÷u66÷&W2Óâ'v–çæVÅ÷6†÷u÷66÷&W2"ÀÐ¢vÖTWfVçEG—S£¥F÷7G&V×5&WVW7Df–æ—6†VBÓâ'F÷÷7G&V×5÷&WVW7Eöf–æ—6†VB"ÀÐ¢vÖTWfVçEG—S£¤6ö×WF—F—fU7FFT6†ævVBÓâ&6ö×WF—F—fU÷7FFUö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤vÆö&Åv$FFWFFVBÓâ&vÆö&Å÷v%öFF÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¥7F÷vF6„6†ævVBÓâ'7F÷÷vF6…ö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤G57F÷Óâ&G5÷7F÷"ÀÐ¢vÖTWfVçEG—S£¤G567&VVç6†÷BÓâ&G5÷67&VVç6†÷B"ÀÐ¢vÖTWfVçEG—S£¥6†÷tÖF6…7VÖÖ'’Óâ'6†÷uöÖF6…÷7VÖÖ'’"ÀÐ¢vÖTWfVçEG—S£¤W‡W&–Væ6T6†ævVBÓâ&W‡W&–Væ6Uö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤&Vv–å‡ÆW'Óâ&&Vv–å÷‡öÆW'"ÀÐ¢vÖTWfVçEG—S£¤ÖF6†Ö¶W%7FG5WFFVBÓâ&ÖF6†Ö¶W%÷7FG5÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¥&VÖF6…f÷FUW&–öD÷fW"Óâ'&VÖF6…÷f÷FU÷W&–öEö÷fW""ÀÐ¢vÖTWfVçEG—S£¥&VÖF6„f–ÆVEFô7&VFRÓâ'&VÖF6…öf–ÆVE÷Fõö7&VFR"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%&VÖF6„6†ævRÓâ'Æ–W%÷&VÖF6…ö6†ævR"ÀÐ¢vÖTWfVçEG—S£¥–æuWFFVBÓâ'–æu÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¤ÔÕ7FG5WFFVBÓâ&Ö×7FG5÷WFFVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$æW‡DÖf÷FT6†ævRÓâ'Æ–W%öæW‡EöÖ÷f÷FUö6†ævR"ÀÐ¢vÖTWfVçEG—S£¥f÷FTÖ46†ævVBÓâ'f÷FUöÖ5ö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥&÷FôFVd6†ævVBÓâ'&÷FõöFVeö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¥Æ–W$FöÖ–æF–öâÓâ'Æ–W%öFöÖ–æF–öâ"ÀÐ¢vÖTWfVçEG—S£¥Æ–W%&ö6¶WE6µW6†VBÓâ'Æ–W%÷&ö6¶WG6µ÷W6†VB"ÀÐ¢vÖTWfVçEG—S£¥VW7E&WVW7BÓâ'VW7E÷&WVW7B"ÀÐ¢vÖTWfVçEG—S£¥VW7E&W7öç6RÓâ'VW7E÷&W7öç6R"ÀÐ¢vÖTWfVçEG—S£¥VW7E&öw&W72Óâ'VW7E÷&öw&W72"ÀÐ¢vÖTWfVçEG—S£¥&ö¦V7F–ÆU&VÖ÷fVBÓâ'&ö¦V7F–ÆU÷&VÖ÷fVB"ÀÐ¢vÖTWfVçEG—S£¥VW7DÖFF6†ævVBÓâ'VW7EöÖöFFö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤v4F÷W6VEÆ–W$–væ—FVBÓâ&v5öF÷W6VE÷Æ–W%ö–væ—FVB"ÀÐ¢vÖTWfVçEG—S£¥VW7EGW&ä–å7FFRÓâ'VW7E÷GW&åö–å÷7FFR"ÀÐ¢vÖTWfVçEG—S£¤—FV×46¶æ÷vÆVFvVBÓâ&—FV×5ö6¶æ÷vÆVFvVB"ÀÐ¢vÖTWfVçEG—S£¤6W$¶–ÆÆVBÓâ&6W%ö¶–ÆÆVB"ÀÐ¢vÖTWfVçEG—S£¤Ö–äÖVçU7F&–Æ—¦VBÓâ&Ö–æÖVçU÷7F&–Æ—¦VB"ÀÐ¢vÖTWfVçEG—S£¥v÷&ÆE7FGW46†ævVBÓâ'v÷&ÆE÷7FGW5ö6†ævVB"ÀÐ¢vÖTWfVçEG—S£¤„ÅEe7FGW2Óâ&†ÇGe÷7FGW2"ÀÐ¢vÖTWfVçEG—S£¤„ÅEd6ÖW&ÖâÓâ&†ÇGeö6ÖW&Öâ"ÀÐ¢vÖTWfVçEG—S£¤„ÅEe&æ´6ÖW&Óâ&†ÇGe÷&æµö6ÖW&"ÀÐ¢vÖTWfVçEG—S£¤„ÅEe&æ´VçF—G’Óâ&†ÇGe÷&æµöVçF—G’"ÀÐ¢vÖTWfVçEG—S£¤„ÅEdf—†VBÓâ&†ÇGeöf—†VB"ÀÐ¢vÖTWfVçEG—S£¤„ÅEd6†6RÓâ&†ÇGeö6†6R"ÀÐ¢vÖTWfVçEG—S£¤„ÅEdÖW76vRÓâ&†ÇGeöÖW76vR"ÀÐ¢vÖTWfVçEG—S£¤„ÅEeF—FÆRÓâ&†ÇGe÷F—FÆR"ÀÐ¢vÖTWfVçEG—S£¤„ÅEd6†BÓâ&†ÇGeö6†B"ÀÐ¢vÖTWfVçEG—S£¥&WÆ•7F'E&V6÷&BÓâ'&WÆ•÷7F'G&V6÷&B"ÀÐ¢vÖTWfVçEG—S£¥&WÆ•6W76–öä–æfòÓâ'&WÆ•÷6W76–öæ–æfò"ÀÐ¢vÖTWfVçEG—S£¥&WÆ”VæE&V6÷&BÓâ'&WÆ•öVæG&V6÷&B"ÀÐ¢vÖTWfVçEG—S£¥&WÆ•&WÆ—4f–Æ&ÆRÓâ'&WÆ•÷&WÆ—6f–Æ&ÆR"ÀÐ¢vÖTWfVçEG—S£¥&WÆ•6W'fW$W'&÷"Óâ'&WÆ•÷6W'fW&W'&÷""ÀÐ¢vÖTWfVçEG—S£¥Væ¶æ÷vâ‡G’’ÓâG’ÀÐ¢ÐÐ¢ÐÐ§ÐÐ¦–×ÂvÖTWfVçB°Ð¢V"fâ&VB‡7G&VÓ¢f×WB7G&VÒÂFVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâ’Óâ&W7VÇCÅ6VÆcâ°Ð¢ö²†ÖF6‚FVf–æ—F–öâæWfVçE÷G—R°Ð¢vÖTWfVçEG—S£¥6W'fW%7vâÓâ°Ð¢vÖTWfVçC£¥6W'fW%7vâ„&÷ƒ£¦æWrƒÅ6W'fW%7väWfVçCã£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’Ð¢ÐÐ¢vÖTWfVçEG—S£¥6W'fW$6†ævTÆWfVÄf–ÆVBÓâvÖTWfVçC£¥6W'fW$6†ævTÆWfVÄf–ÆVB€Ð¢6W'fW$6†ævTÆWfVÄf–ÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥6W'fW%6‡WFF÷vâÓâ°Ð¢vÖTWfVçC£¥6W'fW%6‡WFF÷vâ…6W'fW%6‡WFF÷väWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6W'fW$7f"Óâ°Ð¢vÖTWfVçC£¥6W'fW$7f"…6W'fW$7f$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6W'fW$ÖW76vRÓâ°Ð¢vÖTWfVçC£¥6W'fW$ÖW76vR…6W'fW$ÖW76vTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6W'fW$FD&âÓâ°Ð¢vÖTWfVçC£¥6W'fW$FD&â„&÷ƒ£¦æWrƒÅ6W'fW$FD&äWfVçCã£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’Ð¢ÐÐ¢vÖTWfVçEG—S£¥6W'fW%&VÖ÷fT&âÓâ°Ð¢vÖTWfVçC£¥6W'fW%&VÖ÷fT&â…6W'fW%&VÖ÷fT&äWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$6öææV7BÓâ°Ð¢vÖTWfVçC£¥Æ–W$6öææV7B…Æ–W$6öææV7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$6öææV7D6Æ–VçBÓâ°Ð¢vÖTWfVçC£¥Æ–W$6öææV7D6Æ–VçB…Æ–W$6öææV7D6Æ–VçDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$–æfòÓâ°Ð¢vÖTWfVçC£¥Æ–W$–æfò…Æ–W$–æfôWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$F—66öææV7BÓâ°Ð¢vÖTWfVçC£¥Æ–W$F—66öææV7B…Æ–W$F—66öææV7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$7F—fFRÓâ°Ð¢vÖTWfVçC£¥Æ–W$7F—fFR…Æ–W$7F—fFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%6’Óâ°Ð¢vÖTWfVçC£¥Æ–W%6’…Æ–W%6”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6Æ–VçDF—66öææV7BÓâ°Ð¢vÖTWfVçC£¤6Æ–VçDF—66öææV7B„6Æ–VçDF—66öææV7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6Æ–VçD&Vv–ä6öææV7BÓâ°Ð¢vÖTWfVçC£¤6Æ–VçD&Vv–ä6öææV7B„6Æ–VçD&Vv–ä6öææV7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6Æ–VçD6öææV7FVBÓâ°Ð¢vÖTWfVçC£¤6Æ–VçD6öææV7FVB„6Æ–VçD6öææV7FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6Æ–VçDgVÆÄ6öææV7BÓâ°Ð¢vÖTWfVçC£¤6Æ–VçDgVÆÄ6öææV7B„6Æ–VçDgVÆÄ6öææV7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤†÷7EV—BÓâ°Ð¢vÖTWfVçC£¤†÷7EV—B„†÷7EV—DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÔ–æfòÓâ°Ð¢vÖTWfVçC£¥FVÔ–æfò…FVÔ–æfôWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕ66÷&RÓâ°Ð¢vÖTWfVçC£¥FVÕ66÷&R…FVÕ66÷&TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ”'&öF67DVF–òÓâvÖTWfVçC£¥FVÕÆ”'&öF67DVF–ò€Ð¢FVÕÆ”'&öF67DVF–ôWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W%FVÒÓâ°Ð¢vÖTWfVçC£¥Æ–W%FVÒ…Æ–W%FVÔWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$6Æ72Óâ°Ð¢vÖTWfVçC£¥Æ–W$6Æ72…Æ–W$6Æ74WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$FVF‚Óâ°Ð¢vÖTWfVçC£¥Æ–W$FVF‚„&÷ƒ£¦æWrƒÅÆ–W$FVF„WfVçCã£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’Ð¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$‡W'BÓâ°Ð¢vÖTWfVçC£¥Æ–W$‡W'B…Æ–W$‡W'DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$6†BÓâ°Ð¢vÖTWfVçC£¥Æ–W$6†B…Æ–W$6†DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%66÷&RÓâ°Ð¢vÖTWfVçC£¥Æ–W%66÷&R…Æ–W%66÷&TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%7vâÓâ°Ð¢vÖTWfVçC£¥Æ–W%7vâ…Æ–W%7väWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%6†ö÷BÓâ°Ð¢vÖTWfVçC£¥Æ–W%6†ö÷B…Æ–W%6†ö÷DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%W6RÓâ°Ð¢vÖTWfVçC£¥Æ–W%W6R…Æ–W%W6TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$6†ævTæÖRÓâ°Ð¢vÖTWfVçC£¥Æ–W$6†ævTæÖR…Æ–W$6†ævTæÖTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$†–çDÖW76vRÓâ°Ð¢vÖTWfVçC£¥Æ–W$†–çDÖW76vR…Æ–W$†–çDÖW76vTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤&6UÆ–W%FVÆW÷'FVBÓâvÖTWfVçC£¤&6UÆ–W%FVÆW÷'FVB€Ð¢&6UÆ–W%FVÆW÷'FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤vÖT–æ—BÓâ°Ð¢vÖTWfVçC£¤vÖT–æ—B„vÖT–æ—DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤vÖTæWtÖÓâ°Ð¢vÖTWfVçC£¤vÖTæWtÖ„vÖTæWtÖWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤vÖU7F'BÓâ°Ð¢vÖTWfVçC£¤vÖU7F'B„vÖU7F'DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤vÖTVæBÓâvÖTWfVçC£¤vÖTVæB„vÖTVæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’ÀÐ¢vÖTWfVçEG—S£¥&÷VæE7F'BÓâ°Ð¢vÖTWfVçC£¥&÷VæE7F'B…&÷VæE7F'DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&÷VæDVæBÓâ°Ð¢vÖTWfVçC£¥&÷VæDVæB…&÷VæDVæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤vÖTÖW76vRÓâ°Ð¢vÖTWfVçC£¤vÖTÖW76vR„vÖTÖW76vTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤'&V´'&V¶&ÆRÓâ°Ð¢vÖTWfVçC£¤'&V´'&V¶&ÆR„'&V´'&V¶&ÆTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤'&Vµ&÷Óâ°Ð¢vÖTWfVçC£¤'&Vµ&÷„'&Vµ&÷WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤VçF—G”¶–ÆÆVBÓâ°Ð¢vÖTWfVçC£¤VçF—G”¶–ÆÆVB„VçF—G”¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤&öçW5WFFVBÓâ°Ð¢vÖTWfVçC£¤&öçW5WFFVB„&öçW5WFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6†–WfVÖVçDWfVçBÓâ°Ð¢vÖTWfVçC£¤6†–WfVÖVçDWfVçB„6†–WfVÖVçDWfVçDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6†–WfVÖVçD–æ7&VÖVçBÓâvÖTWfVçC£¤6†–WfVÖVçD–æ7&VÖVçB€Ð¢6†–WfVÖVçD–æ7&VÖVçDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥‡—6wVå–6·WÓâ°Ð¢vÖTWfVçC£¥‡—6wVå–6·W…‡—6wVå–6·WWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤fÆ&T–væ—FTç2Óâ°Ð¢vÖTWfVçC£¤fÆ&T–væ—FTç2„fÆ&T–væ—FTç4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤†VÆ–6÷FW$w&VæFUVçDÖ—72ÓâvÖTWfVçC£¤†VÆ–6÷FW$w&VæFUVçDÖ—72€Ð¢†VÆ–6÷FW$w&VæFUVçDÖ—74WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥W6W$FFF÷væÆöFVBÓâ°Ð¢vÖTWfVçC£¥W6W$FFF÷væÆöFVB…W6W$FFF÷væÆöFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&vFöÆÄF—76öÇfVBÓâ°Ð¢vÖTWfVçC£¥&vFöÆÄF—76öÇfVB…&vFöÆÄF—76öÇfVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEd6†ævVDÖöFRÓâ°Ð¢vÖTWfVçC£¤„ÅEd6†ævVDÖöFR„„ÅEd6†ævVDÖöFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEd6†ævVEF&vWBÓâ°Ð¢vÖTWfVçC£¤„ÅEd6†ævVEF&vWB„„ÅEd6†ævVEF&vWDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥f÷FTVæFVBÓâ°Ð¢vÖTWfVçC£¥f÷FTVæFVB…f÷FTVæFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥f÷FU7F'FVBÓâ°Ð¢vÖTWfVçC£¥f÷FU7F'FVB…f÷FU7F'FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥f÷FT6†ævVBÓâ°Ð¢vÖTWfVçC£¥f÷FT6†ævVB…f÷FT6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥f÷FU76VBÓâ°Ð¢vÖTWfVçC£¥f÷FU76VB…f÷FU76VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥f÷FTf–ÆVBÓâ°Ð¢vÖTWfVçC£¥f÷FTf–ÆVB…f÷FTf–ÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥f÷FT67BÓâ°Ð¢vÖTWfVçC£¥f÷FT67B…f÷FT67DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥f÷FT÷F–öç2Óâ°Ð¢vÖTWfVçC£¥f÷FT÷F–öç2„&÷ƒ£¦æWrƒÅf÷FT÷F–öç4WfVçCã£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’Ð¢ÐÐ¢vÖTWfVçEG—S£¥&WÆ•6fVBÓâ°Ð¢vÖTWfVçC£¥&WÆ•6fVB…&WÆ•6fVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤VçFW&VEW&f÷&Öæ6TÖöFRÓâvÖTWfVçC£¤VçFW&VEW&f÷&Öæ6TÖöFR€Ð¢VçFW&VEW&f÷&Öæ6TÖöFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤'&÷w6U&WÆ—2Óâ°Ð¢vÖTWfVçC£¤'&÷w6U&WÆ—2„'&÷w6U&WÆ—4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&WÆ•–÷WGV&U7FG2Óâ°Ð¢vÖTWfVçC£¥&WÆ•–÷WGV&U7FG2…&WÆ•–÷WGV&U7FG4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤–çfVçF÷'•WFFVBÓâ°Ð¢vÖTWfVçC£¤–çfVçF÷'•WFFVB„–çfVçF÷'•WFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6'EWFFVBÓâ°Ð¢vÖTWfVçC£¤6'EWFFVB„6'EWFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥7F÷&U&–6U6†VWEWFFVBÓâvÖTWfVçC£¥7F÷&U&–6U6†VWEWFFVB€Ð¢7F÷&U&–6U6†VWEWFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤V6öä–çfVçF÷'”6öææV7FVBÓâvÖTWfVçC£¤V6öä–çfVçF÷'”6öææV7FVB€Ð¢V6öä–çfVçF÷'”6öææV7FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤—FVÕ66†VÖ–æ—F–Æ—¦VBÓâvÖTWfVçC£¤—FVÕ66†VÖ–æ—F–Æ—¦VB€Ð¢—FVÕ66†VÖ–æ—F–Æ—¦VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤v4æWu6W76–öâÓâ°Ð¢vÖTWfVçC£¤v4æWu6W76–öâ„v4æWu6W76–öäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤v4Æ÷7E6W76–öâÓâ°Ð¢vÖTWfVçC£¤v4Æ÷7E6W76–öâ„v4Æ÷7E6W76–öäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤–çG&ôf–æ—6‚Óâ°Ð¢vÖTWfVçC£¤–çG&ôf–æ—6‚„–çG&ôf–æ—6„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤–çG&ôæW‡D6ÖW&Óâ°Ð¢vÖTWfVçC£¤–çG&ôæW‡D6ÖW&„–çG&ôæW‡D6ÖW&WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$6†ævT6Æ72Óâ°Ð¢vÖTWfVçC£¥Æ–W$6†ævT6Æ72…Æ–W$6†ævT6Æ74WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FdÖF–ÖU&VÖ–æ–ærÓâ°Ð¢vÖTWfVçC£¥FdÖF–ÖU&VÖ–æ–ær…FdÖF–ÖU&VÖ–æ–ætWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FdvÖT÷fW"Óâ°Ð¢vÖTWfVçC£¥FdvÖT÷fW"…FdvÖT÷fW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤7FdfÆt6GW&VBÓâ°Ð¢vÖTWfVçC£¤7FdfÆt6GW&VB„7FdfÆt6GW&VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çD–æ—F–Æ—¦VBÓâvÖTWfVçC£¤6öçG&öÅö–çD–æ—F–Æ—¦VB€Ð¢6öçG&öÅö–çD–æ—F–Æ—¦VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT–ÖvW2ÓâvÖTWfVçC£¤6öçG&öÅö–çEWFFT–ÖvW2€Ð¢6öçG&öÅö–çEWFFT–ÖvW4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEWFFTÆ–÷WBÓâvÖTWfVçC£¤6öçG&öÅö–çEWFFTÆ–÷WB€Ð¢6öçG&öÅö–çEWFFTÆ–÷WDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT6–ærÓâvÖTWfVçC£¤6öçG&öÅö–çEWFFT6–ær€Ð¢6öçG&öÅö–çEWFFT6–ætWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT÷væW"ÓâvÖTWfVçC£¤6öçG&öÅö–çEWFFT÷væW"€Ð¢6öçG&öÅö–çEWFFT÷væW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çE7F'EF÷V6‚ÓâvÖTWfVçC£¤6öçG&öÅö–çE7F'EF÷V6‚€Ð¢6öçG&öÅö–çE7F'EF÷V6„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çDVæEF÷V6‚ÓâvÖTWfVçC£¤6öçG&öÅö–çDVæEF÷V6‚€Ð¢6öçG&öÅö–çDVæEF÷V6„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEVÇ6TVÆVÖVçBÓâvÖTWfVçC£¤6öçG&öÅö–çEVÇ6TVÆVÖVçB€Ð¢6öçG&öÅö–çEVÇ6TVÆVÖVçDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çDf¶T6GW&RÓâvÖTWfVçC£¤6öçG&öÅö–çDf¶T6GW&R€Ð¢6öçG&öÅö–çDf¶T6GW&TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W"Óâ°Ð¢vÖTWfVçC£¤6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W"€Ð¢6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢Ð¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæE6VÆV7FVBÓâvÖTWfVçC£¥FVÕÆ•&÷VæE6VÆV7FVB€Ð¢FVÕÆ•&÷VæE6VÆV7FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæE7F'BÓâ°Ð¢vÖTWfVçC£¥FVÕÆ•&÷VæE7F'B…FVÕÆ•&÷VæE7F'DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæD7F—fRÓâ°Ð¢vÖTWfVçC£¥FVÕÆ•&÷VæD7F—fR…FVÕÆ•&÷VæD7F—fTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•v—F–æt&Vv–ç2ÓâvÖTWfVçC£¥FVÕÆ•v—F–æt&Vv–ç2€Ð¢FVÕÆ•v—F–æt&Vv–ç4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•v—F–ætVæG2Óâ°Ð¢vÖTWfVçC£¥FVÕÆ•v—F–ætVæG2…FVÕÆ•v—F–ætVæG4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•v—F–æt&÷WEFôVæBÓâvÖTWfVçC£¥FVÕÆ•v—F–æt&÷WEFôVæB€Ð¢FVÕÆ•v—F–æt&÷WEFôVæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&W7F'E&÷VæBÓâvÖTWfVçC£¥FVÕÆ•&W7F'E&÷VæB€Ð¢FVÕÆ•&W7F'E&÷VæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&VG•&W7F'BÓâvÖTWfVçC£¥FVÕÆ•&VG•&W7F'B€Ð¢FVÕÆ•&VG•&W7F'DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæE&W7F'E6V6öæG2ÓâvÖTWfVçC£¥FVÕÆ•&÷VæE&W7F'E6V6öæG2€Ð¢FVÕÆ•&÷VæE&W7F'E6V6öæG4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•FVÕ&VG’Óâ°Ð¢vÖTWfVçC£¥FVÕÆ•FVÕ&VG’…FVÕÆ•FVÕ&VG”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæEv–âÓâ°Ð¢vÖTWfVçC£¥FVÕÆ•&÷VæEv–â…FVÕÆ•&÷VæEv–äWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•WFFUF–ÖW"Óâ°Ð¢vÖTWfVçC£¥FVÕÆ•WFFUF–ÖW"…FVÕÆ•WFFUF–ÖW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•&÷VæE7FÆVÖFRÓâvÖTWfVçC£¥FVÕÆ•&÷VæE7FÆVÖFR€Ð¢FVÕÆ•&÷VæE7FÆVÖFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”÷fW'F–ÖT&Vv–âÓâvÖTWfVçC£¥FVÕÆ”÷fW'F–ÖT&Vv–â€Ð¢FVÕÆ”÷fW'F–ÖT&Vv–äWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”÷fW'F–ÖTVæBÓâ°Ð¢vÖTWfVçC£¥FVÕÆ”÷fW'F–ÖTVæB…FVÕÆ”÷fW'F–ÖTVæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•7VFFVäFVF„&Vv–âÓâvÖTWfVçC£¥FVÕÆ•7VFFVäFVF„&Vv–â€Ð¢FVÕÆ•7VFFVäFVF„&Vv–äWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•7VFFVäFVF„VæBÓâvÖTWfVçC£¥FVÕÆ•7VFFVäFVF„VæB€Ð¢FVÕÆ•7VFFVäFVF„VæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”vÖT÷fW"Óâ°Ð¢vÖTWfVçC£¥FVÕÆ”vÖT÷fW"…FVÕÆ”vÖT÷fW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ”ÖF–ÖU&VÖ–æ–ærÓâvÖTWfVçC£¥FVÕÆ”ÖF–ÖU&VÖ–æ–ær€Ð¢FVÕÆ”ÖF–ÖU&VÖ–æ–ætWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•F–ÖW$fÆ6‚Óâ°Ð¢vÖTWfVçC£¥FVÕÆ•F–ÖW$fÆ6‚…FVÕÆ•F–ÖW$fÆ6„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•F–ÖW%F–ÖTFFVBÓâvÖTWfVçC£¥FVÕÆ•F–ÖW%F–ÖTFFVB€Ð¢FVÕÆ•F–ÖW%F–ÖTFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•ö–çE7F'D6GW&RÓâvÖTWfVçC£¥FVÕÆ•ö–çE7F'D6GW&R€Ð¢FVÕÆ•ö–çE7F'D6GW&TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•ö–çD6GW&VBÓâvÖTWfVçC£¥FVÕÆ•ö–çD6GW&VB€Ð¢FVÕÆ•ö–çD6GW&VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•ö–çDÆö6¶VBÓâ°Ð¢vÖTWfVçC£¥FVÕÆ•ö–çDÆö6¶VB…FVÕÆ•ö–çDÆö6¶VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•ö–çEVæÆö6¶VBÓâvÖTWfVçC£¥FVÕÆ•ö–çEVæÆö6¶VB€Ð¢FVÕÆ•ö–çEVæÆö6¶VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”6GW&T'&ö¶VâÓâvÖTWfVçC£¥FVÕÆ”6GW&T'&ö¶Vâ€Ð¢FVÕÆ”6GW&T'&ö¶VäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”6GW&T&Æö6¶VBÓâvÖTWfVçC£¥FVÕÆ”6GW&T&Æö6¶VB€Ð¢FVÕÆ”6GW&T&Æö6¶VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”fÆtWfVçBÓâ°Ð¢vÖTWfVçC£¥FVÕÆ”fÆtWfVçB…FVÕÆ”fÆtWfVçDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•v–åæVÂÓâ°Ð¢vÖTWfVçC£¥FVÕÆ•v–åæVÂ…FVÕÆ•v–åæVÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•FVÔ&Ææ6VEÆ–W"ÓâvÖTWfVçC£¥FVÕÆ•FVÔ&Ææ6VEÆ–W"€Ð¢FVÕÆ•FVÔ&Ææ6VEÆ–W$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ•6WGWf–æ—6†VBÓâvÖTWfVçC£¥FVÕÆ•6WGWf–æ—6†VB€Ð¢FVÕÆ•6WGWf–æ—6†VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥FVÕÆ”ÆW'BÓâ°Ð¢vÖTWfVçC£¥FVÕÆ”ÆW'B…FVÕÆ”ÆW'DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥G&–æ–æt6ö×ÆWFRÓâ°Ð¢vÖTWfVçC£¥G&–æ–æt6ö×ÆWFR…G&–æ–æt6ö×ÆWFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6†÷tg&VW¦UæVÂÓâ°Ð¢vÖTWfVçC£¥6†÷tg&VW¦UæVÂ…6†÷tg&VW¦UæVÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤†–FTg&VW¦UæVÂÓâ°Ð¢vÖTWfVçC£¤†–FTg&VW¦UæVÂ„†–FTg&VW¦UæVÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤g&VW¦T6Õ7F'FVBÓâ°Ð¢vÖTWfVçC£¤g&VW¦T6Õ7F'FVB„g&VW¦T6Õ7F'FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævUFVÒÓâvÖTWfVçC£¤Æö6ÅÆ–W$6†ævUFVÒ€Ð¢Æö6ÅÆ–W$6†ævUFVÔWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W%66÷&T6†ævVBÓâvÖTWfVçC£¤Æö6ÅÆ–W%66÷&T6†ævVB€Ð¢Æö6ÅÆ–W%66÷&T6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævT6Æ72ÓâvÖTWfVçC£¤Æö6ÅÆ–W$6†ævT6Æ72€Ð¢Æö6ÅÆ–W$6†ævT6Æ74WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W%&W7vâÓâ°Ð¢vÖTWfVçC£¤Æö6ÅÆ–W%&W7vâ„Æö6ÅÆ–W%&W7väWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤'V–ÆF–æt–æfô6†ævVBÓâ°Ð¢vÖTWfVçC£¤'V–ÆF–æt–æfô6†ævVB„'V–ÆF–æt–æfô6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævTF—6wV—6RÓâvÖTWfVçC£¤Æö6ÅÆ–W$6†ævTF—6wV—6R€Ð¢Æö6ÅÆ–W$6†ævTF—6wV—6TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$66÷VçD6†ævVBÓâvÖTWfVçC£¥Æ–W$66÷VçD6†ævVB€Ð¢Æ–W$66÷VçD6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥7•F&W6WBÓâ°Ð¢vÖTWfVçC£¥7•F&W6WB…7•F&W6WDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤fÆu7FGW5WFFRÓâ°Ð¢vÖTWfVçC£¤fÆu7FGW5WFFR„fÆu7FGW5WFFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%7FG5WFFVBÓâ°Ð¢vÖTWfVçC£¥Æ–W%7FG5WFFVB…Æ–W%7FG5WFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ––æt6öÖÖVçF'’Óâ°Ð¢vÖTWfVçC£¥Æ––æt6öÖÖVçF'’…Æ––æt6öÖÖVçF'”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$6†&vTFWÆ÷–VBÓâvÖTWfVçC£¥Æ–W$6†&vTFWÆ÷–VB€Ð¢Æ–W$6†&vTFWÆ÷–VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$'V–ÇDö&¦V7BÓâ°Ð¢vÖTWfVçC£¥Æ–W$'V–ÇDö&¦V7B…Æ–W$'V–ÇDö&¦V7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%Ww&FVDö&¦V7BÓâvÖTWfVçC£¥Æ–W%Ww&FVDö&¦V7B€Ð¢Æ–W%Ww&FVDö&¦V7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6''”ö&¦V7BÓâ°Ð¢vÖTWfVçC£¥Æ–W$6''”ö&¦V7B…Æ–W$6''”ö&¦V7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$G&÷ö&¦V7BÓâ°Ð¢vÖTWfVçC£¥Æ–W$G&÷ö&¦V7B…Æ–W$G&÷ö&¦V7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ö&¦V7E&VÖ÷fVBÓâ°Ð¢vÖTWfVçC£¤ö&¦V7E&VÖ÷fVB„ö&¦V7E&VÖ÷fVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ö&¦V7DFW7G&÷–VBÓâ°Ð¢vÖTWfVçC£¤ö&¦V7DFW7G&÷–VB„ö&¦V7DFW7G&÷–VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ö&¦V7DFWFöæFVBÓâ°Ð¢vÖTWfVçC£¤ö&¦V7DFWFöæFVB„ö&¦V7DFWFöæFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6†–WfVÖVçDV&æVBÓâ°Ð¢vÖTWfVçC£¤6†–WfVÖVçDV&æVB„6†–WfVÖVçDV&æVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥7V5F&vWEWFFVBÓâ°Ð¢vÖTWfVçC£¥7V5F&vWEWFFVB…7V5F&vWEWFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥F÷W&æÖVçE7FFUWFFRÓâvÖTWfVçC£¥F÷W&æÖVçE7FFUWFFR€Ð¢F÷W&æÖVçE7FFUWFFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâÓâvÖTWfVçC£¥F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâ€Ð¢F÷W&æÖVçDVæ&ÆT6÷VçFF÷väWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6ÆÆVDf÷$ÖVF–2ÓâvÖTWfVçC£¥Æ–W$6ÆÆVDf÷$ÖVF–2€Ð¢Æ–W$6ÆÆVDf÷$ÖVF–4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$6¶VDf÷$&ÆÂÓâ°Ð¢vÖTWfVçC£¥Æ–W$6¶VDf÷$&ÆÂ…Æ–W$6¶VDf÷$&ÆÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$&V6ÖTö'6W'fW"ÓâvÖTWfVçC£¤Æö6ÅÆ–W$&V6ÖTö'6W'fW"€Ð¢Æö6ÅÆ–W$&V6ÖTö'6W'fW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$–væ—FVD–çbÓâ°Ð¢vÖTWfVçC£¥Æ–W$–væ—FVD–çb…Æ–W$–væ—FVD–çdWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$–væ—FVBÓâ°Ð¢vÖTWfVçC£¥Æ–W$–væ—FVB…Æ–W$–væ—FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$W‡F–æwV—6†VBÓâ°Ð¢vÖTWfVçC£¥Æ–W$W‡F–æwV—6†VB…Æ–W$W‡F–æwV—6†VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%FVÆW÷'FVBÓâ°Ð¢vÖTWfVçC£¥Æ–W%FVÆW÷'FVB…Æ–W%FVÆW÷'FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$†VÆVDÖVF–46ÆÂÓâvÖTWfVçC£¥Æ–W$†VÆVDÖVF–46ÆÂ€Ð¢Æ–W$†VÆVDÖVF–46ÆÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$6†&vU&VG’ÓâvÖTWfVçC£¤Æö6ÅÆ–W$6†&vU&VG’€Ð¢Æö6ÅÆ–W$6†&vU&VG”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W%v–æDF÷vâÓâ°Ð¢vÖTWfVçC£¤Æö6ÅÆ–W%v–æDF÷vâ„Æö6ÅÆ–W%v–æDF÷väWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$–çgVÆæVBÓâ°Ð¢vÖTWfVçC£¥Æ–W$–çgVÆæVB…Æ–W$–çgVÆæVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W66÷'E7VVBÓâ°Ð¢vÖTWfVçC£¤W66÷'E7VVB„W66÷'E7VVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W66÷'E&öw&W72Óâ°Ð¢vÖTWfVçC£¤W66÷'E&öw&W72„W66÷'E&öw&W74WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W66÷'E&V6VFRÓâ°Ð¢vÖTWfVçC£¤W66÷'E&V6VFR„W66÷'E&V6VFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤vÖUT”7F—fFVBÓâ°Ð¢vÖTWfVçC£¤vÖUT”7F—fFVB„vÖUT”7F—fFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤vÖUT”†–FFVâÓâ°Ð¢vÖTWfVçC£¤vÖUT”†–FFVâ„vÖUT”†–FFVäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$W66÷'E66÷&RÓâ°Ð¢vÖTWfVçC£¥Æ–W$W66÷'E66÷&R…Æ–W$W66÷'E66÷&TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$†VÄöä†—BÓâ°Ð¢vÖTWfVçC£¥Æ–W$†VÄöä†—B…Æ–W$†VÄöä†—DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%7FVÅ6æGf–6‚Óâ°Ð¢vÖTWfVçC£¥Æ–W%7FVÅ6æGf–6‚…Æ–W%7FVÅ6æGf–6„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6†÷t6Æ74Æ–÷WBÓâ°Ð¢vÖTWfVçC£¥6†÷t6Æ74Æ–÷WB…6†÷t6Æ74Æ–÷WDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6†÷ug5æVÂÓâ°Ð¢vÖTWfVçC£¥6†÷ug5æVÂ…6†÷ug5æVÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$FÖvVBÓâ°Ð¢vÖTWfVçC£¥Æ–W$FÖvVB…Æ–W$FÖvVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤&VæÆ–W$æ÷F–f–6F–öâÓâvÖTWfVçC£¤&VæÆ–W$æ÷F–f–6F–öâ€Ð¢&VæÆ–W$æ÷F–f–6F–öäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤&VæÖF6„Ö…7G&V²Óâ°Ð¢vÖTWfVçC£¤&VæÖF6„Ö…7G&V²„&VæÖF6„Ö…7G&V´WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤&Væ&÷VæE7F'BÓâ°Ð¢vÖTWfVçC£¤&Væ&÷VæE7F'B„&Væ&÷VæE7F'DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤&Væv–åæVÂÓâ°Ð¢vÖTWfVçC£¤&Væv–åæVÂ„&Væv–åæVÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥fUv–åæVÂÓâ°Ð¢vÖTWfVçC£¥fUv–åæVÂ…fUv–åæVÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤—$F6‚ÓâvÖTWfVçC£¤—$F6‚„—$F6„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’ÀÐ¢vÖTWfVçEG—S£¤ÆæFVBÓâvÖTWfVçC£¤ÆæFVB„ÆæFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$FÖvTFöFvVBÓâ°Ð¢vÖTWfVçC£¥Æ–W$FÖvTFöFvVB…Æ–W$FÖvTFöFvVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%7GVææVBÓâ°Ð¢vÖTWfVçC£¥Æ–W%7GVææVB…Æ–W%7GVææVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥66÷WDw&æE6ÆÒÓâ°Ð¢vÖTWfVçC£¥66÷WDw&æE6ÆÒ…66÷WDw&æE6ÆÔWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥66÷WE6ÆÖFöÆÄÆæFVBÓâ°Ð¢vÖTWfVçC£¥66÷WE6ÆÖFöÆÄÆæFVB…66÷WE6ÆÖFöÆÄÆæFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤'&÷t–×7BÓâ°Ð¢vÖTWfVçC£¤'&÷t–×7B„'&÷t–×7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$¦&FVBÓâ°Ð¢vÖTWfVçC£¥Æ–W$¦&FVB…Æ–W$¦&FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$¦&FVDfFRÓâ°Ð¢vÖTWfVçC£¥Æ–W$¦&FVDfFR…Æ–W$¦&FVDfFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%6†–VÆD&Æö6¶VBÓâ°Ð¢vÖTWfVçC£¥Æ–W%6†–VÆD&Æö6¶VB…Æ–W%6†–VÆD&Æö6¶VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%–ææVBÓâ°Ð¢vÖTWfVçC£¥Æ–W%–ææVB…Æ–W%–ææVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$†VÆVD'”ÖVF–2Óâ°Ð¢vÖTWfVçC£¥Æ–W$†VÆVD'”ÖVF–2…Æ–W$†VÆVD'”ÖVF–4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%6VDö&¦V7BÓâ°Ð¢vÖTWfVçC£¥Æ–W%6VDö&¦V7B…Æ–W%6VDö&¦V7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤—FVÔf÷VæBÓâ°Ð¢vÖTWfVçC£¤—FVÔf÷VæB„—FVÔf÷VæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6†÷tææ÷FF–öâÓâ°Ð¢vÖTWfVçC£¥6†÷tææ÷FF–öâ…6†÷tææ÷FF–öäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤†–FTææ÷FF–öâÓâ°Ð¢vÖTWfVçC£¤†–FTææ÷FF–öâ„†–FTææ÷FF–öäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥÷7D–çfVçF÷'”Æ–6F–öâÓâvÖTWfVçC£¥÷7D–çfVçF÷'”Æ–6F–öâ€Ð¢÷7D–çfVçF÷'”Æ–6F–öäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEVæÆö6µWFFVBÓâvÖTWfVçC£¤6öçG&öÅö–çEVæÆö6µWFFVB€Ð¢6öçG&öÅö–çEVæÆö6µWFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤FWÆ÷”'Vfd&ææW"Óâ°Ð¢vÖTWfVçC£¤FWÆ÷”'Vfd&ææW"„FWÆ÷”'Vfd&ææW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$'VfbÓâ°Ð¢vÖTWfVçC£¥Æ–W$'Vfb…Æ–W$'VfdWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖVF–4FVF‚Óâ°Ð¢vÖTWfVçC£¤ÖVF–4FVF‚„ÖVF–4FVF„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤÷fW'F–ÖTærÓâ°Ð¢vÖTWfVçC£¤÷fW'F–ÖTær„÷fW'F–ÖTætWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FV×46†ævVBÓâ°Ð¢vÖTWfVçC£¥FV×46†ævVB…FV×46†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVåV×¶–äw&"ÓâvÖTWfVçC£¤†ÆÆ÷vVVåV×¶–äw&"€Ð¢†ÆÆ÷vVVåV×¶–äw&$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥&ö6¶WD§V×Óâ°Ð¢vÖTWfVçC£¥&ö6¶WD§V×…&ö6¶WD§V×WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&ö6¶WD§V×ÆæFVBÓâ°Ð¢vÖTWfVçC£¥&ö6¶WD§V×ÆæFVB…&ö6¶WD§V×ÆæFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥7F–6·”§V×Óâ°Ð¢vÖTWfVçC£¥7F–6·”§V×…7F–6·”§V×WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥7F–6·”§V×ÆæFVBÓâ°Ð¢vÖTWfVçC£¥7F–6·”§V×ÆæFVB…7F–6·”§V×ÆæFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&ö6¶WE6´ÆVæ6‚Óâ°Ð¢vÖTWfVçC£¥&ö6¶WE6´ÆVæ6‚…&ö6¶WE6´ÆVæ6„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&ö6¶WE6´ÆæFVBÓâ°Ð¢vÖTWfVçC£¥&ö6¶WE6´ÆæFVB…&ö6¶WE6´ÆæFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖVF–4FVfVæFVBÓâ°Ð¢vÖTWfVçC£¤ÖVF–4FVfVæFVB„ÖVF–4FVfVæFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W$†VÆVBÓâ°Ð¢vÖTWfVçC£¤Æö6ÅÆ–W$†VÆVB„Æö6ÅÆ–W$†VÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$FW7G&÷–VE—T&öÖ"ÓâvÖTWfVçC£¥Æ–W$FW7G&÷–VE—T&öÖ"€Ð¢Æ–W$FW7G&÷–VE—T&öÖ$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤ö&¦V7DFVfÆV7FVBÓâ°Ð¢vÖTWfVçC£¤ö&¦V7DFVfÆV7FVB„ö&¦V7DFVfÆV7FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$×gÓâ°Ð¢vÖTWfVçC£¥Æ–W$×g…Æ–W$×gWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&–E7väÖö"Óâ°Ð¢vÖTWfVçC£¥&–E7väÖö"…&–E7väÖö$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&–E7vå7VBÓâ°Ð¢vÖTWfVçC£¥&–E7vå7VB…&–E7vå7VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤æd&Æö6¶VBÓâ°Ð¢vÖTWfVçC£¤æd&Æö6¶VB„æd&Æö6¶VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥F…G&6µ76VBÓâ°Ð¢vÖTWfVçC£¥F…G&6µ76VB…F…G&6µ76VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤çVÔ6W'46†ævVBÓâ°Ð¢vÖTWfVçC£¤çVÔ6W'46†ævVB„çVÔ6W'46†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%&VvVæW&FRÓâ°Ð¢vÖTWfVçC£¥Æ–W%&VvVæW&FR…Æ–W%&VvVæW&FTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥WFFU7FGW4—FVÒÓâ°Ð¢vÖTWfVçC£¥WFFU7FGW4—FVÒ…WFFU7FGW4—FVÔWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥7FG5&W6WE&÷VæBÓâ°Ð¢vÖTWfVçC£¥7FG5&W6WE&÷VæB…7FG5&W6WE&÷VæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥66÷&U7FG467V×VÆFVEWFFRÓâvÖTWfVçC£¥66÷&U7FG467V×VÆFVEWFFR€Ð¢66÷&U7FG467V×VÆFVEWFFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥66÷&U7FG467V×VÆFVE&W6WBÓâvÖTWfVçC£¥66÷&U7FG467V×VÆFVE&W6WB€Ð¢66÷&U7FG467V×VÆFVE&W6WDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6†–WfVÖVçDV&æVDÆö6ÂÓâvÖTWfVçC£¤6†–WfVÖVçDV&æVDÆö6Â€Ð¢6†–WfVÖVçDV&æVDÆö6ÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†VÆVBÓâ°Ð¢vÖTWfVçC£¥Æ–W$†VÆVB…Æ–W$†VÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤'V–ÆF–æt†VÆVBÓâ°Ð¢vÖTWfVçC£¤'V–ÆF–æt†VÆVB„'V–ÆF–æt†VÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤—FVÕ–6·WÓâ°Ð¢vÖTWfVçC£¤—FVÕ–6·W„—FVÕ–6·WWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤GVVÅ7FGW2Óâ°Ð¢vÖTWfVçC£¤GVVÅ7FGW2„GVVÅ7FGW4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤f—6„æ÷F–6RÓâ°Ð¢vÖTWfVçC£¤f—6„æ÷F–6R„&÷ƒ£¦æWrƒÄf—6„æ÷F–6TWfVçCã£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’Ð¢ÐÐ¢vÖTWfVçEG—S£¤f—6„æ÷F–6T&ÒÓâ°Ð¢vÖTWfVçC£¤f—6„æ÷F–6T&Ò„&÷ƒ£¦æWrƒÄf—6„æ÷F–6T&ÔWfVçCã£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’Ð¢ÐÐ¢vÖTWfVçEG—S£¥6Ææ÷F–6RÓâ°Ð¢vÖTWfVçC£¥6Ææ÷F–6R„&÷ƒ£¦æWrƒÅ6Ææ÷F–6TWfVçCã£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’Ð¢ÐÐ¢vÖTWfVçEG—S£¥F‡&÷v&ÆT†—BÓâ°Ð¢vÖTWfVçC£¥F‡&÷v&ÆT†—B„&÷ƒ£¦æWrƒÅF‡&÷v&ÆT†—DWfVçCã£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’Ð¢ÐÐ¢vÖTWfVçEG—S£¥V×¶–äÆ÷&E7VÖÖöæVBÓâ°Ð¢vÖTWfVçC£¥V×¶–äÆ÷&E7VÖÖöæVB…V×¶–äÆ÷&E7VÖÖöæVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥V×¶–äÆ÷&D¶–ÆÆVBÓâ°Ð¢vÖTWfVçC£¥V×¶–äÆ÷&D¶–ÆÆVB…V×¶–äÆ÷&D¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖW&6×W57VÖÖöæVBÓâ°Ð¢vÖTWfVçC£¤ÖW&6×W57VÖÖöæVB„ÖW&6×W57VÖÖöæVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖW&6×W4¶–ÆÆVBÓâ°Ð¢vÖTWfVçC£¤ÖW&6×W4¶–ÆÆVB„ÖW&6×W4¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖW&6×W4W66Uv&æ–ærÓâvÖTWfVçC£¤ÖW&6×W4W66Uv&æ–ær€Ð¢ÖW&6×W4W66Uv&æ–ætWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤ÖW&6×W4W66VBÓâ°Ð¢vÖTWfVçC£¤ÖW&6×W4W66VB„ÖW&6×W4W66VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷757VÖÖöæVBÓâ°Ð¢vÖTWfVçC£¤W–V&ÆÄ&÷757VÖÖöæVB„W–V&ÆÄ&÷757VÖÖöæVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷757GVææVBÓâ°Ð¢vÖTWfVçC£¤W–V&ÆÄ&÷757GVææVB„W–V&ÆÄ&÷757GVææVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷74¶–ÆÆVBÓâ°Ð¢vÖTWfVçC£¤W–V&ÆÄ&÷74¶–ÆÆVB„W–V&ÆÄ&÷74¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷74¶–ÆÆW"Óâ°Ð¢vÖTWfVçC£¤W–V&ÆÄ&÷74¶–ÆÆW"„W–V&ÆÄ&÷74¶–ÆÆW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷74W66T–ÖÖ–æVçBÓâvÖTWfVçC£¤W–V&ÆÄ&÷74W66T–ÖÖ–æVçB€Ð¢W–V&ÆÄ&÷74W66T–ÖÖ–æVçDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤W–V&ÆÄ&÷74W66VBÓâ°Ð¢vÖTWfVçC£¤W–V&ÆÄ&÷74W66VB„W–V&ÆÄ&÷74W66VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ç4‡W'BÓâvÖTWfVçC£¤ç4‡W'B„ç4‡W'DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’ÀÐ¢vÖTWfVçEG—S£¤6öçG&öÅö–çEF–ÖW%WFFVBÓâvÖTWfVçC£¤6öçG&öÅö–çEF–ÖW%WFFVB€Ð¢6öçG&öÅö–çEF–ÖW%WFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†–v„f—fU7F'BÓâ°Ð¢vÖTWfVçC£¥Æ–W$†–v„f—fU7F'B…Æ–W$†–v„f—fU7F'DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$†–v„f—fT6æ6VÂÓâvÖTWfVçC£¥Æ–W$†–v„f—fT6æ6VÂ€Ð¢Æ–W$†–v„f—fT6æ6VÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$†–v„f—fU7V66W72ÓâvÖTWfVçC£¥Æ–W$†–v„f—fU7V66W72€Ð¢Æ–W$†–v„f—fU7V66W74WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$&öçW5ö–çG2Óâ°Ð¢vÖTWfVçC£¥Æ–W$&öçW5ö–çG2…Æ–W$&öçW5ö–çG4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%Ww&FVBÓâ°Ð¢vÖTWfVçC£¥Æ–W%Ww&FVB…Æ–W%Ww&FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$'W–&6²Óâ°Ð¢vÖTWfVçC£¥Æ–W$'W–&6²…Æ–W$'W–&6´WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%W6VE÷vW%W&÷GFÆRÓâvÖTWfVçC£¥Æ–W%W6VE÷vW%W&÷GFÆR€Ð¢Æ–W%W6VE÷vW%W&÷GFÆTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6‡&—7FÖ4v–gDw&"Óâ°Ð¢vÖTWfVçC£¤6‡&—7FÖ4v–gDw&"„6‡&—7FÖ4v–gDw&$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæRÓâvÖTWfVçC£¥Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæR€Ð¢Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥'G•WFFVBÓâ°Ð¢vÖTWfVçC£¥'G•WFFVB…'G•WFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥'G•&Vd6†ævVBÓâ°Ð¢vÖTWfVçC£¥'G•&Vd6†ævVB…'G•&Vd6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥'G”7&—FW&–6†ævVBÓâvÖTWfVçC£¥'G”7&—FW&–6†ævVB€Ð¢'G”7&—FW&–6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥'G”–çf—FW46†ævVBÓâ°Ð¢vÖTWfVçC£¥'G”–çf—FW46†ævVB…'G”–çf—FW46†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥'G•VWVU7FFT6†ævVBÓâvÖTWfVçC£¥'G•VWVU7FFT6†ævVB€Ð¢'G•VWVU7FFT6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥'G”6†BÓâ°Ð¢vÖTWfVçC£¥'G”6†B…'G”6†DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥'G”ÖVÖ&W$¦ö–âÓâ°Ð¢vÖTWfVçC£¥'G”ÖVÖ&W$¦ö–â…'G”ÖVÖ&W$¦ö–äWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥'G”ÖVÖ&W$ÆVfRÓâ°Ð¢vÖTWfVçC£¥'G”ÖVÖ&W$ÆVfR…'G”ÖVÖ&W$ÆVfTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖF6„–çf—FW5WFFVBÓâ°Ð¢vÖTWfVçC£¤ÖF6„–çf—FW5WFFVB„ÖF6„–çf—FW5WFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤Æö&'•WFFVBÓâ°Ð¢vÖTWfVçC£¤Æö&'•WFFVB„Æö&'•WFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤×fÔÖ—76–öåWFFRÓâ°Ð¢vÖTWfVçC£¤×fÔÖ—76–öåWFFR„×fÔÖ—76–öåWFFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&V6Æ7VÆFT†öÆ–F—2Óâ°Ð¢vÖTWfVçC£¥&V6Æ7VÆFT†öÆ–F—2…&V6Æ7VÆFT†öÆ–F—4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$7W'&Væ7”6†ævVBÓâvÖTWfVçC£¥Æ–W$7W'&Væ7”6†ævVB€Ð¢Æ–W$7W'&Væ7”6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤Föö×6F•&ö6¶WD÷VâÓâ°Ð¢vÖTWfVçC£¤Föö×6F•&ö6¶WD÷Vâ„Föö×6F•&ö6¶WD÷VäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&VÖ÷fTæVÖW6—5&VÆF–öç6†—2ÓâvÖTWfVçC£¥&VÖ÷fTæVÖW6—5&VÆF–öç6†—2€Ð¢&VÖ÷fTæVÖW6—5&VÆF–öç6†—4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÔ7&VF—D&öçW5vfRÓâ°Ð¢vÖTWfVçC£¤×fÔ7&VF—D&öçW5vfR„×fÔ7&VF—D&öçW5vfTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤×fÔ7&VF—D&öçW4ÆÂÓâ°Ð¢vÖTWfVçC£¤×fÔ7&VF—D&öçW4ÆÂ„×fÔ7&VF—D&öçW4ÆÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤×fÔ7&VF—D&öçW4ÆÄGfæ6VBÓâvÖTWfVçC£¤×fÔ7&VF—D&öçW4ÆÄGfæ6VB€Ð¢×fÔ7&VF—D&öçW4ÆÄGfæ6VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÕV–6µ6VçG'•Ww&FRÓâvÖTWfVçC£¤×fÕV–6µ6VçG'•Ww&FR€Ð¢×fÕV–6µ6VçG'•Ww&FTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÕFæ´FW7G&÷–VD'•Æ–W'2ÓâvÖTWfVçC£¤×fÕFæ´FW7G&÷–VD'•Æ–W'2€Ð¢×fÕFæ´FW7G&÷–VD'•Æ–W'4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ"ÓâvÖTWfVçC£¤×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ"€Ð¢×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÕ–6·W7W'&Væ7’Óâ°Ð¢vÖTWfVçC£¤×fÕ–6·W7W'&Væ7’„×fÕ–6·W7W'&Væ7”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤×fÔ&öÖ$6'&–W$¶–ÆÆVBÓâvÖTWfVçC£¤×fÔ&öÖ$6'&–W$¶–ÆÆVB€Ð¢×fÔ&öÖ$6'&–W$¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÕ6VçG'”'W7FW$FWFöæFRÓâvÖTWfVçC£¤×fÕ6VçG'”'W7FW$FWFöæFR€Ð¢×fÕ6VçG'”'W7FW$FWFöæFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÕ66÷WDÖ&¶VDf÷$FVF‚ÓâvÖTWfVçC£¤×fÕ66÷WDÖ&¶VDf÷$FVF‚€Ð¢×fÕ66÷WDÖ&¶VDf÷$FVF„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÔÖVF–5÷vW%W6†&VBÓâvÖTWfVçC£¤×fÔÖVF–5÷vW%W6†&VB€Ð¢×fÔÖVF–5÷vW%W6†&VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÔ&Vv–åvfRÓâ°Ð¢vÖTWfVçC£¤×fÔ&Vv–åvfR„×fÔ&Vv–åvfTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤×fÕvfT6ö×ÆWFRÓâ°Ð¢vÖTWfVçC£¤×fÕvfT6ö×ÆWFR„×fÕvfT6ö×ÆWFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤×fÔÖ—76–öä6ö×ÆWFRÓâ°Ð¢vÖTWfVçC£¤×fÔÖ—76–öä6ö×ÆWFR„×fÔÖ—76–öä6ö×ÆWFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤×fÔ&öÖ%&W6WD'•Æ–W"ÓâvÖTWfVçC£¤×fÔ&öÖ%&W6WD'•Æ–W"€Ð¢×fÔ&öÖ%&W6WD'•Æ–W$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÔ&öÖ$Æ&ÕG&–vvW&VBÓâvÖTWfVçC£¤×fÔ&öÖ$Æ&ÕG&–vvW&VB€Ð¢×fÔ&öÖ$Æ&ÕG&–vvW&VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W"ÓâvÖTWfVçC£¤×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W"€Ð¢×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÕvfTf–ÆVBÓâ°Ð¢vÖTWfVçC£¤×fÕvfTf–ÆVB„×fÕvfTf–ÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤×fÕ&W6WE7FG2Óâ°Ð¢vÖTWfVçC£¤×fÕ&W6WE7FG2„×fÕ&W6WE7FG4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤FÖvU&W6—7FVBÓâ°Ð¢vÖTWfVçC£¤FÖvU&W6—7FVB„FÖvU&W6—7FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&Wf—fUÆ–W$æ÷F–g’Óâ°Ð¢vÖTWfVçC£¥&Wf—fUÆ–W$æ÷F–g’…&Wf—fUÆ–W$æ÷F–g”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&Wf—fUÆ–W%7F÷VBÓâ°Ð¢vÖTWfVçC£¥&Wf—fUÆ–W%7F÷VB…&Wf—fUÆ–W%7F÷VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&Wf—fUÆ–W$6ö×ÆWFRÓâvÖTWfVçC£¥&Wf—fUÆ–W$6ö×ÆWFR€Ð¢&Wf—fUÆ–W$6ö×ÆWFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W%GW&æVEFôv†÷7BÓâ°Ð¢vÖTWfVçC£¥Æ–W%GW&æVEFôv†÷7B…Æ–W%GW&æVEFôv†÷7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖVF–wVå6†–VÆD&Æö6¶VDFÖvRÓâvÖTWfVçC£¤ÖVF–wVå6†–VÆD&Æö6¶VDFÖvR€Ð¢ÖVF–wVå6†–VÆD&Æö6¶VDFÖvTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÔGevfT6ö×ÆWFTæôvFW2ÓâvÖTWfVçC£¤×fÔGevfT6ö×ÆWFTæôvFW2€Ð¢×fÔGevfT6ö×ÆWFTæôvFW4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÕ6æ—W$†VG6†÷D7W'&Væ7’ÓâvÖTWfVçC£¤×fÕ6æ—W$†VG6†÷D7W'&Væ7’€Ð¢×fÕ6æ—W$†VG6†÷D7W'&Væ7”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÔÖææ†GFå—BÓâ°Ð¢vÖTWfVçC£¤×fÔÖææ†GFå—B„×fÔÖææ†GFå—DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤fÆt6'&–VD–äFWFV7F–öå¦öæRÓâvÖTWfVçC£¤fÆt6'&–VD–äFWFV7F–öå¦öæR€Ð¢fÆt6'&–VD–äFWFV7F–öå¦öæTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤×fÔGevfT¶–ÆÆVE7GVå&F–òÓâvÖTWfVçC£¤×fÔGevfT¶–ÆÆVE7GVå&F–ò€Ð¢×fÔGevfT¶–ÆÆVE7GVå&F–ôWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W$F—&V7D†—E7GVâÓâ°Ð¢vÖTWfVçC£¥Æ–W$F—&V7D†—E7GVâ…Æ–W$F—&V7D†—E7GVäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤×fÕ6VçG'”'W7FW$¶–ÆÆVBÓâvÖTWfVçC£¤×fÕ6VçG'”'W7FW$¶–ÆÆVB€Ð¢×fÕ6VçG'”'W7FW$¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Ww&FW4f–ÆT6†ævVBÓâ°Ð¢vÖTWfVçC£¥Ww&FW4f–ÆT6†ævVB…Ww&FW4f–ÆT6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&EFVÕö–çG46†ævVBÓâ°Ð¢vÖTWfVçC£¥&EFVÕö–çG46†ævVB…&EFVÕö–çG46†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&E'VÆW57FFT6†ævVBÓâ°Ð¢vÖTWfVçC£¥&E'VÆW57FFT6†ævVB…&E'VÆW57FFT6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&E&ö&÷D¶–ÆÆVBÓâ°Ð¢vÖTWfVçC£¥&E&ö&÷D¶–ÆÆVB…&E&ö&÷D¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&E&ö&÷D–×7BÓâ°Ð¢vÖTWfVçC£¥&E&ö&÷D–×7B…&E&ö&÷D–×7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÕÆ•&U&÷VæEF–ÖTÆVgBÓâvÖTWfVçC£¥FVÕÆ•&U&÷VæEF–ÖTÆVgB€Ð¢FVÕÆ•&U&÷VæEF–ÖTÆVgDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥&6‡WFTFWÆ÷’Óâ°Ð¢vÖTWfVçC£¥&6‡WFTFWÆ÷’…&6‡WFTFWÆ÷”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&6‡WFT†öÇ7FW"Óâ°Ð¢vÖTWfVçC£¥&6‡WFT†öÇ7FW"…&6‡WFT†öÇ7FW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤¶–ÆÅ&Vf–ÆÇ4ÖWFW"Óâ°Ð¢vÖTWfVçC£¤¶–ÆÅ&Vf–ÆÇ4ÖWFW"„¶–ÆÅ&Vf–ÆÇ4ÖWFW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥'5FVçDWfVçBÓâ°Ð¢vÖTWfVçC£¥'5FVçDWfVçB…'5FVçDWfVçDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6öæv¶–ÆÂÓâ°Ð¢vÖTWfVçC£¤6öæv¶–ÆÂ„6öæv¶–ÆÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$–æ—F–Å7vâÓâ°Ð¢vÖTWfVçC£¥Æ–W$–æ—F–Å7vâ…Æ–W$–æ—F–Å7väWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6ö×WF—F—fUf–7F÷'’Óâ°Ð¢vÖTWfVçC£¤6ö×WF—F—fUf–7F÷'’„6ö×WF—F—fUf–7F÷'”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6ö×WF—F—fU7FG5WFFRÓâvÖTWfVçC£¤6ö×WF—F—fU7FG5WFFR€Ð¢6ö×WF—F—fU7FG5WFFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤Ö–æ”vÖUv–âÓâ°Ð¢vÖTWfVçC£¤Ö–æ”vÖUv–â„Ö–æ”vÖUv–äWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6VçG'”öävô7F—fRÓâ°Ð¢vÖTWfVçC£¥6VçG'”öävô7F—fR…6VçG'”öävô7F—fTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤GV6µ‡ÆWfVÅWÓâ°Ð¢vÖTWfVçC£¤GV6µ‡ÆWfVÅW„GV6µ‡ÆWfVÅWWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥VW7DÆöt÷VæVBÓâ°Ð¢vÖTWfVçC£¥VW7DÆöt÷VæVB…VW7DÆöt÷VæVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥66†VÖWFFVBÓâ°Ð¢vÖTWfVçC£¥66†VÖWFFVB…66†VÖWFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤Æö6ÅÆ–W%–6·WvVöâÓâvÖTWfVçC£¤Æö6ÅÆ–W%–6·WvVöâ€Ð¢Æö6ÅÆ–W%–6·WvVöäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥&EÆ–W%66÷&Uö–çG2Óâ°Ð¢vÖTWfVçC£¥&EÆ–W%66÷&Uö–çG2…&EÆ–W%66÷&Uö–çG4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤FVÖöÖäFWE7F–6¶–W2Óâ°Ð¢vÖTWfVçC£¤FVÖöÖäFWE7F–6¶–W2„FVÖöÖäFWE7F–6¶–W4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥VW7Dö&¦V7F—fT6ö×ÆWFVBÓâvÖTWfVçC£¥VW7Dö&¦V7F—fT6ö×ÆWFVB€Ð¢VW7Dö&¦V7F—fT6ö×ÆWFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W%66÷&T6†ævVBÓâ°Ð¢vÖTWfVçC£¥Æ–W%66÷&T6†ævVB…Æ–W%66÷&T6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤¶–ÆÆVD6–æuÆ–W"Óâ°Ð¢vÖTWfVçC£¤¶–ÆÆVD6–æuÆ–W"„¶–ÆÆVD6–æuÆ–W$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤Vçf—&öæÖVçFÄFVF‚Óâ°Ð¢vÖTWfVçC£¤Vçf—&öæÖVçFÄFVF‚„Vçf—&öæÖVçFÄFVF„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&ö¦V7F–ÆTF—&V7D†—BÓâ°Ð¢vÖTWfVçC£¥&ö¦V7F–ÆTF—&V7D†—B…&ö¦V7F–ÆTF—&V7D†—DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥74vWBÓâvÖTWfVçC£¥74vWB…74vWDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’ÀÐ¢vÖTWfVçEG—S£¥7566÷&RÓâ°Ð¢vÖTWfVçC£¥7566÷&R…7566÷&TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥74g&VRÓâ°Ð¢vÖTWfVçC£¥74g&VR…74g&VTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥75746Vv‡BÓâ°Ð¢vÖTWfVçC£¥75746Vv‡B…75746Vv‡DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥74&ÆÅ7FöÆVâÓâ°Ð¢vÖTWfVçC£¥74&ÆÅ7FöÆVâ…74&ÆÅ7FöÆVäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥74&ÆÄ&Æö6¶VBÓâ°Ð¢vÖTWfVçC£¥74&ÆÄ&Æö6¶VB…74&ÆÄ&Æö6¶VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤FÖvU&WfVçFVBÓâ°Ð¢vÖTWfVçC£¤FÖvU&WfVçFVB„FÖvU&WfVçFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVä&÷74¶–ÆÆVBÓâ°Ð¢vÖTWfVçC£¤†ÆÆ÷vVVä&÷74¶–ÆÆVB„†ÆÆ÷vVVä&÷74¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W66VDÆö÷D—6ÆæBÓâ°Ð¢vÖTWfVçC£¤W66VDÆö÷D—6ÆæB„W66VDÆö÷D—6ÆæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FvvVEÆ–W$4—BÓâ°Ð¢vÖTWfVçC£¥FvvVEÆ–W$4—B…FvvVEÆ–W$4—DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖW&6×W57GVææVBÓâ°Ð¢vÖTWfVçC£¤ÖW&6×W57GVææVB„ÖW&6×W57GVææVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖW&6×W5&÷f÷VæBÓâ°Ð¢vÖTWfVçC£¤ÖW&6×W5&÷f÷VæB„ÖW&6×W5&÷f÷VæDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVBÓâvÖTWfVçC£¤†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVB€Ð¢†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥6¶VÆWFöä¶–ÆÆVEVW7BÓâ°Ð¢vÖTWfVçC£¥6¶VÆWFöä¶–ÆÆVEVW7B…6¶VÆWFöä¶–ÆÆVEVW7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6¶VÆWFöä¶–æt¶–ÆÆVEVW7BÓâvÖTWfVçC£¥6¶VÆWFöä¶–æt¶–ÆÆVEVW7B€Ð¢6¶VÆWFöä¶–æt¶–ÆÆVEVW7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤W66T†VÆÂÓâ°Ð¢vÖTWfVçC£¤W66T†VÆÂ„W66T†VÆÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤7&÷757V7G&Ä'&–FvRÓâ°Ð¢vÖTWfVçC£¤7&÷757V7G&Ä'&–FvR„7&÷757V7G&Ä'&–FvTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤Ö–æ”vÖUvöâÓâ°Ð¢vÖTWfVçC£¤Ö–æ”vÖUvöâ„Ö–æ”vÖUvöäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&W7väv†÷7BÓâ°Ð¢vÖTWfVçC£¥&W7väv†÷7B…&W7väv†÷7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤¶–ÆÄ–ä†VÆÂÓâ°Ð¢vÖTWfVçC£¤¶–ÆÄ–ä†VÆÂ„¶–ÆÄ–ä†VÆÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVäGV6´6öÆÆV7FVBÓâvÖTWfVçC£¤†ÆÆ÷vVVäGV6´6öÆÆV7FVB€Ð¢†ÆÆ÷vVVäGV6´6öÆÆV7FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥7V6–Å66÷&RÓâ°Ð¢vÖTWfVçC£¥7V6–Å66÷&R…7V6–Å66÷&TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥FVÔÆVFW$¶–ÆÆVBÓâ°Ð¢vÖTWfVçC£¥FVÔÆVFW$¶–ÆÆVB…FVÔÆVFW$¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVBÓâvÖTWfVçC£¤†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVB€Ð¢†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥&V6Æ7VÆFUG'V6RÓâ°Ð¢vÖTWfVçC£¥&V6Æ7VÆFUG'V6R…&V6Æ7VÆFUG'V6TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤FVE&–ævW$6†VDFVF‚ÓâvÖTWfVçC£¤FVE&–ævW$6†VDFVF‚€Ð¢FVE&–ævW$6†VDFVF„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤7&÷76&÷t†VÂÓâ°Ð¢vÖTWfVçC£¤7&÷76&÷t†VÂ„7&÷76&÷t†VÄWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤FÖvTÖ—F–vFVBÓâ°Ð¢vÖTWfVçC£¤FÖvTÖ—F–vFVB„FÖvTÖ—F–vFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥–ÆöEW6†VBÓâ°Ð¢vÖTWfVçC£¥–ÆöEW6†VB…–ÆöEW6†VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$&æFöæVDÖF6‚ÓâvÖTWfVçC£¥Æ–W$&æFöæVDÖF6‚€Ð¢Æ–W$&æFöæVDÖF6„WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6ÄG&vÆ–æRÓâ°Ð¢vÖTWfVçC£¤6ÄG&vÆ–æR„6ÄG&vÆ–æTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&W7F'EF–ÖW%F–ÖRÓâ°Ð¢vÖTWfVçC£¥&W7F'EF–ÖW%F–ÖR…&W7F'EF–ÖW%F–ÖTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥v–äÆ–Ö—D6†ævVBÓâ°Ð¢vÖTWfVçC£¥v–äÆ–Ö—D6†ævVB…v–äÆ–Ö—D6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥v–åæVÅ6†÷u66÷&W2Óâ°Ð¢vÖTWfVçC£¥v–åæVÅ6†÷u66÷&W2…v–åæVÅ6†÷u66÷&W4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥F÷7G&V×5&WVW7Df–æ—6†VBÓâvÖTWfVçC£¥F÷7G&V×5&WVW7Df–æ—6†VB€Ð¢F÷7G&V×5&WVW7Df–æ—6†VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤6ö×WF—F—fU7FFT6†ævVBÓâvÖTWfVçC£¤6ö×WF—F—fU7FFT6†ævVB€Ð¢6ö×WF—F—fU7FFT6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¤vÆö&Åv$FFWFFVBÓâvÖTWfVçC£¤vÆö&Åv$FFWFFVB€Ð¢vÆö&Åv$FFWFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥7F÷vF6„6†ævVBÓâ°Ð¢vÖTWfVçC£¥7F÷vF6„6†ævVB…7F÷vF6„6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤G57F÷ÓâvÖTWfVçC£¤G57F÷„G57F÷WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“ò’ÀÐ¢vÖTWfVçEG—S£¤G567&VVç6†÷BÓâ°Ð¢vÖTWfVçC£¤G567&VVç6†÷B„G567&VVç6†÷DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥6†÷tÖF6…7VÖÖ'’Óâ°Ð¢vÖTWfVçC£¥6†÷tÖF6…7VÖÖ'’…6†÷tÖF6…7VÖÖ'”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤W‡W&–Væ6T6†ævVBÓâ°Ð¢vÖTWfVçC£¤W‡W&–Væ6T6†ævVB„W‡W&–Væ6T6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤&Vv–å‡ÆW'Óâ°Ð¢vÖTWfVçC£¤&Vv–å‡ÆW'„&Vv–å‡ÆW'WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÖF6†Ö¶W%7FG5WFFVBÓâvÖTWfVçC£¤ÖF6†Ö¶W%7FG5WFFVB€Ð¢ÖF6†Ö¶W%7FG5WFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥&VÖF6…f÷FUW&–öD÷fW"ÓâvÖTWfVçC£¥&VÖF6…f÷FUW&–öD÷fW"€Ð¢&VÖF6…f÷FUW&–öD÷fW$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥&VÖF6„f–ÆVEFô7&VFRÓâvÖTWfVçC£¥&VÖF6„f–ÆVEFô7&VFR€Ð¢&VÖF6„f–ÆVEFô7&VFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥Æ–W%&VÖF6„6†ævRÓâ°Ð¢vÖTWfVçC£¥Æ–W%&VÖF6„6†ævR…Æ–W%&VÖF6„6†ævTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥–æuWFFVBÓâ°Ð¢vÖTWfVçC£¥–æuWFFVB…–æuWFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤ÔÕ7FG5WFFVBÓâ°Ð¢vÖTWfVçC£¤ÔÕ7FG5WFFVB„ÔÕ7FG5WFFVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$æW‡DÖf÷FT6†ævRÓâvÖTWfVçC£¥Æ–W$æW‡DÖf÷FT6†ævR€Ð¢Æ–W$æW‡DÖf÷FT6†ævTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥f÷FTÖ46†ævVBÓâ°Ð¢vÖTWfVçC£¥f÷FTÖ46†ævVB…f÷FTÖ46†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&÷FôFVd6†ævVBÓâ°Ð¢vÖTWfVçC£¥&÷FôFVd6†ævVB…&÷FôFVd6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W$FöÖ–æF–öâÓâ°Ð¢vÖTWfVçC£¥Æ–W$FöÖ–æF–öâ…Æ–W$FöÖ–æF–öäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Æ–W%&ö6¶WE6µW6†VBÓâvÖTWfVçC£¥Æ–W%&ö6¶WE6µW6†VB€Ð¢Æ–W%&ö6¶WE6µW6†VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥VW7E&WVW7BÓâ°Ð¢vÖTWfVçC£¥VW7E&WVW7B…VW7E&WVW7DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥VW7E&W7öç6RÓâ°Ð¢vÖTWfVçC£¥VW7E&W7öç6R…VW7E&W7öç6TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥VW7E&öw&W72Óâ°Ð¢vÖTWfVçC£¥VW7E&öw&W72…VW7E&öw&W74WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&ö¦V7F–ÆU&VÖ÷fVBÓâ°Ð¢vÖTWfVçC£¥&ö¦V7F–ÆU&VÖ÷fVB…&ö¦V7F–ÆU&VÖ÷fVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥VW7DÖFF6†ævVBÓâ°Ð¢vÖTWfVçC£¥VW7DÖFF6†ævVB…VW7DÖFF6†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤v4F÷W6VEÆ–W$–væ—FVBÓâvÖTWfVçC£¤v4F÷W6VEÆ–W$–væ—FVB€Ð¢v4F÷W6VEÆ–W$–væ—FVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥VW7EGW&ä–å7FFRÓâ°Ð¢vÖTWfVçC£¥VW7EGW&ä–å7FFR…VW7EGW&ä–å7FFTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤—FV×46¶æ÷vÆVFvVBÓâ°Ð¢vÖTWfVçC£¤—FV×46¶æ÷vÆVFvVB„—FV×46¶æ÷vÆVFvVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤6W$¶–ÆÆVBÓâ°Ð¢vÖTWfVçC£¤6W$¶–ÆÆVB„6W$¶–ÆÆVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤Ö–äÖVçU7F&–Æ—¦VBÓâ°Ð¢vÖTWfVçC£¤Ö–äÖVçU7F&–Æ—¦VB„Ö–äÖVçU7F&–Æ—¦VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥v÷&ÆE7FGW46†ævVBÓâ°Ð¢vÖTWfVçC£¥v÷&ÆE7FGW46†ævVB…v÷&ÆE7FGW46†ævVDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEe7FGW2Óâ°Ð¢vÖTWfVçC£¤„ÅEe7FGW2„„ÅEe7FGW4WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEd6ÖW&ÖâÓâ°Ð¢vÖTWfVçC£¤„ÅEd6ÖW&Öâ„„ÅEd6ÖW&ÖäWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEe&æ´6ÖW&Óâ°Ð¢vÖTWfVçC£¤„ÅEe&æ´6ÖW&„„ÅEe&æ´6ÖW&WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEe&æ´VçF—G’Óâ°Ð¢vÖTWfVçC£¤„ÅEe&æ´VçF—G’„„ÅEe&æ´VçF—G”WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEdf—†VBÓâ°Ð¢vÖTWfVçC£¤„ÅEdf—†VB„„ÅEdf—†VDWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEd6†6RÓâ°Ð¢vÖTWfVçC£¤„ÅEd6†6R„„ÅEd6†6TWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEdÖW76vRÓâ°Ð¢vÖTWfVçC£¤„ÅEdÖW76vR„„ÅEdÖW76vTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEeF—FÆRÓâ°Ð¢vÖTWfVçC£¤„ÅEeF—FÆR„„ÅEeF—FÆTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¤„ÅEd6†BÓâ°Ð¢vÖTWfVçC£¤„ÅEd6†B„„ÅEd6†DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&WÆ•7F'E&V6÷&BÓâ°Ð¢vÖTWfVçC£¥&WÆ•7F'E&V6÷&B…&WÆ•7F'E&V6÷&DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&WÆ•6W76–öä–æfòÓâ°Ð¢vÖTWfVçC£¥&WÆ•6W76–öä–æfò…&WÆ•6W76–öä–æfôWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&WÆ”VæE&V6÷&BÓâ°Ð¢vÖTWfVçC£¥&WÆ”VæE&V6÷&B…&WÆ”VæE&V6÷&DWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥&WÆ•&WÆ—4f–Æ&ÆRÓâvÖTWfVçC£¥&WÆ•&WÆ—4f–Æ&ÆR€Ð¢&WÆ•&WÆ—4f–Æ&ÆTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÀÐ¢’ÀÐ¢vÖTWfVçEG—S£¥&WÆ•6W'fW$W'&÷"Óâ°Ð¢vÖTWfVçC£¥&WÆ•6W'fW$W'&÷"…&WÆ•6W'fW$W'&÷$WfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢vÖTWfVçEG—S£¥Væ¶æ÷vâ…ò’Óâ°Ð¢vÖTWfVçC£¥Væ¶æ÷vâ…&tvÖTWfVçC£§&VB‡7G&VÒÂFVf–æ—F–öâ“òÐ¢ÐÐ¢ÒÐ¢ÐÐ¢V"fâw&—FR€Ð¢g6VÆbÀÐ¢7G&VÓ¢f×WB&—Ew&—FU7G&VÓÄÆ—GFÆTVæF–ãâÀÐ¢FVf–æ—F–öã¢dvÖTWfVçDFVf–æ—F–öâÀÐ¢’Óâ&W7VÇCÂ‚“â°Ð¢ÖF6‚g6VÆb°Ð¢vÖTWfVçC£¥6W'fW%7vâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6W'fW$6†ævTÆWfVÄf–ÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6W'fW%6‡WFF÷vâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6W'fW$7f"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6W'fW$ÖW76vR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6W'fW$FD&â†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6W'fW%&VÖ÷fT&â†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6öææV7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6öææV7D6Æ–VçB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$–æfò†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$F—66öææV7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$7F—fFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%6’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6Æ–VçDF—66öææV7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6Æ–VçD&Vv–ä6öææV7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6Æ–VçD6öææV7FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6Æ–VçDgVÆÄ6öææV7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤†÷7EV—B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÔ–æfò†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕ66÷&R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ”'&öF67DVF–ò†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%FVÒ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6Æ72†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$FVF‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$‡W'B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6†B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%66÷&R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%7vâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%6†ö÷B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%W6R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6†ævTæÖR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$†–çDÖW76vR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤&6UÆ–W%FVÆW÷'FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤vÖT–æ—B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤vÖTæWtÖ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤vÖU7F'B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤vÖTVæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&÷VæE7F'B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&÷VæDVæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤vÖTÖW76vR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤'&V´'&V¶&ÆR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤'&Vµ&÷†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤VçF—G”¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤&öçW5WFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6†–WfVÖVçDWfVçB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6†–WfVÖVçD–æ7&VÖVçB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥‡—6wVå–6·W†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤fÆ&T–væ—FTç2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤†VÆ–6÷FW$w&VæFUVçDÖ—72†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥W6W$FFF÷væÆöFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&vFöÆÄF—76öÇfVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEd6†ævVDÖöFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEd6†ævVEF&vWB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥f÷FTVæFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥f÷FU7F'FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥f÷FT6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥f÷FU76VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥f÷FTf–ÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥f÷FT67B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥f÷FT÷F–öç2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&WÆ•6fVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤VçFW&VEW&f÷&Öæ6TÖöFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤'&÷w6U&WÆ—2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&WÆ•–÷WGV&U7FG2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤–çfVçF÷'•WFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6'EWFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥7F÷&U&–6U6†VWEWFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤V6öä–çfVçF÷'”6öææV7FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤—FVÕ66†VÖ–æ—F–Æ—¦VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤v4æWu6W76–öâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤v4Æ÷7E6W76–öâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤–çG&ôf–æ—6‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤–çG&ôæW‡D6ÖW&†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6†ævT6Æ72†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FdÖF–ÖU&VÖ–æ–ær†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FdvÖT÷fW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤7FdfÆt6GW&VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çD–æ—F–Æ—¦VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEWFFT–ÖvW2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEWFFTÆ–÷WB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEWFFT6–ær†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEWFFT÷væW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çE7F'EF÷V6‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çDVæEF÷V6‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEVÇ6TVÆVÖVçB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çDf¶T6GW&R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæE6VÆV7FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæE7F'B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæD7F—fR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•v—F–æt&Vv–ç2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•v—F–ætVæG2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•v—F–æt&÷WEFôVæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&W7F'E&÷VæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&VG•&W7F'B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæE&W7F'E6V6öæG2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•FVÕ&VG’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæEv–â†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•WFFUF–ÖW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæE7FÆVÖFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ”÷fW'F–ÖT&Vv–â†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ”÷fW'F–ÖTVæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•7VFFVäFVF„&Vv–â†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•7VFFVäFVF„VæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ”vÖT÷fW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ”ÖF–ÖU&VÖ–æ–ær†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•F–ÖW$fÆ6‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•F–ÖW%F–ÖTFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•ö–çE7F'D6GW&R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•ö–çD6GW&VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•ö–çDÆö6¶VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•ö–çEVæÆö6¶VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ”6GW&T'&ö¶Vâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ”6GW&T&Æö6¶VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ”fÆtWfVçB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•v–åæVÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•FVÔ&Ææ6VEÆ–W"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•6WGWf–æ—6†VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ”ÆW'B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥G&–æ–æt6ö×ÆWFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6†÷tg&VW¦UæVÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤†–FTg&VW¦UæVÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤g&VW¦T6Õ7F'FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$6†ævUFVÒ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W%66÷&T6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$6†ævT6Æ72†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W%&W7vâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤'V–ÆF–æt–æfô6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$6†ævTF—6wV—6R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$66÷VçD6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥7•F&W6WB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤fÆu7FGW5WFFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%7FG5WFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ––æt6öÖÖVçF'’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6†&vTFWÆ÷–VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$'V–ÇDö&¦V7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%Ww&FVDö&¦V7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6''”ö&¦V7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$G&÷ö&¦V7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ö&¦V7E&VÖ÷fVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ö&¦V7DFW7G&÷–VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ö&¦V7DFWFöæFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6†–WfVÖVçDV&æVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥7V5F&vWEWFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥F÷W&æÖVçE7FFUWFFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6ÆÆVDf÷$ÖVF–2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$6¶VDf÷$&ÆÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$&V6ÖTö'6W'fW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$–væ—FVD–çb†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$–væ—FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$W‡F–æwV—6†VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%FVÆW÷'FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$†VÆVDÖVF–46ÆÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$6†&vU&VG’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W%v–æDF÷vâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$–çgVÆæVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W66÷'E7VVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W66÷'E&öw&W72†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W66÷'E&V6VFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤vÖUT”7F—fFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤vÖUT”†–FFVâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$W66÷'E66÷&R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$†VÄöä†—B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%7FVÅ6æGf–6‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6†÷t6Æ74Æ–÷WB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6†÷ug5æVÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$FÖvVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤&VæÆ–W$æ÷F–f–6F–öâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤&VæÖF6„Ö…7G&V²†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤&Væ&÷VæE7F'B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤&Væv–åæVÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥fUv–åæVÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤—$F6‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÆæFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$FÖvTFöFvVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%7GVææVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥66÷WDw&æE6ÆÒ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥66÷WE6ÆÖFöÆÄÆæFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤'&÷t–×7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$¦&FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$¦&FVDfFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%6†–VÆD&Æö6¶VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%–ææVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$†VÆVD'”ÖVF–2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%6VDö&¦V7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤—FVÔf÷VæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6†÷tææ÷FF–öâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤†–FTææ÷FF–öâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥÷7D–çfVçF÷'”Æ–6F–öâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEVæÆö6µWFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤FWÆ÷”'Vfd&ææW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$'Vfb†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖVF–4FVF‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤÷fW'F–ÖTær†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FV×46†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVåV×¶–äw&"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&ö6¶WD§V×†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&ö6¶WD§V×ÆæFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥7F–6·”§V×†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥7F–6·”§V×ÆæFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&ö6¶WE6´ÆVæ6‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&ö6¶WE6´ÆæFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖVF–4FVfVæFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$†VÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$FW7G&÷–VE—T&öÖ"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ö&¦V7DFVfÆV7FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$×g†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&–E7väÖö"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&–E7vå7VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤æd&Æö6¶VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥F…G&6µ76VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤çVÔ6W'46†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%&VvVæW&FR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥WFFU7FGW4—FVÒ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥7FG5&W6WE&÷VæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥66÷&U7FG467V×VÆFVEWFFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥66÷&U7FG467V×VÆFVE&W6WB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6†–WfVÖVçDV&æVDÆö6Â†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$†VÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤'V–ÆF–æt†VÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤—FVÕ–6·W†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤GVVÅ7FGW2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤f—6„æ÷F–6R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤f—6„æ÷F–6T&Ò†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6Ææ÷F–6R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥F‡&÷v&ÆT†—B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥V×¶–äÆ÷&E7VÖÖöæVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥V×¶–äÆ÷&D¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖW&6×W57VÖÖöæVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖW&6×W4¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖW&6×W4W66Uv&æ–ær†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖW&6×W4W66VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷757VÖÖöæVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷757GVææVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷74¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷74¶–ÆÆW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷74W66T–ÖÖ–æVçB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷74W66VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ç4‡W'B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEF–ÖW%WFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$†–v„f—fU7F'B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$†–v„f—fT6æ6VÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$†–v„f—fU7V66W72†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$&öçW5ö–çG2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%Ww&FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$'W–&6²†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%W6VE÷vW%W&÷GFÆR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6‡&—7FÖ4v–gDw&"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥'G•WFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥'G•&Vd6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥'G”7&—FW&–6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥'G”–çf—FW46†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥'G•VWVU7FFT6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥'G”6†B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥'G”ÖVÖ&W$¦ö–â†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥'G”ÖVÖ&W$ÆVfR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖF6„–çf—FW5WFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö&'•WFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔÖ—76–öåWFFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&V6Æ7VÆFT†öÆ–F—2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$7W'&Væ7”6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Föö×6F•&ö6¶WD÷Vâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&VÖ÷fTæVÖW6—5&VÆF–öç6†—2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔ7&VF—D&öçW5vfR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔ7&VF—D&öçW4ÆÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔ7&VF—D&öçW4ÆÄGfæ6VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕV–6µ6VçG'•Ww&FR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕFæ´FW7G&÷–VD'•Æ–W'2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕ–6·W7W'&Væ7’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔ&öÖ$6'&–W$¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕ6VçG'”'W7FW$FWFöæFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕ66÷WDÖ&¶VDf÷$FVF‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔÖVF–5÷vW%W6†&VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔ&Vv–åvfR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕvfT6ö×ÆWFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔÖ—76–öä6ö×ÆWFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔ&öÖ%&W6WD'•Æ–W"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔ&öÖ$Æ&ÕG&–vvW&VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕvfTf–ÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕ&W6WE7FG2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤FÖvU&W6—7FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&Wf—fUÆ–W$æ÷F–g’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&Wf—fUÆ–W%7F÷VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&Wf—fUÆ–W$6ö×ÆWFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%GW&æVEFôv†÷7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖVF–wVå6†–VÆD&Æö6¶VDFÖvR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔGevfT6ö×ÆWFTæôvFW2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕ6æ—W$†VG6†÷D7W'&Væ7’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔÖææ†GFå—B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤fÆt6'&–VD–äFWFV7F–öå¦öæR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÔGevfT¶–ÆÆVE7GVå&F–ò†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$F—&V7D†—E7GVâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤×fÕ6VçG'”'W7FW$¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Ww&FW4f–ÆT6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&EFVÕö–çG46†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&E'VÆW57FFT6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&E&ö&÷D¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&E&ö&÷D–×7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&U&÷VæEF–ÖTÆVgB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&6‡WFTFWÆ÷’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&6‡WFT†öÇ7FW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤¶–ÆÅ&Vf–ÆÇ4ÖWFW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥'5FVçDWfVçB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6öæv¶–ÆÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$–æ—F–Å7vâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6ö×WF—F—fUf–7F÷'’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6ö×WF—F—fU7FG5WFFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Ö–æ”vÖUv–â†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6VçG'”öävô7F—fR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤GV6µ‡ÆWfVÅW†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥VW7DÆöt÷VæVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥66†VÖWFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W%–6·WvVöâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&EÆ–W%66÷&Uö–çG2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤FVÖöÖäFWE7F–6¶–W2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥VW7Dö&¦V7F—fT6ö×ÆWFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%66÷&T6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤¶–ÆÆVD6–æuÆ–W"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Vçf—&öæÖVçFÄFVF‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&ö¦V7F–ÆTF—&V7D†—B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥74vWB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥7566÷&R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥74g&VR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥75746Vv‡B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥74&ÆÅ7FöÆVâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥74&ÆÄ&Æö6¶VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤FÖvU&WfVçFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVä&÷74¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W66VDÆö÷D—6ÆæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FvvVEÆ–W$4—B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖW&6×W57GVææVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖW&6×W5&÷f÷VæB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6¶VÆWFöä¶–ÆÆVEVW7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6¶VÆWFöä¶–æt¶–ÆÆVEVW7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W66T†VÆÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤7&÷757V7G&Ä'&–FvR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Ö–æ”vÖUvöâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&W7väv†÷7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤¶–ÆÄ–ä†VÆÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVäGV6´6öÆÆV7FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥7V6–Å66÷&R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥FVÔÆVFW$¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&V6Æ7VÆFUG'V6R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤FVE&–ævW$6†VDFVF‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤7&÷76&÷t†VÂ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤FÖvTÖ—F–vFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥–ÆöEW6†VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$&æFöæVDÖF6‚†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6ÄG&vÆ–æR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&W7F'EF–ÖW%F–ÖR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥v–äÆ–Ö—D6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥v–åæVÅ6†÷u66÷&W2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥F÷7G&V×5&WVW7Df–æ—6†VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6ö×WF—F—fU7FFT6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤vÆö&Åv$FFWFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥7F÷vF6„6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤G57F÷†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤G567&VVç6†÷B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥6†÷tÖF6…7VÖÖ'’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤W‡W&–Væ6T6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤&Vv–å‡ÆW'†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÖF6†Ö¶W%7FG5WFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&VÖF6…f÷FUW&–öD÷fW"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&VÖF6„f–ÆVEFô7&VFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%&VÖF6„6†ævR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥–æuWFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤ÔÕ7FG5WFFVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$æW‡DÖf÷FT6†ævR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥f÷FTÖ46†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&÷FôFVd6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W$FöÖ–æF–öâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Æ–W%&ö6¶WE6µW6†VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥VW7E&WVW7B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥VW7E&W7öç6R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥VW7E&öw&W72†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&ö¦V7F–ÆU&VÖ÷fVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥VW7DÖFF6†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤v4F÷W6VEÆ–W$–væ—FVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥VW7EGW&ä–å7FFR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤—FV×46¶æ÷vÆVFvVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤6W$¶–ÆÆVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤Ö–äÖVçU7F&–Æ—¦VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥v÷&ÆE7FGW46†ævVB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEe7FGW2†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEd6ÖW&Öâ†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEe&æ´6ÖW&†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEe&æ´VçF—G’†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEdf—†VB†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEd6†6R†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEdÖW76vR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEeF—FÆR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¤„ÅEd6†B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&WÆ•7F'E&V6÷&B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&WÆ•6W76–öä–æfò†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&WÆ”VæE&V6÷&B†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&WÆ•&WÆ—4f–Æ&ÆR†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥&WÆ•6W'fW$W'&÷"†WfVçB’ÓâWfVçBçw&—FR‡7G&VÒÂFVf–æ—F–öâ’ÀÐ¢vÖTWfVçC£¥Væ¶æ÷vâ‡&r’Óâö²‡&rçw&—FR‡7G&VÒ“ò’ÀÐ¢ÐÐ¢ÐÐ¢V"fâWfVçE÷G—R‚g6VÆb’ÓâvÖTWfVçEG—R°Ð¢ÖF6‚g6VÆb°Ð¢vÖTWfVçC£¥6W'fW%7vâ…ò’ÓâvÖTWfVçEG—S£¥6W'fW%7vâÀÐ¢vÖTWfVçC£¥6W'fW$6†ævTÆWfVÄf–ÆVB…ò’ÓâvÖTWfVçEG—S£¥6W'fW$6†ævTÆWfVÄf–ÆVBÀÐ¢vÖTWfVçC£¥6W'fW%6‡WFF÷vâ…ò’ÓâvÖTWfVçEG—S£¥6W'fW%6‡WFF÷vâÀÐ¢vÖTWfVçC£¥6W'fW$7f"…ò’ÓâvÖTWfVçEG—S£¥6W'fW$7f"ÀÐ¢vÖTWfVçC£¥6W'fW$ÖW76vR…ò’ÓâvÖTWfVçEG—S£¥6W'fW$ÖW76vRÀÐ¢vÖTWfVçC£¥6W'fW$FD&â…ò’ÓâvÖTWfVçEG—S£¥6W'fW$FD&âÀÐ¢vÖTWfVçC£¥6W'fW%&VÖ÷fT&â…ò’ÓâvÖTWfVçEG—S£¥6W'fW%&VÖ÷fT&âÀÐ¢vÖTWfVçC£¥Æ–W$6öææV7B…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6öææV7BÀÐ¢vÖTWfVçC£¥Æ–W$6öææV7D6Æ–VçB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6öææV7D6Æ–VçBÀÐ¢vÖTWfVçC£¥Æ–W$–æfò…ò’ÓâvÖTWfVçEG—S£¥Æ–W$–æfòÀÐ¢vÖTWfVçC£¥Æ–W$F—66öææV7B…ò’ÓâvÖTWfVçEG—S£¥Æ–W$F—66öææV7BÀÐ¢vÖTWfVçC£¥Æ–W$7F—fFR…ò’ÓâvÖTWfVçEG—S£¥Æ–W$7F—fFRÀÐ¢vÖTWfVçC£¥Æ–W%6’…ò’ÓâvÖTWfVçEG—S£¥Æ–W%6’ÀÐ¢vÖTWfVçC£¤6Æ–VçDF—66öææV7B…ò’ÓâvÖTWfVçEG—S£¤6Æ–VçDF—66öææV7BÀÐ¢vÖTWfVçC£¤6Æ–VçD&Vv–ä6öææV7B…ò’ÓâvÖTWfVçEG—S£¤6Æ–VçD&Vv–ä6öææV7BÀÐ¢vÖTWfVçC£¤6Æ–VçD6öææV7FVB…ò’ÓâvÖTWfVçEG—S£¤6Æ–VçD6öææV7FVBÀÐ¢vÖTWfVçC£¤6Æ–VçDgVÆÄ6öææV7B…ò’ÓâvÖTWfVçEG—S£¤6Æ–VçDgVÆÄ6öææV7BÀÐ¢vÖTWfVçC£¤†÷7EV—B…ò’ÓâvÖTWfVçEG—S£¤†÷7EV—BÀÐ¢vÖTWfVçC£¥FVÔ–æfò…ò’ÓâvÖTWfVçEG—S£¥FVÔ–æfòÀÐ¢vÖTWfVçC£¥FVÕ66÷&R…ò’ÓâvÖTWfVçEG—S£¥FVÕ66÷&RÀÐ¢vÖTWfVçC£¥FVÕÆ”'&öF67DVF–ò…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ”'&öF67DVF–òÀÐ¢vÖTWfVçC£¥Æ–W%FVÒ…ò’ÓâvÖTWfVçEG—S£¥Æ–W%FVÒÀÐ¢vÖTWfVçC£¥Æ–W$6Æ72…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6Æ72ÀÐ¢vÖTWfVçC£¥Æ–W$FVF‚…ò’ÓâvÖTWfVçEG—S£¥Æ–W$FVF‚ÀÐ¢vÖTWfVçC£¥Æ–W$‡W'B…ò’ÓâvÖTWfVçEG—S£¥Æ–W$‡W'BÀÐ¢vÖTWfVçC£¥Æ–W$6†B…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6†BÀÐ¢vÖTWfVçC£¥Æ–W%66÷&R…ò’ÓâvÖTWfVçEG—S£¥Æ–W%66÷&RÀÐ¢vÖTWfVçC£¥Æ–W%7vâ…ò’ÓâvÖTWfVçEG—S£¥Æ–W%7vâÀÐ¢vÖTWfVçC£¥Æ–W%6†ö÷B…ò’ÓâvÖTWfVçEG—S£¥Æ–W%6†ö÷BÀÐ¢vÖTWfVçC£¥Æ–W%W6R…ò’ÓâvÖTWfVçEG—S£¥Æ–W%W6RÀÐ¢vÖTWfVçC£¥Æ–W$6†ævTæÖR…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6†ævTæÖRÀÐ¢vÖTWfVçC£¥Æ–W$†–çDÖW76vR…ò’ÓâvÖTWfVçEG—S£¥Æ–W$†–çDÖW76vRÀÐ¢vÖTWfVçC£¤&6UÆ–W%FVÆW÷'FVB…ò’ÓâvÖTWfVçEG—S£¤&6UÆ–W%FVÆW÷'FVBÀÐ¢vÖTWfVçC£¤vÖT–æ—B…ò’ÓâvÖTWfVçEG—S£¤vÖT–æ—BÀÐ¢vÖTWfVçC£¤vÖTæWtÖ…ò’ÓâvÖTWfVçEG—S£¤vÖTæWtÖÀÐ¢vÖTWfVçC£¤vÖU7F'B…ò’ÓâvÖTWfVçEG—S£¤vÖU7F'BÀÐ¢vÖTWfVçC£¤vÖTVæB…ò’ÓâvÖTWfVçEG—S£¤vÖTVæBÀÐ¢vÖTWfVçC£¥&÷VæE7F'B…ò’ÓâvÖTWfVçEG—S£¥&÷VæE7F'BÀÐ¢vÖTWfVçC£¥&÷VæDVæB…ò’ÓâvÖTWfVçEG—S£¥&÷VæDVæBÀÐ¢vÖTWfVçC£¤vÖTÖW76vR…ò’ÓâvÖTWfVçEG—S£¤vÖTÖW76vRÀÐ¢vÖTWfVçC£¤'&V´'&V¶&ÆR…ò’ÓâvÖTWfVçEG—S£¤'&V´'&V¶&ÆRÀÐ¢vÖTWfVçC£¤'&Vµ&÷…ò’ÓâvÖTWfVçEG—S£¤'&Vµ&÷ÀÐ¢vÖTWfVçC£¤VçF—G”¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¤VçF—G”¶–ÆÆVBÀÐ¢vÖTWfVçC£¤&öçW5WFFVB…ò’ÓâvÖTWfVçEG—S£¤&öçW5WFFVBÀÐ¢vÖTWfVçC£¤6†–WfVÖVçDWfVçB…ò’ÓâvÖTWfVçEG—S£¤6†–WfVÖVçDWfVçBÀÐ¢vÖTWfVçC£¤6†–WfVÖVçD–æ7&VÖVçB…ò’ÓâvÖTWfVçEG—S£¤6†–WfVÖVçD–æ7&VÖVçBÀÐ¢vÖTWfVçC£¥‡—6wVå–6·W…ò’ÓâvÖTWfVçEG—S£¥‡—6wVå–6·WÀÐ¢vÖTWfVçC£¤fÆ&T–væ—FTç2…ò’ÓâvÖTWfVçEG—S£¤fÆ&T–væ—FTç2ÀÐ¢vÖTWfVçC£¤†VÆ–6÷FW$w&VæFUVçDÖ—72…ò’ÓâvÖTWfVçEG—S£¤†VÆ–6÷FW$w&VæFUVçDÖ—72ÀÐ¢vÖTWfVçC£¥W6W$FFF÷væÆöFVB…ò’ÓâvÖTWfVçEG—S£¥W6W$FFF÷væÆöFVBÀÐ¢vÖTWfVçC£¥&vFöÆÄF—76öÇfVB…ò’ÓâvÖTWfVçEG—S£¥&vFöÆÄF—76öÇfVBÀÐ¢vÖTWfVçC£¤„ÅEd6†ævVDÖöFR…ò’ÓâvÖTWfVçEG—S£¤„ÅEd6†ævVDÖöFRÀÐ¢vÖTWfVçC£¤„ÅEd6†ævVEF&vWB…ò’ÓâvÖTWfVçEG—S£¤„ÅEd6†ævVEF&vWBÀÐ¢vÖTWfVçC£¥f÷FTVæFVB…ò’ÓâvÖTWfVçEG—S£¥f÷FTVæFVBÀÐ¢vÖTWfVçC£¥f÷FU7F'FVB…ò’ÓâvÖTWfVçEG—S£¥f÷FU7F'FVBÀÐ¢vÖTWfVçC£¥f÷FT6†ævVB…ò’ÓâvÖTWfVçEG—S£¥f÷FT6†ævVBÀÐ¢vÖTWfVçC£¥f÷FU76VB…ò’ÓâvÖTWfVçEG—S£¥f÷FU76VBÀÐ¢vÖTWfVçC£¥f÷FTf–ÆVB…ò’ÓâvÖTWfVçEG—S£¥f÷FTf–ÆVBÀÐ¢vÖTWfVçC£¥f÷FT67B…ò’ÓâvÖTWfVçEG—S£¥f÷FT67BÀÐ¢vÖTWfVçC£¥f÷FT÷F–öç2…ò’ÓâvÖTWfVçEG—S£¥f÷FT÷F–öç2ÀÐ¢vÖTWfVçC£¥&WÆ•6fVB…ò’ÓâvÖTWfVçEG—S£¥&WÆ•6fVBÀÐ¢vÖTWfVçC£¤VçFW&VEW&f÷&Öæ6TÖöFR…ò’ÓâvÖTWfVçEG—S£¤VçFW&VEW&f÷&Öæ6TÖöFRÀÐ¢vÖTWfVçC£¤'&÷w6U&WÆ—2…ò’ÓâvÖTWfVçEG—S£¤'&÷w6U&WÆ—2ÀÐ¢vÖTWfVçC£¥&WÆ•–÷WGV&U7FG2…ò’ÓâvÖTWfVçEG—S£¥&WÆ•–÷WGV&U7FG2ÀÐ¢vÖTWfVçC£¤–çfVçF÷'•WFFVB…ò’ÓâvÖTWfVçEG—S£¤–çfVçF÷'•WFFVBÀÐ¢vÖTWfVçC£¤6'EWFFVB…ò’ÓâvÖTWfVçEG—S£¤6'EWFFVBÀÐ¢vÖTWfVçC£¥7F÷&U&–6U6†VWEWFFVB…ò’ÓâvÖTWfVçEG—S£¥7F÷&U&–6U6†VWEWFFVBÀÐ¢vÖTWfVçC£¤V6öä–çfVçF÷'”6öææV7FVB…ò’ÓâvÖTWfVçEG—S£¤V6öä–çfVçF÷'”6öææV7FVBÀÐ¢vÖTWfVçC£¤—FVÕ66†VÖ–æ—F–Æ—¦VB…ò’ÓâvÖTWfVçEG—S£¤—FVÕ66†VÖ–æ—F–Æ—¦VBÀÐ¢vÖTWfVçC£¤v4æWu6W76–öâ…ò’ÓâvÖTWfVçEG—S£¤v4æWu6W76–öâÀÐ¢vÖTWfVçC£¤v4Æ÷7E6W76–öâ…ò’ÓâvÖTWfVçEG—S£¤v4Æ÷7E6W76–öâÀÐ¢vÖTWfVçC£¤–çG&ôf–æ—6‚…ò’ÓâvÖTWfVçEG—S£¤–çG&ôf–æ—6‚ÀÐ¢vÖTWfVçC£¤–çG&ôæW‡D6ÖW&…ò’ÓâvÖTWfVçEG—S£¤–çG&ôæW‡D6ÖW&ÀÐ¢vÖTWfVçC£¥Æ–W$6†ævT6Æ72…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6†ævT6Æ72ÀÐ¢vÖTWfVçC£¥FdÖF–ÖU&VÖ–æ–ær…ò’ÓâvÖTWfVçEG—S£¥FdÖF–ÖU&VÖ–æ–ærÀÐ¢vÖTWfVçC£¥FdvÖT÷fW"…ò’ÓâvÖTWfVçEG—S£¥FdvÖT÷fW"ÀÐ¢vÖTWfVçC£¤7FdfÆt6GW&VB…ò’ÓâvÖTWfVçEG—S£¤7FdfÆt6GW&VBÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çD–æ—F–Æ—¦VB…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çD–æ—F–Æ—¦VBÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEWFFT–ÖvW2…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT–ÖvW2ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEWFFTÆ–÷WB…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEWFFTÆ–÷WBÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEWFFT6–ær…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT6–ærÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEWFFT÷væW"…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEWFFT÷væW"ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çE7F'EF÷V6‚…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çE7F'EF÷V6‚ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çDVæEF÷V6‚…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çDVæEF÷V6‚ÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEVÇ6TVÆVÖVçB…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEVÇ6TVÆVÖVçBÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çDf¶T6GW&R…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çDf¶T6GW&RÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W"…ò’Óâ°Ð¢vÖTWfVçEG—S£¤6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W Ð¢ÐÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæE6VÆV7FVB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæE6VÆV7FVBÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæE7F'B…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæE7F'BÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæD7F—fR…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæD7F—fRÀÐ¢vÖTWfVçC£¥FVÕÆ•v—F–æt&Vv–ç2…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•v—F–æt&Vv–ç2ÀÐ¢vÖTWfVçC£¥FVÕÆ•v—F–ætVæG2…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•v—F–ætVæG2ÀÐ¢vÖTWfVçC£¥FVÕÆ•v—F–æt&÷WEFôVæB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•v—F–æt&÷WEFôVæBÀÐ¢vÖTWfVçC£¥FVÕÆ•&W7F'E&÷VæB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•&W7F'E&÷VæBÀÐ¢vÖTWfVçC£¥FVÕÆ•&VG•&W7F'B…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•&VG•&W7F'BÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæE&W7F'E6V6öæG2…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæE&W7F'E6V6öæG2ÀÐ¢vÖTWfVçC£¥FVÕÆ•FVÕ&VG’…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•FVÕ&VG’ÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæEv–â…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæEv–âÀÐ¢vÖTWfVçC£¥FVÕÆ•WFFUF–ÖW"…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•WFFUF–ÖW"ÀÐ¢vÖTWfVçC£¥FVÕÆ•&÷VæE7FÆVÖFR…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•&÷VæE7FÆVÖFRÀÐ¢vÖTWfVçC£¥FVÕÆ”÷fW'F–ÖT&Vv–â…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ”÷fW'F–ÖT&Vv–âÀÐ¢vÖTWfVçC£¥FVÕÆ”÷fW'F–ÖTVæB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ”÷fW'F–ÖTVæBÀÐ¢vÖTWfVçC£¥FVÕÆ•7VFFVäFVF„&Vv–â…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•7VFFVäFVF„&Vv–âÀÐ¢vÖTWfVçC£¥FVÕÆ•7VFFVäFVF„VæB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•7VFFVäFVF„VæBÀÐ¢vÖTWfVçC£¥FVÕÆ”vÖT÷fW"…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ”vÖT÷fW"ÀÐ¢vÖTWfVçC£¥FVÕÆ”ÖF–ÖU&VÖ–æ–ær…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ”ÖF–ÖU&VÖ–æ–ærÀÐ¢vÖTWfVçC£¥FVÕÆ•F–ÖW$fÆ6‚…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•F–ÖW$fÆ6‚ÀÐ¢vÖTWfVçC£¥FVÕÆ•F–ÖW%F–ÖTFFVB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•F–ÖW%F–ÖTFFVBÀÐ¢vÖTWfVçC£¥FVÕÆ•ö–çE7F'D6GW&R…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•ö–çE7F'D6GW&RÀÐ¢vÖTWfVçC£¥FVÕÆ•ö–çD6GW&VB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•ö–çD6GW&VBÀÐ¢vÖTWfVçC£¥FVÕÆ•ö–çDÆö6¶VB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•ö–çDÆö6¶VBÀÐ¢vÖTWfVçC£¥FVÕÆ•ö–çEVæÆö6¶VB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•ö–çEVæÆö6¶VBÀÐ¢vÖTWfVçC£¥FVÕÆ”6GW&T'&ö¶Vâ…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ”6GW&T'&ö¶VâÀÐ¢vÖTWfVçC£¥FVÕÆ”6GW&T&Æö6¶VB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ”6GW&T&Æö6¶VBÀÐ¢vÖTWfVçC£¥FVÕÆ”fÆtWfVçB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ”fÆtWfVçBÀÐ¢vÖTWfVçC£¥FVÕÆ•v–åæVÂ…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•v–åæVÂÀÐ¢vÖTWfVçC£¥FVÕÆ•FVÔ&Ææ6VEÆ–W"…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•FVÔ&Ææ6VEÆ–W"ÀÐ¢vÖTWfVçC£¥FVÕÆ•6WGWf–æ—6†VB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•6WGWf–æ—6†VBÀÐ¢vÖTWfVçC£¥FVÕÆ”ÆW'B…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ”ÆW'BÀÐ¢vÖTWfVçC£¥G&–æ–æt6ö×ÆWFR…ò’ÓâvÖTWfVçEG—S£¥G&–æ–æt6ö×ÆWFRÀÐ¢vÖTWfVçC£¥6†÷tg&VW¦UæVÂ…ò’ÓâvÖTWfVçEG—S£¥6†÷tg&VW¦UæVÂÀÐ¢vÖTWfVçC£¤†–FTg&VW¦UæVÂ…ò’ÓâvÖTWfVçEG—S£¤†–FTg&VW¦UæVÂÀÐ¢vÖTWfVçC£¤g&VW¦T6Õ7F'FVB…ò’ÓâvÖTWfVçEG—S£¤g&VW¦T6Õ7F'FVBÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$6†ævUFVÒ…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævUFVÒÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W%66÷&T6†ævVB…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W%66÷&T6†ævVBÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$6†ævT6Æ72…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævT6Æ72ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W%&W7vâ…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W%&W7vâÀÐ¢vÖTWfVçC£¤'V–ÆF–æt–æfô6†ævVB…ò’ÓâvÖTWfVçEG—S£¤'V–ÆF–æt–æfô6†ævVBÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$6†ævTF—6wV—6R…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$6†ævTF—6wV—6RÀÐ¢vÖTWfVçC£¥Æ–W$66÷VçD6†ævVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$66÷VçD6†ævVBÀÐ¢vÖTWfVçC£¥7•F&W6WB…ò’ÓâvÖTWfVçEG—S£¥7•F&W6WBÀÐ¢vÖTWfVçC£¤fÆu7FGW5WFFR…ò’ÓâvÖTWfVçEG—S£¤fÆu7FGW5WFFRÀÐ¢vÖTWfVçC£¥Æ–W%7FG5WFFVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W%7FG5WFFVBÀÐ¢vÖTWfVçC£¥Æ––æt6öÖÖVçF'’…ò’ÓâvÖTWfVçEG—S£¥Æ––æt6öÖÖVçF'’ÀÐ¢vÖTWfVçC£¥Æ–W$6†&vTFWÆ÷–VB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6†&vTFWÆ÷–VBÀÐ¢vÖTWfVçC£¥Æ–W$'V–ÇDö&¦V7B…ò’ÓâvÖTWfVçEG—S£¥Æ–W$'V–ÇDö&¦V7BÀÐ¢vÖTWfVçC£¥Æ–W%Ww&FVDö&¦V7B…ò’ÓâvÖTWfVçEG—S£¥Æ–W%Ww&FVDö&¦V7BÀÐ¢vÖTWfVçC£¥Æ–W$6''”ö&¦V7B…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6''”ö&¦V7BÀÐ¢vÖTWfVçC£¥Æ–W$G&÷ö&¦V7B…ò’ÓâvÖTWfVçEG—S£¥Æ–W$G&÷ö&¦V7BÀÐ¢vÖTWfVçC£¤ö&¦V7E&VÖ÷fVB…ò’ÓâvÖTWfVçEG—S£¤ö&¦V7E&VÖ÷fVBÀÐ¢vÖTWfVçC£¤ö&¦V7DFW7G&÷–VB…ò’ÓâvÖTWfVçEG—S£¤ö&¦V7DFW7G&÷–VBÀÐ¢vÖTWfVçC£¤ö&¦V7DFWFöæFVB…ò’ÓâvÖTWfVçEG—S£¤ö&¦V7DFWFöæFVBÀÐ¢vÖTWfVçC£¤6†–WfVÖVçDV&æVB…ò’ÓâvÖTWfVçEG—S£¤6†–WfVÖVçDV&æVBÀÐ¢vÖTWfVçC£¥7V5F&vWEWFFVB…ò’ÓâvÖTWfVçEG—S£¥7V5F&vWEWFFVBÀÐ¢vÖTWfVçC£¥F÷W&æÖVçE7FFUWFFR…ò’ÓâvÖTWfVçEG—S£¥F÷W&æÖVçE7FFUWFFRÀÐ¢vÖTWfVçC£¥F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâ…ò’ÓâvÖTWfVçEG—S£¥F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâÀÐ¢vÖTWfVçC£¥Æ–W$6ÆÆVDf÷$ÖVF–2…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6ÆÆVDf÷$ÖVF–2ÀÐ¢vÖTWfVçC£¥Æ–W$6¶VDf÷$&ÆÂ…ò’ÓâvÖTWfVçEG—S£¥Æ–W$6¶VDf÷$&ÆÂÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$&V6ÖTö'6W'fW"…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$&V6ÖTö'6W'fW"ÀÐ¢vÖTWfVçC£¥Æ–W$–væ—FVD–çb…ò’ÓâvÖTWfVçEG—S£¥Æ–W$–væ—FVD–çbÀÐ¢vÖTWfVçC£¥Æ–W$–væ—FVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$–væ—FVBÀÐ¢vÖTWfVçC£¥Æ–W$W‡F–æwV—6†VB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$W‡F–æwV—6†VBÀÐ¢vÖTWfVçC£¥Æ–W%FVÆW÷'FVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W%FVÆW÷'FVBÀÐ¢vÖTWfVçC£¥Æ–W$†VÆVDÖVF–46ÆÂ…ò’ÓâvÖTWfVçEG—S£¥Æ–W$†VÆVDÖVF–46ÆÂÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$6†&vU&VG’…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$6†&vU&VG’ÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W%v–æDF÷vâ…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W%v–æDF÷vâÀÐ¢vÖTWfVçC£¥Æ–W$–çgVÆæVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$–çgVÆæVBÀÐ¢vÖTWfVçC£¤W66÷'E7VVB…ò’ÓâvÖTWfVçEG—S£¤W66÷'E7VVBÀÐ¢vÖTWfVçC£¤W66÷'E&öw&W72…ò’ÓâvÖTWfVçEG—S£¤W66÷'E&öw&W72ÀÐ¢vÖTWfVçC£¤W66÷'E&V6VFR…ò’ÓâvÖTWfVçEG—S£¤W66÷'E&V6VFRÀÐ¢vÖTWfVçC£¤vÖUT”7F—fFVB…ò’ÓâvÖTWfVçEG—S£¤vÖUT”7F—fFVBÀÐ¢vÖTWfVçC£¤vÖUT”†–FFVâ…ò’ÓâvÖTWfVçEG—S£¤vÖUT”†–FFVâÀÐ¢vÖTWfVçC£¥Æ–W$W66÷'E66÷&R…ò’ÓâvÖTWfVçEG—S£¥Æ–W$W66÷'E66÷&RÀÐ¢vÖTWfVçC£¥Æ–W$†VÄöä†—B…ò’ÓâvÖTWfVçEG—S£¥Æ–W$†VÄöä†—BÀÐ¢vÖTWfVçC£¥Æ–W%7FVÅ6æGf–6‚…ò’ÓâvÖTWfVçEG—S£¥Æ–W%7FVÅ6æGf–6‚ÀÐ¢vÖTWfVçC£¥6†÷t6Æ74Æ–÷WB…ò’ÓâvÖTWfVçEG—S£¥6†÷t6Æ74Æ–÷WBÀÐ¢vÖTWfVçC£¥6†÷ug5æVÂ…ò’ÓâvÖTWfVçEG—S£¥6†÷ug5æVÂÀÐ¢vÖTWfVçC£¥Æ–W$FÖvVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$FÖvVBÀÐ¢vÖTWfVçC£¤&VæÆ–W$æ÷F–f–6F–öâ…ò’ÓâvÖTWfVçEG—S£¤&VæÆ–W$æ÷F–f–6F–öâÀÐ¢vÖTWfVçC£¤&VæÖF6„Ö…7G&V²…ò’ÓâvÖTWfVçEG—S£¤&VæÖF6„Ö…7G&V²ÀÐ¢vÖTWfVçC£¤&Væ&÷VæE7F'B…ò’ÓâvÖTWfVçEG—S£¤&Væ&÷VæE7F'BÀÐ¢vÖTWfVçC£¤&Væv–åæVÂ…ò’ÓâvÖTWfVçEG—S£¤&Væv–åæVÂÀÐ¢vÖTWfVçC£¥fUv–åæVÂ…ò’ÓâvÖTWfVçEG—S£¥fUv–åæVÂÀÐ¢vÖTWfVçC£¤—$F6‚…ò’ÓâvÖTWfVçEG—S£¤—$F6‚ÀÐ¢vÖTWfVçC£¤ÆæFVB…ò’ÓâvÖTWfVçEG—S£¤ÆæFVBÀÐ¢vÖTWfVçC£¥Æ–W$FÖvTFöFvVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$FÖvTFöFvVBÀÐ¢vÖTWfVçC£¥Æ–W%7GVææVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W%7GVææVBÀÐ¢vÖTWfVçC£¥66÷WDw&æE6ÆÒ…ò’ÓâvÖTWfVçEG—S£¥66÷WDw&æE6ÆÒÀÐ¢vÖTWfVçC£¥66÷WE6ÆÖFöÆÄÆæFVB…ò’ÓâvÖTWfVçEG—S£¥66÷WE6ÆÖFöÆÄÆæFVBÀÐ¢vÖTWfVçC£¤'&÷t–×7B…ò’ÓâvÖTWfVçEG—S£¤'&÷t–×7BÀÐ¢vÖTWfVçC£¥Æ–W$¦&FVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$¦&FVBÀÐ¢vÖTWfVçC£¥Æ–W$¦&FVDfFR…ò’ÓâvÖTWfVçEG—S£¥Æ–W$¦&FVDfFRÀÐ¢vÖTWfVçC£¥Æ–W%6†–VÆD&Æö6¶VB…ò’ÓâvÖTWfVçEG—S£¥Æ–W%6†–VÆD&Æö6¶VBÀÐ¢vÖTWfVçC£¥Æ–W%–ææVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W%–ææVBÀÐ¢vÖTWfVçC£¥Æ–W$†VÆVD'”ÖVF–2…ò’ÓâvÖTWfVçEG—S£¥Æ–W$†VÆVD'”ÖVF–2ÀÐ¢vÖTWfVçC£¥Æ–W%6VDö&¦V7B…ò’ÓâvÖTWfVçEG—S£¥Æ–W%6VDö&¦V7BÀÐ¢vÖTWfVçC£¤—FVÔf÷VæB…ò’ÓâvÖTWfVçEG—S£¤—FVÔf÷VæBÀÐ¢vÖTWfVçC£¥6†÷tææ÷FF–öâ…ò’ÓâvÖTWfVçEG—S£¥6†÷tææ÷FF–öâÀÐ¢vÖTWfVçC£¤†–FTææ÷FF–öâ…ò’ÓâvÖTWfVçEG—S£¤†–FTææ÷FF–öâÀÐ¢vÖTWfVçC£¥÷7D–çfVçF÷'”Æ–6F–öâ…ò’ÓâvÖTWfVçEG—S£¥÷7D–çfVçF÷'”Æ–6F–öâÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEVæÆö6µWFFVB…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEVæÆö6µWFFVBÀÐ¢vÖTWfVçC£¤FWÆ÷”'Vfd&ææW"…ò’ÓâvÖTWfVçEG—S£¤FWÆ÷”'Vfd&ææW"ÀÐ¢vÖTWfVçC£¥Æ–W$'Vfb…ò’ÓâvÖTWfVçEG—S£¥Æ–W$'VfbÀÐ¢vÖTWfVçC£¤ÖVF–4FVF‚…ò’ÓâvÖTWfVçEG—S£¤ÖVF–4FVF‚ÀÐ¢vÖTWfVçC£¤÷fW'F–ÖTær…ò’ÓâvÖTWfVçEG—S£¤÷fW'F–ÖTærÀÐ¢vÖTWfVçC£¥FV×46†ævVB…ò’ÓâvÖTWfVçEG—S£¥FV×46†ævVBÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVåV×¶–äw&"…ò’ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVåV×¶–äw&"ÀÐ¢vÖTWfVçC£¥&ö6¶WD§V×…ò’ÓâvÖTWfVçEG—S£¥&ö6¶WD§V×ÀÐ¢vÖTWfVçC£¥&ö6¶WD§V×ÆæFVB…ò’ÓâvÖTWfVçEG—S£¥&ö6¶WD§V×ÆæFVBÀÐ¢vÖTWfVçC£¥7F–6·”§V×…ò’ÓâvÖTWfVçEG—S£¥7F–6·”§V×ÀÐ¢vÖTWfVçC£¥7F–6·”§V×ÆæFVB…ò’ÓâvÖTWfVçEG—S£¥7F–6·”§V×ÆæFVBÀÐ¢vÖTWfVçC£¥&ö6¶WE6´ÆVæ6‚…ò’ÓâvÖTWfVçEG—S£¥&ö6¶WE6´ÆVæ6‚ÀÐ¢vÖTWfVçC£¥&ö6¶WE6´ÆæFVB…ò’ÓâvÖTWfVçEG—S£¥&ö6¶WE6´ÆæFVBÀÐ¢vÖTWfVçC£¤ÖVF–4FVfVæFVB…ò’ÓâvÖTWfVçEG—S£¤ÖVF–4FVfVæFVBÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W$†VÆVB…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W$†VÆVBÀÐ¢vÖTWfVçC£¥Æ–W$FW7G&÷–VE—T&öÖ"…ò’ÓâvÖTWfVçEG—S£¥Æ–W$FW7G&÷–VE—T&öÖ"ÀÐ¢vÖTWfVçC£¤ö&¦V7DFVfÆV7FVB…ò’ÓâvÖTWfVçEG—S£¤ö&¦V7DFVfÆV7FVBÀÐ¢vÖTWfVçC£¥Æ–W$×g…ò’ÓâvÖTWfVçEG—S£¥Æ–W$×gÀÐ¢vÖTWfVçC£¥&–E7väÖö"…ò’ÓâvÖTWfVçEG—S£¥&–E7väÖö"ÀÐ¢vÖTWfVçC£¥&–E7vå7VB…ò’ÓâvÖTWfVçEG—S£¥&–E7vå7VBÀÐ¢vÖTWfVçC£¤æd&Æö6¶VB…ò’ÓâvÖTWfVçEG—S£¤æd&Æö6¶VBÀÐ¢vÖTWfVçC£¥F…G&6µ76VB…ò’ÓâvÖTWfVçEG—S£¥F…G&6µ76VBÀÐ¢vÖTWfVçC£¤çVÔ6W'46†ævVB…ò’ÓâvÖTWfVçEG—S£¤çVÔ6W'46†ævVBÀÐ¢vÖTWfVçC£¥Æ–W%&VvVæW&FR…ò’ÓâvÖTWfVçEG—S£¥Æ–W%&VvVæW&FRÀÐ¢vÖTWfVçC£¥WFFU7FGW4—FVÒ…ò’ÓâvÖTWfVçEG—S£¥WFFU7FGW4—FVÒÀÐ¢vÖTWfVçC£¥7FG5&W6WE&÷VæB…ò’ÓâvÖTWfVçEG—S£¥7FG5&W6WE&÷VæBÀÐ¢vÖTWfVçC£¥66÷&U7FG467V×VÆFVEWFFR…ò’ÓâvÖTWfVçEG—S£¥66÷&U7FG467V×VÆFVEWFFRÀÐ¢vÖTWfVçC£¥66÷&U7FG467V×VÆFVE&W6WB…ò’ÓâvÖTWfVçEG—S£¥66÷&U7FG467V×VÆFVE&W6WBÀÐ¢vÖTWfVçC£¤6†–WfVÖVçDV&æVDÆö6Â…ò’ÓâvÖTWfVçEG—S£¤6†–WfVÖVçDV&æVDÆö6ÂÀÐ¢vÖTWfVçC£¥Æ–W$†VÆVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$†VÆVBÀÐ¢vÖTWfVçC£¤'V–ÆF–æt†VÆVB…ò’ÓâvÖTWfVçEG—S£¤'V–ÆF–æt†VÆVBÀÐ¢vÖTWfVçC£¤—FVÕ–6·W…ò’ÓâvÖTWfVçEG—S£¤—FVÕ–6·WÀÐ¢vÖTWfVçC£¤GVVÅ7FGW2…ò’ÓâvÖTWfVçEG—S£¤GVVÅ7FGW2ÀÐ¢vÖTWfVçC£¤f—6„æ÷F–6R…ò’ÓâvÖTWfVçEG—S£¤f—6„æ÷F–6RÀÐ¢vÖTWfVçC£¤f—6„æ÷F–6T&Ò…ò’ÓâvÖTWfVçEG—S£¤f—6„æ÷F–6T&ÒÀÐ¢vÖTWfVçC£¥6Ææ÷F–6R…ò’ÓâvÖTWfVçEG—S£¥6Ææ÷F–6RÀÐ¢vÖTWfVçC£¥F‡&÷v&ÆT†—B…ò’ÓâvÖTWfVçEG—S£¥F‡&÷v&ÆT†—BÀÐ¢vÖTWfVçC£¥V×¶–äÆ÷&E7VÖÖöæVB…ò’ÓâvÖTWfVçEG—S£¥V×¶–äÆ÷&E7VÖÖöæVBÀÐ¢vÖTWfVçC£¥V×¶–äÆ÷&D¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¥V×¶–äÆ÷&D¶–ÆÆVBÀÐ¢vÖTWfVçC£¤ÖW&6×W57VÖÖöæVB…ò’ÓâvÖTWfVçEG—S£¤ÖW&6×W57VÖÖöæVBÀÐ¢vÖTWfVçC£¤ÖW&6×W4¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¤ÖW&6×W4¶–ÆÆVBÀÐ¢vÖTWfVçC£¤ÖW&6×W4W66Uv&æ–ær…ò’ÓâvÖTWfVçEG—S£¤ÖW&6×W4W66Uv&æ–ærÀÐ¢vÖTWfVçC£¤ÖW&6×W4W66VB…ò’ÓâvÖTWfVçEG—S£¤ÖW&6×W4W66VBÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷757VÖÖöæVB…ò’ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷757VÖÖöæVBÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷757GVææVB…ò’ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷757GVææVBÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷74¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷74¶–ÆÆVBÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷74¶–ÆÆW"…ò’ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷74¶–ÆÆW"ÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷74W66T–ÖÖ–æVçB…ò’ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷74W66T–ÖÖ–æVçBÀÐ¢vÖTWfVçC£¤W–V&ÆÄ&÷74W66VB…ò’ÓâvÖTWfVçEG—S£¤W–V&ÆÄ&÷74W66VBÀÐ¢vÖTWfVçC£¤ç4‡W'B…ò’ÓâvÖTWfVçEG—S£¤ç4‡W'BÀÐ¢vÖTWfVçC£¤6öçG&öÅö–çEF–ÖW%WFFVB…ò’ÓâvÖTWfVçEG—S£¤6öçG&öÅö–çEF–ÖW%WFFVBÀÐ¢vÖTWfVçC£¥Æ–W$†–v„f—fU7F'B…ò’ÓâvÖTWfVçEG—S£¥Æ–W$†–v„f—fU7F'BÀÐ¢vÖTWfVçC£¥Æ–W$†–v„f—fT6æ6VÂ…ò’ÓâvÖTWfVçEG—S£¥Æ–W$†–v„f—fT6æ6VÂÀÐ¢vÖTWfVçC£¥Æ–W$†–v„f—fU7V66W72…ò’ÓâvÖTWfVçEG—S£¥Æ–W$†–v„f—fU7V66W72ÀÐ¢vÖTWfVçC£¥Æ–W$&öçW5ö–çG2…ò’ÓâvÖTWfVçEG—S£¥Æ–W$&öçW5ö–çG2ÀÐ¢vÖTWfVçC£¥Æ–W%Ww&FVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W%Ww&FVBÀÐ¢vÖTWfVçC£¥Æ–W$'W–&6²…ò’ÓâvÖTWfVçEG—S£¥Æ–W$'W–&6²ÀÐ¢vÖTWfVçC£¥Æ–W%W6VE÷vW%W&÷GFÆR…ò’ÓâvÖTWfVçEG—S£¥Æ–W%W6VE÷vW%W&÷GFÆRÀÐ¢vÖTWfVçC£¤6‡&—7FÖ4v–gDw&"…ò’ÓâvÖTWfVçEG—S£¤6‡&—7FÖ4v–gDw&"ÀÐ¢vÖTWfVçC£¥Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæR…ò’ÓâvÖTWfVçEG—S£¥Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæRÀÐ¢vÖTWfVçC£¥'G•WFFVB…ò’ÓâvÖTWfVçEG—S£¥'G•WFFVBÀÐ¢vÖTWfVçC£¥'G•&Vd6†ævVB…ò’ÓâvÖTWfVçEG—S£¥'G•&Vd6†ævVBÀÐ¢vÖTWfVçC£¥'G”7&—FW&–6†ævVB…ò’ÓâvÖTWfVçEG—S£¥'G”7&—FW&–6†ævVBÀÐ¢vÖTWfVçC£¥'G”–çf—FW46†ævVB…ò’ÓâvÖTWfVçEG—S£¥'G”–çf—FW46†ævVBÀÐ¢vÖTWfVçC£¥'G•VWVU7FFT6†ævVB…ò’ÓâvÖTWfVçEG—S£¥'G•VWVU7FFT6†ævVBÀÐ¢vÖTWfVçC£¥'G”6†B…ò’ÓâvÖTWfVçEG—S£¥'G”6†BÀÐ¢vÖTWfVçC£¥'G”ÖVÖ&W$¦ö–â…ò’ÓâvÖTWfVçEG—S£¥'G”ÖVÖ&W$¦ö–âÀÐ¢vÖTWfVçC£¥'G”ÖVÖ&W$ÆVfR…ò’ÓâvÖTWfVçEG—S£¥'G”ÖVÖ&W$ÆVfRÀÐ¢vÖTWfVçC£¤ÖF6„–çf—FW5WFFVB…ò’ÓâvÖTWfVçEG—S£¤ÖF6„–çf—FW5WFFVBÀÐ¢vÖTWfVçC£¤Æö&'•WFFVB…ò’ÓâvÖTWfVçEG—S£¤Æö&'•WFFVBÀÐ¢vÖTWfVçC£¤×fÔÖ—76–öåWFFR…ò’ÓâvÖTWfVçEG—S£¤×fÔÖ—76–öåWFFRÀÐ¢vÖTWfVçC£¥&V6Æ7VÆFT†öÆ–F—2…ò’ÓâvÖTWfVçEG—S£¥&V6Æ7VÆFT†öÆ–F—2ÀÐ¢vÖTWfVçC£¥Æ–W$7W'&Væ7”6†ævVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W$7W'&Væ7”6†ævVBÀÐ¢vÖTWfVçC£¤Föö×6F•&ö6¶WD÷Vâ…ò’ÓâvÖTWfVçEG—S£¤Föö×6F•&ö6¶WD÷VâÀÐ¢vÖTWfVçC£¥&VÖ÷fTæVÖW6—5&VÆF–öç6†—2…ò’ÓâvÖTWfVçEG—S£¥&VÖ÷fTæVÖW6—5&VÆF–öç6†—2ÀÐ¢vÖTWfVçC£¤×fÔ7&VF—D&öçW5vfR…ò’ÓâvÖTWfVçEG—S£¤×fÔ7&VF—D&öçW5vfRÀÐ¢vÖTWfVçC£¤×fÔ7&VF—D&öçW4ÆÂ…ò’ÓâvÖTWfVçEG—S£¤×fÔ7&VF—D&öçW4ÆÂÀÐ¢vÖTWfVçC£¤×fÔ7&VF—D&öçW4ÆÄGfæ6VB…ò’ÓâvÖTWfVçEG—S£¤×fÔ7&VF—D&öçW4ÆÄGfæ6VBÀÐ¢vÖTWfVçC£¤×fÕV–6µ6VçG'•Ww&FR…ò’ÓâvÖTWfVçEG—S£¤×fÕV–6µ6VçG'•Ww&FRÀÐ¢vÖTWfVçC£¤×fÕFæ´FW7G&÷–VD'•Æ–W'2…ò’ÓâvÖTWfVçEG—S£¤×fÕFæ´FW7G&÷–VD'•Æ–W'2ÀÐ¢vÖTWfVçC£¤×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ"…ò’ÓâvÖTWfVçEG—S£¤×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ"ÀÐ¢vÖTWfVçC£¤×fÕ–6·W7W'&Væ7’…ò’ÓâvÖTWfVçEG—S£¤×fÕ–6·W7W'&Væ7’ÀÐ¢vÖTWfVçC£¤×fÔ&öÖ$6'&–W$¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¤×fÔ&öÖ$6'&–W$¶–ÆÆVBÀÐ¢vÖTWfVçC£¤×fÕ6VçG'”'W7FW$FWFöæFR…ò’ÓâvÖTWfVçEG—S£¤×fÕ6VçG'”'W7FW$FWFöæFRÀÐ¢vÖTWfVçC£¤×fÕ66÷WDÖ&¶VDf÷$FVF‚…ò’ÓâvÖTWfVçEG—S£¤×fÕ66÷WDÖ&¶VDf÷$FVF‚ÀÐ¢vÖTWfVçC£¤×fÔÖVF–5÷vW%W6†&VB…ò’ÓâvÖTWfVçEG—S£¤×fÔÖVF–5÷vW%W6†&VBÀÐ¢vÖTWfVçC£¤×fÔ&Vv–åvfR…ò’ÓâvÖTWfVçEG—S£¤×fÔ&Vv–åvfRÀÐ¢vÖTWfVçC£¤×fÕvfT6ö×ÆWFR…ò’ÓâvÖTWfVçEG—S£¤×fÕvfT6ö×ÆWFRÀÐ¢vÖTWfVçC£¤×fÔÖ—76–öä6ö×ÆWFR…ò’ÓâvÖTWfVçEG—S£¤×fÔÖ—76–öä6ö×ÆWFRÀÐ¢vÖTWfVçC£¤×fÔ&öÖ%&W6WD'•Æ–W"…ò’ÓâvÖTWfVçEG—S£¤×fÔ&öÖ%&W6WD'•Æ–W"ÀÐ¢vÖTWfVçC£¤×fÔ&öÖ$Æ&ÕG&–vvW&VB…ò’ÓâvÖTWfVçEG—S£¤×fÔ&öÖ$Æ&ÕG&–vvW&VBÀÐ¢vÖTWfVçC£¤×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W"…ò’ÓâvÖTWfVçEG—S£¤×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W"ÀÐ¢vÖTWfVçC£¤×fÕvfTf–ÆVB…ò’ÓâvÖTWfVçEG—S£¤×fÕvfTf–ÆVBÀÐ¢vÖTWfVçC£¤×fÕ&W6WE7FG2…ò’ÓâvÖTWfVçEG—S£¤×fÕ&W6WE7FG2ÀÐ¢vÖTWfVçC£¤FÖvU&W6—7FVB…ò’ÓâvÖTWfVçEG—S£¤FÖvU&W6—7FVBÀÐ¢vÖTWfVçC£¥&Wf—fUÆ–W$æ÷F–g’…ò’ÓâvÖTWfVçEG—S£¥&Wf—fUÆ–W$æ÷F–g’ÀÐ¢vÖTWfVçC£¥&Wf—fUÆ–W%7F÷VB…ò’ÓâvÖTWfVçEG—S£¥&Wf—fUÆ–W%7F÷VBÀÐ¢vÖTWfVçC£¥&Wf—fUÆ–W$6ö×ÆWFR…ò’ÓâvÖTWfVçEG—S£¥&Wf—fUÆ–W$6ö×ÆWFRÀÐ¢vÖTWfVçC£¥Æ–W%GW&æVEFôv†÷7B…ò’ÓâvÖTWfVçEG—S£¥Æ–W%GW&æVEFôv†÷7BÀÐ¢vÖTWfVçC£¤ÖVF–wVå6†–VÆD&Æö6¶VDFÖvR…ò’ÓâvÖTWfVçEG—S£¤ÖVF–wVå6†–VÆD&Æö6¶VDFÖvRÀÐ¢vÖTWfVçC£¤×fÔGevfT6ö×ÆWFTæôvFW2…ò’ÓâvÖTWfVçEG—S£¤×fÔGevfT6ö×ÆWFTæôvFW2ÀÐ¢vÖTWfVçC£¤×fÕ6æ—W$†VG6†÷D7W'&Væ7’…ò’ÓâvÖTWfVçEG—S£¤×fÕ6æ—W$†VG6†÷D7W'&Væ7’ÀÐ¢vÖTWfVçC£¤×fÔÖææ†GFå—B…ò’ÓâvÖTWfVçEG—S£¤×fÔÖææ†GFå—BÀÐ¢vÖTWfVçC£¤fÆt6'&–VD–äFWFV7F–öå¦öæR…ò’ÓâvÖTWfVçEG—S£¤fÆt6'&–VD–äFWFV7F–öå¦öæRÀÐ¢vÖTWfVçC£¤×fÔGevfT¶–ÆÆVE7GVå&F–ò…ò’ÓâvÖTWfVçEG—S£¤×fÔGevfT¶–ÆÆVE7GVå&F–òÀÐ¢vÖTWfVçC£¥Æ–W$F—&V7D†—E7GVâ…ò’ÓâvÖTWfVçEG—S£¥Æ–W$F—&V7D†—E7GVâÀÐ¢vÖTWfVçC£¤×fÕ6VçG'”'W7FW$¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¤×fÕ6VçG'”'W7FW$¶–ÆÆVBÀÐ¢vÖTWfVçC£¥Ww&FW4f–ÆT6†ævVB…ò’ÓâvÖTWfVçEG—S£¥Ww&FW4f–ÆT6†ævVBÀÐ¢vÖTWfVçC£¥&EFVÕö–çG46†ævVB…ò’ÓâvÖTWfVçEG—S£¥&EFVÕö–çG46†ævVBÀÐ¢vÖTWfVçC£¥&E'VÆW57FFT6†ævVB…ò’ÓâvÖTWfVçEG—S£¥&E'VÆW57FFT6†ævVBÀÐ¢vÖTWfVçC£¥&E&ö&÷D¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¥&E&ö&÷D¶–ÆÆVBÀÐ¢vÖTWfVçC£¥&E&ö&÷D–×7B…ò’ÓâvÖTWfVçEG—S£¥&E&ö&÷D–×7BÀÐ¢vÖTWfVçC£¥FVÕÆ•&U&÷VæEF–ÖTÆVgB…ò’ÓâvÖTWfVçEG—S£¥FVÕÆ•&U&÷VæEF–ÖTÆVgBÀÐ¢vÖTWfVçC£¥&6‡WFTFWÆ÷’…ò’ÓâvÖTWfVçEG—S£¥&6‡WFTFWÆ÷’ÀÐ¢vÖTWfVçC£¥&6‡WFT†öÇ7FW"…ò’ÓâvÖTWfVçEG—S£¥&6‡WFT†öÇ7FW"ÀÐ¢vÖTWfVçC£¤¶–ÆÅ&Vf–ÆÇ4ÖWFW"…ò’ÓâvÖTWfVçEG—S£¤¶–ÆÅ&Vf–ÆÇ4ÖWFW"ÀÐ¢vÖTWfVçC£¥'5FVçDWfVçB…ò’ÓâvÖTWfVçEG—S£¥'5FVçDWfVçBÀÐ¢vÖTWfVçC£¤6öæv¶–ÆÂ…ò’ÓâvÖTWfVçEG—S£¤6öæv¶–ÆÂÀÐ¢vÖTWfVçC£¥Æ–W$–æ—F–Å7vâ…ò’ÓâvÖTWfVçEG—S£¥Æ–W$–æ—F–Å7vâÀÐ¢vÖTWfVçC£¤6ö×WF—F—fUf–7F÷'’…ò’ÓâvÖTWfVçEG—S£¤6ö×WF—F—fUf–7F÷'’ÀÐ¢vÖTWfVçC£¤6ö×WF—F—fU7FG5WFFR…ò’ÓâvÖTWfVçEG—S£¤6ö×WF—F—fU7FG5WFFRÀÐ¢vÖTWfVçC£¤Ö–æ”vÖUv–â…ò’ÓâvÖTWfVçEG—S£¤Ö–æ”vÖUv–âÀÐ¢vÖTWfVçC£¥6VçG'”öävô7F—fR…ò’ÓâvÖTWfVçEG—S£¥6VçG'”öävô7F—fRÀÐ¢vÖTWfVçC£¤GV6µ‡ÆWfVÅW…ò’ÓâvÖTWfVçEG—S£¤GV6µ‡ÆWfVÅWÀÐ¢vÖTWfVçC£¥VW7DÆöt÷VæVB…ò’ÓâvÖTWfVçEG—S£¥VW7DÆöt÷VæVBÀÐ¢vÖTWfVçC£¥66†VÖWFFVB…ò’ÓâvÖTWfVçEG—S£¥66†VÖWFFVBÀÐ¢vÖTWfVçC£¤Æö6ÅÆ–W%–6·WvVöâ…ò’ÓâvÖTWfVçEG—S£¤Æö6ÅÆ–W%–6·WvVöâÀÐ¢vÖTWfVçC£¥&EÆ–W%66÷&Uö–çG2…ò’ÓâvÖTWfVçEG—S£¥&EÆ–W%66÷&Uö–çG2ÀÐ¢vÖTWfVçC£¤FVÖöÖäFWE7F–6¶–W2…ò’ÓâvÖTWfVçEG—S£¤FVÖöÖäFWE7F–6¶–W2ÀÐ¢vÖTWfVçC£¥VW7Dö&¦V7F—fT6ö×ÆWFVB…ò’ÓâvÖTWfVçEG—S£¥VW7Dö&¦V7F—fT6ö×ÆWFVBÀÐ¢vÖTWfVçC£¥Æ–W%66÷&T6†ævVB…ò’ÓâvÖTWfVçEG—S£¥Æ–W%66÷&T6†ævVBÀÐ¢vÖTWfVçC£¤¶–ÆÆVD6–æuÆ–W"…ò’ÓâvÖTWfVçEG—S£¤¶–ÆÆVD6–æuÆ–W"ÀÐ¢vÖTWfVçC£¤Vçf—&öæÖVçFÄFVF‚…ò’ÓâvÖTWfVçEG—S£¤Vçf—&öæÖVçFÄFVF‚ÀÐ¢vÖTWfVçC£¥&ö¦V7F–ÆTF—&V7D†—B…ò’ÓâvÖTWfVçEG—S£¥&ö¦V7F–ÆTF—&V7D†—BÀÐ¢vÖTWfVçC£¥74vWB…ò’ÓâvÖTWfVçEG—S£¥74vWBÀÐ¢vÖTWfVçC£¥7566÷&R…ò’ÓâvÖTWfVçEG—S£¥7566÷&RÀÐ¢vÖTWfVçC£¥74g&VR…ò’ÓâvÖTWfVçEG—S£¥74g&VRÀÐ¢vÖTWfVçC£¥75746Vv‡B…ò’ÓâvÖTWfVçEG—S£¥75746Vv‡BÀÐ¢vÖTWfVçC£¥74&ÆÅ7FöÆVâ…ò’ÓâvÖTWfVçEG—S£¥74&ÆÅ7FöÆVâÀÐ¢vÖTWfVçC£¥74&ÆÄ&Æö6¶VB…ò’ÓâvÖTWfVçEG—S£¥74&ÆÄ&Æö6¶VBÀÐ¢vÖTWfVçC£¤FÖvU&WfVçFVB…ò’ÓâvÖTWfVçEG—S£¤FÖvU&WfVçFVBÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVä&÷74¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVä&÷74¶–ÆÆVBÀÐ¢vÖTWfVçC£¤W66VDÆö÷D—6ÆæB…ò’ÓâvÖTWfVçEG—S£¤W66VDÆö÷D—6ÆæBÀÐ¢vÖTWfVçC£¥FvvVEÆ–W$4—B…ò’ÓâvÖTWfVçEG—S£¥FvvVEÆ–W$4—BÀÐ¢vÖTWfVçC£¤ÖW&6×W57GVææVB…ò’ÓâvÖTWfVçEG—S£¤ÖW&6×W57GVææVBÀÐ¢vÖTWfVçC£¤ÖW&6×W5&÷f÷VæB…ò’ÓâvÖTWfVçEG—S£¤ÖW&6×W5&÷f÷VæBÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVBÀÐ¢vÖTWfVçC£¥6¶VÆWFöä¶–ÆÆVEVW7B…ò’ÓâvÖTWfVçEG—S£¥6¶VÆWFöä¶–ÆÆVEVW7BÀÐ¢vÖTWfVçC£¥6¶VÆWFöä¶–æt¶–ÆÆVEVW7B…ò’ÓâvÖTWfVçEG—S£¥6¶VÆWFöä¶–æt¶–ÆÆVEVW7BÀÐ¢vÖTWfVçC£¤W66T†VÆÂ…ò’ÓâvÖTWfVçEG—S£¤W66T†VÆÂÀÐ¢vÖTWfVçC£¤7&÷757V7G&Ä'&–FvR…ò’ÓâvÖTWfVçEG—S£¤7&÷757V7G&Ä'&–FvRÀÐ¢vÖTWfVçC£¤Ö–æ”vÖUvöâ…ò’ÓâvÖTWfVçEG—S£¤Ö–æ”vÖUvöâÀÐ¢vÖTWfVçC£¥&W7väv†÷7B…ò’ÓâvÖTWfVçEG—S£¥&W7väv†÷7BÀÐ¢vÖTWfVçC£¤¶–ÆÄ–ä†VÆÂ…ò’ÓâvÖTWfVçEG—S£¤¶–ÆÄ–ä†VÆÂÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVäGV6´6öÆÆV7FVB…ò’ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVäGV6´6öÆÆV7FVBÀÐ¢vÖTWfVçC£¥7V6–Å66÷&R…ò’ÓâvÖTWfVçEG—S£¥7V6–Å66÷&RÀÐ¢vÖTWfVçC£¥FVÔÆVFW$¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¥FVÔÆVFW$¶–ÆÆVBÀÐ¢vÖTWfVçC£¤†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVB…ò’ÓâvÖTWfVçEG—S£¤†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVBÀÐ¢vÖTWfVçC£¥&V6Æ7VÆFUG'V6R…ò’ÓâvÖTWfVçEG—S£¥&V6Æ7VÆFUG'V6RÀÐ¢vÖTWfVçC£¤FVE&–ævW$6†VDFVF‚…ò’ÓâvÖTWfVçEG—S£¤FVE&–ævW$6†VDFVF‚ÀÐ¢vÖTWfVçC£¤7&÷76&÷t†VÂ…ò’ÓâvÖTWfVçEG—S£¤7&÷76&÷t†VÂÀÐ¢vÖTWfVçC£¤FÖvTÖ—F–vFVB…ò’ÓâvÖTWfVçEG—S£¤FÖvTÖ—F–vFVBÀÐ¢vÖTWfVçC£¥–ÆöEW6†VB…ò’ÓâvÖTWfVçEG—S£¥–ÆöEW6†VBÀÐ¢vÖTWfVçC£¥Æ–W$&æFöæVDÖF6‚…ò’ÓâvÖTWfVçEG—S£¥Æ–W$&æFöæVDÖF6‚ÀÐ¢vÖTWfVçC£¤6ÄG&vÆ–æR…ò’ÓâvÖTWfVçEG—S£¤6ÄG&vÆ–æRÀÐ¢vÖTWfVçC£¥&W7F'EF–ÖW%F–ÖR…ò’ÓâvÖTWfVçEG—S£¥&W7F'EF–ÖW%F–ÖRÀÐ¢vÖTWfVçC£¥v–äÆ–Ö—D6†ævVB…ò’ÓâvÖTWfVçEG—S£¥v–äÆ–Ö—D6†ævVBÀÐ¢vÖTWfVçC£¥v–åæVÅ6†÷u66÷&W2…ò’ÓâvÖTWfVçEG—S£¥v–åæVÅ6†÷u66÷&W2ÀÐ¢vÖTWfVçC£¥F÷7G&V×5&WVW7Df–æ—6†VB…ò’ÓâvÖTWfVçEG—S£¥F÷7G&V×5&WVW7Df–æ—6†VBÀÐ¢vÖTWfVçC£¤6ö×WF—F—fU7FFT6†ævVB…ò’ÓâvÖTWfVçEG—S£¤6ö×WF—F—fU7FFT6†ævVBÀÐ¢vÖTWfVçC£¤vÆö&Åv$FFWFFVB…ò’ÓâvÖTWfVçEG—S£¤vÆö&Åv$FFWFFVBÀÐ¢vÖTWfVçC£¥7F÷vF6„6†ævVB…ò’ÓâvÖTWfVçEG—S£¥7F÷vF6„6†ævVBÀÐ¢vÖTWfVçC£¤G57F÷…ò’ÓâvÖTWfVçEG—S£¤G57F÷ÀÐ¢vÖTWfVçC£¤G567&VVç6†÷B…ò’ÓâvÖTWfVçEG—S£¤G567&VVç6†÷BÀÐ¢vÖTWfVçC£¥6†÷tÖF6…7VÖÖ'’…ò’ÓâvÖTWfVçEG—S£¥6†÷tÖF6…7VÖÖ'’ÀÐ¢vÖTWfVçC£¤W‡W&–Væ6T6†ævVB…ò’ÓâvÖTWfVçEG—S£¤W‡W&–Væ6T6†ævVBÀÐ¢vÖTWfVçC£¤&Vv–å‡ÆW'…ò’ÓâvÖTWfVçEG—S£¤&Vv–å‡ÆW'ÀÐ¢vÖTWfVçC£¤ÖF6†Ö¶W%7FG5WFFVB…ò’ÓâvÖTWfVçEG—S£¤ÖF6†Ö¶W%7FG5WFFVBÀÐ¢vÖTWfVçC£¥&VÖF6…f÷FUW&–öD÷fW"…ò’ÓâvÖTWfVçEG—S£¥&VÖF6…f÷FUW&–öD÷fW"ÀÐ¢vÖTWfVçC£¥&VÖF6„f–ÆVEFô7&VFR…ò’ÓâvÖTWfVçEG—S£¥&VÖF6„f–ÆVEFô7&VFRÀÐ¢vÖTWfVçC£¥Æ–W%&VÖF6„6†ævR…ò’ÓâvÖTWfVçEG—S£¥Æ–W%&VÖF6„6†ævRÀÐ¢vÖTWfVçC£¥–æuWFFVB…ò’ÓâvÖTWfVçEG—S£¥–æuWFFVBÀÐ¢vÖTWfVçC£¤ÔÕ7FG5WFFVB…ò’ÓâvÖTWfVçEG—S£¤ÔÕ7FG5WFFVBÀÐ¢vÖTWfVçC£¥Æ–W$æW‡DÖf÷FT6†ævR…ò’ÓâvÖTWfVçEG—S£¥Æ–W$æW‡DÖf÷FT6†ævRÀÐ¢vÖTWfVçC£¥f÷FTÖ46†ævVB…ò’ÓâvÖTWfVçEG—S£¥f÷FTÖ46†ævVBÀÐ¢vÖTWfVçC£¥&÷FôFVd6†ævVB…ò’ÓâvÖTWfVçEG—S£¥&÷FôFVd6†ævVBÀÐ¢vÖTWfVçC£¥Æ–W$FöÖ–æF–öâ…ò’ÓâvÖTWfVçEG—S£¥Æ–W$FöÖ–æF–öâÀÐ¢vÖTWfVçC£¥Æ–W%&ö6¶WE6µW6†VB…ò’ÓâvÖTWfVçEG—S£¥Æ–W%&ö6¶WE6µW6†VBÀÐ¢vÖTWfVçC£¥VW7E&WVW7B…ò’ÓâvÖTWfVçEG—S£¥VW7E&WVW7BÀÐ¢vÖTWfVçC£¥VW7E&W7öç6R…ò’ÓâvÖTWfVçEG—S£¥VW7E&W7öç6RÀÐ¢vÖTWfVçC£¥VW7E&öw&W72…ò’ÓâvÖTWfVçEG—S£¥VW7E&öw&W72ÀÐ¢vÖTWfVçC£¥&ö¦V7F–ÆU&VÖ÷fVB…ò’ÓâvÖTWfVçEG—S£¥&ö¦V7F–ÆU&VÖ÷fVBÀÐ¢vÖTWfVçC£¥VW7DÖFF6†ævVB…ò’ÓâvÖTWfVçEG—S£¥VW7DÖFF6†ævVBÀÐ¢vÖTWfVçC£¤v4F÷W6VEÆ–W$–væ—FVB…ò’ÓâvÖTWfVçEG—S£¤v4F÷W6VEÆ–W$–væ—FVBÀÐ¢vÖTWfVçC£¥VW7EGW&ä–å7FFR…ò’ÓâvÖTWfVçEG—S£¥VW7EGW&ä–å7FFRÀÐ¢vÖTWfVçC£¤—FV×46¶æ÷vÆVFvVB…ò’ÓâvÖTWfVçEG—S£¤—FV×46¶æ÷vÆVFvVBÀÐ¢vÖTWfVçC£¤6W$¶–ÆÆVB…ò’ÓâvÖTWfVçEG—S£¤6W$¶–ÆÆVBÀÐ¢vÖTWfVçC£¤Ö–äÖVçU7F&–Æ—¦VB…ò’ÓâvÖTWfVçEG—S£¤Ö–äÖVçU7F&–Æ—¦VBÀÐ¢vÖTWfVçC£¥v÷&ÆE7FGW46†ævVB…ò’ÓâvÖTWfVçEG—S£¥v÷&ÆE7FGW46†ævVBÀÐ¢vÖTWfVçC£¤„ÅEe7FGW2…ò’ÓâvÖTWfVçEG—S£¤„ÅEe7FGW2ÀÐ¢vÖTWfVçC£¤„ÅEd6ÖW&Öâ…ò’ÓâvÖTWfVçEG—S£¤„ÅEd6ÖW&ÖâÀÐ¢vÖTWfVçC£¤„ÅEe&æ´6ÖW&…ò’ÓâvÖTWfVçEG—S£¤„ÅEe&æ´6ÖW&ÀÐ¢vÖTWfVçC£¤„ÅEe&æ´VçF—G’…ò’ÓâvÖTWfVçEG—S£¤„ÅEe&æ´VçF—G’ÀÐ¢vÖTWfVçC£¤„ÅEdf—†VB…ò’ÓâvÖTWfVçEG—S£¤„ÅEdf—†VBÀÐ¢vÖTWfVçC£¤„ÅEd6†6R…ò’ÓâvÖTWfVçEG—S£¤„ÅEd6†6RÀÐ¢vÖTWfVçC£¤„ÅEdÖW76vR…ò’ÓâvÖTWfVçEG—S£¤„ÅEdÖW76vRÀÐ¢vÖTWfVçC£¤„ÅEeF—FÆR…ò’ÓâvÖTWfVçEG—S£¤„ÅEeF—FÆRÀÐ¢vÖTWfVçC£¤„ÅEd6†B…ò’ÓâvÖTWfVçEG—S£¤„ÅEd6†BÀÐ¢vÖTWfVçC£¥&WÆ•7F'E&V6÷&B…ò’ÓâvÖTWfVçEG—S£¥&WÆ•7F'E&V6÷&BÀÐ¢vÖTWfVçC£¥&WÆ•6W76–öä–æfò…ò’ÓâvÖTWfVçEG—S£¥&WÆ•6W76–öä–æfòÀÐ¢vÖTWfVçC£¥&WÆ”VæE&V6÷&B…ò’ÓâvÖTWfVçEG—S£¥&WÆ”VæE&V6÷&BÀÐ¢vÖTWfVçC£¥&WÆ•&WÆ—4f–Æ&ÆR…ò’ÓâvÖTWfVçEG—S£¥&WÆ•&WÆ—4f–Æ&ÆRÀÐ¢vÖTWfVçC£¥&WÆ•6W'fW$W'&÷"…ò’ÓâvÖTWfVçEG—S£¥&WÆ•6W'fW$W'&÷"ÀÐ¢vÖTWfVçC£¥Væ¶æ÷vâ‡&r’Óâ&ræWfVçE÷G—Ræ6ÆöæR‚’ÀÐ¢ÐÐ¢ÐÐ§ÐÐ§V"fâvWE÷6—¦W2‚’Óâfçc£¤fçd†6„ÖÂbw7FF–27G"ÂW6—¦Sâ°Ð¢°Ð¢‚%6W'fW%7vâ"Â6—¦Uööc££Å6W'fW%7väWfVçCâ‚’’ÀÐ¢€Ð¢%6W'fW$6†ævTÆWfVÄf–ÆVB"ÀÐ¢6—¦Uööc££Å6W'fW$6†ævTÆWfVÄf–ÆVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%6W'fW%6‡WFF÷vâ"Â6—¦Uööc££Å6W'fW%6‡WFF÷väWfVçCâ‚’’ÀÐ¢‚%6W'fW$7f""Â6—¦Uööc££Å6W'fW$7f$WfVçCâ‚’’ÀÐ¢‚%6W'fW$ÖW76vR"Â6—¦Uööc££Å6W'fW$ÖW76vTWfVçCâ‚’’ÀÐ¢‚%6W'fW$FD&â"Â6—¦Uööc££Å6W'fW$FD&äWfVçCâ‚’’ÀÐ¢‚%6W'fW%&VÖ÷fT&â"Â6—¦Uööc££Å6W'fW%&VÖ÷fT&äWfVçCâ‚’’ÀÐ¢‚%Æ–W$6öææV7B"Â6—¦Uööc££ÅÆ–W$6öææV7DWfVçCâ‚’’ÀÐ¢‚%Æ–W$6öææV7D6Æ–VçB"Â6—¦Uööc££ÅÆ–W$6öææV7D6Æ–VçDWfVçCâ‚’’ÀÐ¢‚%Æ–W$–æfò"Â6—¦Uööc££ÅÆ–W$–æfôWfVçCâ‚’’ÀÐ¢‚%Æ–W$F—66öææV7B"Â6—¦Uööc££ÅÆ–W$F—66öææV7DWfVçCâ‚’’ÀÐ¢‚%Æ–W$7F—fFR"Â6—¦Uööc££ÅÆ–W$7F—fFTWfVçCâ‚’’ÀÐ¢‚%Æ–W%6’"Â6—¦Uööc££ÅÆ–W%6”WfVçCâ‚’’ÀÐ¢‚$6Æ–VçDF—66öææV7B"Â6—¦Uööc££Ä6Æ–VçDF—66öææV7DWfVçCâ‚’’ÀÐ¢‚$6Æ–VçD&Vv–ä6öææV7B"Â6—¦Uööc££Ä6Æ–VçD&Vv–ä6öææV7DWfVçCâ‚’’ÀÐ¢‚$6Æ–VçD6öææV7FVB"Â6—¦Uööc££Ä6Æ–VçD6öææV7FVDWfVçCâ‚’’ÀÐ¢‚$6Æ–VçDgVÆÄ6öææV7B"Â6—¦Uööc££Ä6Æ–VçDgVÆÄ6öææV7DWfVçCâ‚’’ÀÐ¢‚$†÷7EV—B"Â6—¦Uööc££Ä†÷7EV—DWfVçCâ‚’’ÀÐ¢‚%FVÔ–æfò"Â6—¦Uööc££ÅFVÔ–æfôWfVçCâ‚’’ÀÐ¢‚%FVÕ66÷&R"Â6—¦Uööc££ÅFVÕ66÷&TWfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ”'&öF67DVF–ò"ÀÐ¢6—¦Uööc££ÅFVÕÆ”'&öF67DVF–ôWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W%FVÒ"Â6—¦Uööc££ÅÆ–W%FVÔWfVçCâ‚’’ÀÐ¢‚%Æ–W$6Æ72"Â6—¦Uööc££ÅÆ–W$6Æ74WfVçCâ‚’’ÀÐ¢‚%Æ–W$FVF‚"Â6—¦Uööc££ÅÆ–W$FVF„WfVçCâ‚’’ÀÐ¢‚%Æ–W$‡W'B"Â6—¦Uööc££ÅÆ–W$‡W'DWfVçCâ‚’’ÀÐ¢‚%Æ–W$6†B"Â6—¦Uööc££ÅÆ–W$6†DWfVçCâ‚’’ÀÐ¢‚%Æ–W%66÷&R"Â6—¦Uööc££ÅÆ–W%66÷&TWfVçCâ‚’’ÀÐ¢‚%Æ–W%7vâ"Â6—¦Uööc££ÅÆ–W%7väWfVçCâ‚’’ÀÐ¢‚%Æ–W%6†ö÷B"Â6—¦Uööc££ÅÆ–W%6†ö÷DWfVçCâ‚’’ÀÐ¢‚%Æ–W%W6R"Â6—¦Uööc££ÅÆ–W%W6TWfVçCâ‚’’ÀÐ¢‚%Æ–W$6†ævTæÖR"Â6—¦Uööc££ÅÆ–W$6†ævTæÖTWfVçCâ‚’’ÀÐ¢‚%Æ–W$†–çDÖW76vR"Â6—¦Uööc££ÅÆ–W$†–çDÖW76vTWfVçCâ‚’’ÀÐ¢€Ð¢$&6UÆ–W%FVÆW÷'FVB"ÀÐ¢6—¦Uööc££Ä&6UÆ–W%FVÆW÷'FVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$vÖT–æ—B"Â6—¦Uööc££ÄvÖT–æ—DWfVçCâ‚’’ÀÐ¢‚$vÖTæWtÖ"Â6—¦Uööc££ÄvÖTæWtÖWfVçCâ‚’’ÀÐ¢‚$vÖU7F'B"Â6—¦Uööc££ÄvÖU7F'DWfVçCâ‚’’ÀÐ¢‚$vÖTVæB"Â6—¦Uööc££ÄvÖTVæDWfVçCâ‚’’ÀÐ¢‚%&÷VæE7F'B"Â6—¦Uööc££Å&÷VæE7F'DWfVçCâ‚’’ÀÐ¢‚%&÷VæDVæB"Â6—¦Uööc££Å&÷VæDVæDWfVçCâ‚’’ÀÐ¢‚$vÖTÖW76vR"Â6—¦Uööc££ÄvÖTÖW76vTWfVçCâ‚’’ÀÐ¢‚$'&V´'&V¶&ÆR"Â6—¦Uööc££Ä'&V´'&V¶&ÆTWfVçCâ‚’’ÀÐ¢‚$'&Vµ&÷"Â6—¦Uööc££Ä'&Vµ&÷WfVçCâ‚’’ÀÐ¢‚$VçF—G”¶–ÆÆVB"Â6—¦Uööc££ÄVçF—G”¶–ÆÆVDWfVçCâ‚’’ÀÐ¢‚$&öçW5WFFVB"Â6—¦Uööc££Ä&öçW5WFFVDWfVçCâ‚’’ÀÐ¢‚$6†–WfVÖVçDWfVçB"Â6—¦Uööc££Ä6†–WfVÖVçDWfVçDWfVçCâ‚’’ÀÐ¢€Ð¢$6†–WfVÖVçD–æ7&VÖVçB"ÀÐ¢6—¦Uööc££Ä6†–WfVÖVçD–æ7&VÖVçDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%‡—6wVå–6·W"Â6—¦Uööc££Å‡—6wVå–6·WWfVçCâ‚’’ÀÐ¢‚$fÆ&T–væ—FTç2"Â6—¦Uööc££ÄfÆ&T–væ—FTç4WfVçCâ‚’’ÀÐ¢€Ð¢$†VÆ–6÷FW$w&VæFUVçDÖ—72"ÀÐ¢6—¦Uööc££Ä†VÆ–6÷FW$w&VæFUVçDÖ—74WfVçCâ‚’ÀÐ¢’ÀÐ¢‚%W6W$FFF÷væÆöFVB"Â6—¦Uööc££ÅW6W$FFF÷væÆöFVDWfVçCâ‚’’ÀÐ¢‚%&vFöÆÄF—76öÇfVB"Â6—¦Uööc££Å&vFöÆÄF—76öÇfVDWfVçCâ‚’’ÀÐ¢‚$„ÅEd6†ævVDÖöFR"Â6—¦Uööc££Ä„ÅEd6†ævVDÖöFTWfVçCâ‚’’ÀÐ¢‚$„ÅEd6†ævVEF&vWB"Â6—¦Uööc££Ä„ÅEd6†ævVEF&vWDWfVçCâ‚’’ÀÐ¢‚%f÷FTVæFVB"Â6—¦Uööc££Åf÷FTVæFVDWfVçCâ‚’’ÀÐ¢‚%f÷FU7F'FVB"Â6—¦Uööc££Åf÷FU7F'FVDWfVçCâ‚’’ÀÐ¢‚%f÷FT6†ævVB"Â6—¦Uööc££Åf÷FT6†ævVDWfVçCâ‚’’ÀÐ¢‚%f÷FU76VB"Â6—¦Uööc££Åf÷FU76VDWfVçCâ‚’’ÀÐ¢‚%f÷FTf–ÆVB"Â6—¦Uööc££Åf÷FTf–ÆVDWfVçCâ‚’’ÀÐ¢‚%f÷FT67B"Â6—¦Uööc££Åf÷FT67DWfVçCâ‚’’ÀÐ¢‚%f÷FT÷F–öç2"Â6—¦Uööc££Åf÷FT÷F–öç4WfVçCâ‚’’ÀÐ¢‚%&WÆ•6fVB"Â6—¦Uööc££Å&WÆ•6fVDWfVçCâ‚’’ÀÐ¢€Ð¢$VçFW&VEW&f÷&Öæ6TÖöFR"ÀÐ¢6—¦Uööc££ÄVçFW&VEW&f÷&Öæ6TÖöFTWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$'&÷w6U&WÆ—2"Â6—¦Uööc££Ä'&÷w6U&WÆ—4WfVçCâ‚’’ÀÐ¢‚%&WÆ•–÷WGV&U7FG2"Â6—¦Uööc££Å&WÆ•–÷WGV&U7FG4WfVçCâ‚’’ÀÐ¢‚$–çfVçF÷'•WFFVB"Â6—¦Uööc££Ä–çfVçF÷'•WFFVDWfVçCâ‚’’ÀÐ¢‚$6'EWFFVB"Â6—¦Uööc££Ä6'EWFFVDWfVçCâ‚’’ÀÐ¢€Ð¢%7F÷&U&–6U6†VWEWFFVB"ÀÐ¢6—¦Uööc££Å7F÷&U&–6U6†VWEWFFVDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$V6öä–çfVçF÷'”6öææV7FVB"ÀÐ¢6—¦Uööc££ÄV6öä–çfVçF÷'”6öææV7FVDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$—FVÕ66†VÖ–æ—F–Æ—¦VB"ÀÐ¢6—¦Uööc££Ä—FVÕ66†VÖ–æ—F–Æ—¦VDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$v4æWu6W76–öâ"Â6—¦Uööc££Äv4æWu6W76–öäWfVçCâ‚’’ÀÐ¢‚$v4Æ÷7E6W76–öâ"Â6—¦Uööc££Äv4Æ÷7E6W76–öäWfVçCâ‚’’ÀÐ¢‚$–çG&ôf–æ—6‚"Â6—¦Uööc££Ä–çG&ôf–æ—6„WfVçCâ‚’’ÀÐ¢‚$–çG&ôæW‡D6ÖW&"Â6—¦Uööc££Ä–çG&ôæW‡D6ÖW&WfVçCâ‚’’ÀÐ¢‚%Æ–W$6†ævT6Æ72"Â6—¦Uööc££ÅÆ–W$6†ævT6Æ74WfVçCâ‚’’ÀÐ¢‚%FdÖF–ÖU&VÖ–æ–ær"Â6—¦Uööc££ÅFdÖF–ÖU&VÖ–æ–ætWfVçCâ‚’’ÀÐ¢‚%FdvÖT÷fW""Â6—¦Uööc££ÅFdvÖT÷fW$WfVçCâ‚’’ÀÐ¢‚$7FdfÆt6GW&VB"Â6—¦Uööc££Ä7FdfÆt6GW&VDWfVçCâ‚’’ÀÐ¢€Ð¢$6öçG&öÅö–çD–æ—F–Æ—¦VB"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çD–æ—F–Æ—¦VDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çEWFFT–ÖvW2"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çEWFFT–ÖvW4WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çEWFFTÆ–÷WB"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çEWFFTÆ–÷WDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çEWFFT6–ær"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çEWFFT6–ætWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çEWFFT÷væW""ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çEWFFT÷væW$WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çE7F'EF÷V6‚"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çE7F'EF÷V6„WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çDVæEF÷V6‚"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çDVæEF÷V6„WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çEVÇ6TVÆVÖVçB"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çEVÇ6TVÆVÖVçDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çDf¶T6GW&R"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çDf¶T6GW&TWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W""ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çDf¶T6GW&T×VÇF—Æ–W$WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ•&÷VæE6VÆV7FVB"ÀÐ¢6—¦Uööc££ÅFVÕÆ•&÷VæE6VÆV7FVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%FVÕÆ•&÷VæE7F'B"Â6—¦Uööc££ÅFVÕÆ•&÷VæE7F'DWfVçCâ‚’’ÀÐ¢‚%FVÕÆ•&÷VæD7F—fR"Â6—¦Uööc££ÅFVÕÆ•&÷VæD7F—fTWfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ•v—F–æt&Vv–ç2"ÀÐ¢6—¦Uööc££ÅFVÕÆ•v—F–æt&Vv–ç4WfVçCâ‚’ÀÐ¢’ÀÐ¢‚%FVÕÆ•v—F–ætVæG2"Â6—¦Uööc££ÅFVÕÆ•v—F–ætVæG4WfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ•v—F–æt&÷WEFôVæB"ÀÐ¢6—¦Uööc££ÅFVÕÆ•v—F–æt&÷WEFôVæDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ•&W7F'E&÷VæB"ÀÐ¢6—¦Uööc££ÅFVÕÆ•&W7F'E&÷VæDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ•&VG•&W7F'B"ÀÐ¢6—¦Uööc££ÅFVÕÆ•&VG•&W7F'DWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ•&÷VæE&W7F'E6V6öæG2"ÀÐ¢6—¦Uööc££ÅFVÕÆ•&÷VæE&W7F'E6V6öæG4WfVçCâ‚’ÀÐ¢’ÀÐ¢‚%FVÕÆ•FVÕ&VG’"Â6—¦Uööc££ÅFVÕÆ•FVÕ&VG”WfVçCâ‚’’ÀÐ¢‚%FVÕÆ•&÷VæEv–â"Â6—¦Uööc££ÅFVÕÆ•&÷VæEv–äWfVçCâ‚’’ÀÐ¢‚%FVÕÆ•WFFUF–ÖW""Â6—¦Uööc££ÅFVÕÆ•WFFUF–ÖW$WfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ•&÷VæE7FÆVÖFR"ÀÐ¢6—¦Uööc££ÅFVÕÆ•&÷VæE7FÆVÖFTWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ”÷fW'F–ÖT&Vv–â"ÀÐ¢6—¦Uööc££ÅFVÕÆ”÷fW'F–ÖT&Vv–äWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%FVÕÆ”÷fW'F–ÖTVæB"Â6—¦Uööc££ÅFVÕÆ”÷fW'F–ÖTVæDWfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ•7VFFVäFVF„&Vv–â"ÀÐ¢6—¦Uööc££ÅFVÕÆ•7VFFVäFVF„&Vv–äWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ•7VFFVäFVF„VæB"ÀÐ¢6—¦Uööc££ÅFVÕÆ•7VFFVäFVF„VæDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%FVÕÆ”vÖT÷fW""Â6—¦Uööc££ÅFVÕÆ”vÖT÷fW$WfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ”ÖF–ÖU&VÖ–æ–ær"ÀÐ¢6—¦Uööc££ÅFVÕÆ”ÖF–ÖU&VÖ–æ–ætWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%FVÕÆ•F–ÖW$fÆ6‚"Â6—¦Uööc££ÅFVÕÆ•F–ÖW$fÆ6„WfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ•F–ÖW%F–ÖTFFVB"ÀÐ¢6—¦Uööc££ÅFVÕÆ•F–ÖW%F–ÖTFFVDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ•ö–çE7F'D6GW&R"ÀÐ¢6—¦Uööc££ÅFVÕÆ•ö–çE7F'D6GW&TWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ•ö–çD6GW&VB"ÀÐ¢6—¦Uööc££ÅFVÕÆ•ö–çD6GW&VDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%FVÕÆ•ö–çDÆö6¶VB"Â6—¦Uööc££ÅFVÕÆ•ö–çDÆö6¶VDWfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ•ö–çEVæÆö6¶VB"ÀÐ¢6—¦Uööc££ÅFVÕÆ•ö–çEVæÆö6¶VDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ”6GW&T'&ö¶Vâ"ÀÐ¢6—¦Uööc££ÅFVÕÆ”6GW&T'&ö¶VäWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ”6GW&T&Æö6¶VB"ÀÐ¢6—¦Uööc££ÅFVÕÆ”6GW&T&Æö6¶VDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%FVÕÆ”fÆtWfVçB"Â6—¦Uööc££ÅFVÕÆ”fÆtWfVçDWfVçCâ‚’’ÀÐ¢‚%FVÕÆ•v–åæVÂ"Â6—¦Uööc££ÅFVÕÆ•v–åæVÄWfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ•FVÔ&Ææ6VEÆ–W""ÀÐ¢6—¦Uööc££ÅFVÕÆ•FVÔ&Ææ6VEÆ–W$WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%FVÕÆ•6WGWf–æ—6†VB"ÀÐ¢6—¦Uööc££ÅFVÕÆ•6WGWf–æ—6†VDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%FVÕÆ”ÆW'B"Â6—¦Uööc££ÅFVÕÆ”ÆW'DWfVçCâ‚’’ÀÐ¢‚%G&–æ–æt6ö×ÆWFR"Â6—¦Uööc££ÅG&–æ–æt6ö×ÆWFTWfVçCâ‚’’ÀÐ¢‚%6†÷tg&VW¦UæVÂ"Â6—¦Uööc££Å6†÷tg&VW¦UæVÄWfVçCâ‚’’ÀÐ¢‚$†–FTg&VW¦UæVÂ"Â6—¦Uööc££Ä†–FTg&VW¦UæVÄWfVçCâ‚’’ÀÐ¢‚$g&VW¦T6Õ7F'FVB"Â6—¦Uööc££Äg&VW¦T6Õ7F'FVDWfVçCâ‚’’ÀÐ¢€Ð¢$Æö6ÅÆ–W$6†ævUFVÒ"ÀÐ¢6—¦Uööc££ÄÆö6ÅÆ–W$6†ævUFVÔWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$Æö6ÅÆ–W%66÷&T6†ævVB"ÀÐ¢6—¦Uööc££ÄÆö6ÅÆ–W%66÷&T6†ævVDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$Æö6ÅÆ–W$6†ævT6Æ72"ÀÐ¢6—¦Uööc££ÄÆö6ÅÆ–W$6†ævT6Æ74WfVçCâ‚’ÀÐ¢’ÀÐ¢‚$Æö6ÅÆ–W%&W7vâ"Â6—¦Uööc££ÄÆö6ÅÆ–W%&W7väWfVçCâ‚’’ÀÐ¢‚$'V–ÆF–æt–æfô6†ævVB"Â6—¦Uööc££Ä'V–ÆF–æt–æfô6†ævVDWfVçCâ‚’’ÀÐ¢€Ð¢$Æö6ÅÆ–W$6†ævTF—6wV—6R"ÀÐ¢6—¦Uööc££ÄÆö6ÅÆ–W$6†ævTF—6wV—6TWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%Æ–W$66÷VçD6†ævVB"ÀÐ¢6—¦Uööc££ÅÆ–W$66÷VçD6†ævVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%7•F&W6WB"Â6—¦Uööc££Å7•F&W6WDWfVçCâ‚’’ÀÐ¢‚$fÆu7FGW5WFFR"Â6—¦Uööc££ÄfÆu7FGW5WFFTWfVçCâ‚’’ÀÐ¢‚%Æ–W%7FG5WFFVB"Â6—¦Uööc££ÅÆ–W%7FG5WFFVDWfVçCâ‚’’ÀÐ¢‚%Æ––æt6öÖÖVçF'’"Â6—¦Uööc££ÅÆ––æt6öÖÖVçF'”WfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W$6†&vTFWÆ÷–VB"ÀÐ¢6—¦Uööc££ÅÆ–W$6†&vTFWÆ÷–VDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W$'V–ÇDö&¦V7B"Â6—¦Uööc££ÅÆ–W$'V–ÇDö&¦V7DWfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W%Ww&FVDö&¦V7B"ÀÐ¢6—¦Uööc££ÅÆ–W%Ww&FVDö&¦V7DWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W$6''”ö&¦V7B"Â6—¦Uööc££ÅÆ–W$6''”ö&¦V7DWfVçCâ‚’’ÀÐ¢‚%Æ–W$G&÷ö&¦V7B"Â6—¦Uööc££ÅÆ–W$G&÷ö&¦V7DWfVçCâ‚’’ÀÐ¢‚$ö&¦V7E&VÖ÷fVB"Â6—¦Uööc££Äö&¦V7E&VÖ÷fVDWfVçCâ‚’’ÀÐ¢‚$ö&¦V7DFW7G&÷–VB"Â6—¦Uööc££Äö&¦V7DFW7G&÷–VDWfVçCâ‚’’ÀÐ¢‚$ö&¦V7DFWFöæFVB"Â6—¦Uööc££Äö&¦V7DFWFöæFVDWfVçCâ‚’’ÀÐ¢‚$6†–WfVÖVçDV&æVB"Â6—¦Uööc££Ä6†–WfVÖVçDV&æVDWfVçCâ‚’’ÀÐ¢‚%7V5F&vWEWFFVB"Â6—¦Uööc££Å7V5F&vWEWFFVDWfVçCâ‚’’ÀÐ¢€Ð¢%F÷W&æÖVçE7FFUWFFR"ÀÐ¢6—¦Uööc££ÅF÷W&æÖVçE7FFUWFFTWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%F÷W&æÖVçDVæ&ÆT6÷VçFF÷vâ"ÀÐ¢6—¦Uööc££ÅF÷W&æÖVçDVæ&ÆT6÷VçFF÷väWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%Æ–W$6ÆÆVDf÷$ÖVF–2"ÀÐ¢6—¦Uööc££ÅÆ–W$6ÆÆVDf÷$ÖVF–4WfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W$6¶VDf÷$&ÆÂ"Â6—¦Uööc££ÅÆ–W$6¶VDf÷$&ÆÄWfVçCâ‚’’ÀÐ¢€Ð¢$Æö6ÅÆ–W$&V6ÖTö'6W'fW""ÀÐ¢6—¦Uööc££ÄÆö6ÅÆ–W$&V6ÖTö'6W'fW$WfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W$–væ—FVD–çb"Â6—¦Uööc££ÅÆ–W$–væ—FVD–çdWfVçCâ‚’’ÀÐ¢‚%Æ–W$–væ—FVB"Â6—¦Uööc££ÅÆ–W$–væ—FVDWfVçCâ‚’’ÀÐ¢‚%Æ–W$W‡F–æwV—6†VB"Â6—¦Uööc££ÅÆ–W$W‡F–æwV—6†VDWfVçCâ‚’’ÀÐ¢‚%Æ–W%FVÆW÷'FVB"Â6—¦Uööc££ÅÆ–W%FVÆW÷'FVDWfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W$†VÆVDÖVF–46ÆÂ"ÀÐ¢6—¦Uööc££ÅÆ–W$†VÆVDÖVF–46ÆÄWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$Æö6ÅÆ–W$6†&vU&VG’"ÀÐ¢6—¦Uööc££ÄÆö6ÅÆ–W$6†&vU&VG”WfVçCâ‚’ÀÐ¢’ÀÐ¢‚$Æö6ÅÆ–W%v–æDF÷vâ"Â6—¦Uööc££ÄÆö6ÅÆ–W%v–æDF÷väWfVçCâ‚’’ÀÐ¢‚%Æ–W$–çgVÆæVB"Â6—¦Uööc££ÅÆ–W$–çgVÆæVDWfVçCâ‚’’ÀÐ¢‚$W66÷'E7VVB"Â6—¦Uööc££ÄW66÷'E7VVDWfVçCâ‚’’ÀÐ¢‚$W66÷'E&öw&W72"Â6—¦Uööc££ÄW66÷'E&öw&W74WfVçCâ‚’’ÀÐ¢‚$W66÷'E&V6VFR"Â6—¦Uööc££ÄW66÷'E&V6VFTWfVçCâ‚’’ÀÐ¢‚$vÖUT”7F—fFVB"Â6—¦Uööc££ÄvÖUT”7F—fFVDWfVçCâ‚’’ÀÐ¢‚$vÖUT”†–FFVâ"Â6—¦Uööc££ÄvÖUT”†–FFVäWfVçCâ‚’’ÀÐ¢‚%Æ–W$W66÷'E66÷&R"Â6—¦Uööc££ÅÆ–W$W66÷'E66÷&TWfVçCâ‚’’ÀÐ¢‚%Æ–W$†VÄöä†—B"Â6—¦Uööc££ÅÆ–W$†VÄöä†—DWfVçCâ‚’’ÀÐ¢‚%Æ–W%7FVÅ6æGf–6‚"Â6—¦Uööc££ÅÆ–W%7FVÅ6æGf–6„WfVçCâ‚’’ÀÐ¢‚%6†÷t6Æ74Æ–÷WB"Â6—¦Uööc££Å6†÷t6Æ74Æ–÷WDWfVçCâ‚’’ÀÐ¢‚%6†÷ug5æVÂ"Â6—¦Uööc££Å6†÷ug5æVÄWfVçCâ‚’’ÀÐ¢‚%Æ–W$FÖvVB"Â6—¦Uööc££ÅÆ–W$FÖvVDWfVçCâ‚’’ÀÐ¢€Ð¢$&VæÆ–W$æ÷F–f–6F–öâ"ÀÐ¢6—¦Uööc££Ä&VæÆ–W$æ÷F–f–6F–öäWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$&VæÖF6„Ö…7G&V²"Â6—¦Uööc££Ä&VæÖF6„Ö…7G&V´WfVçCâ‚’’ÀÐ¢‚$&Væ&÷VæE7F'B"Â6—¦Uööc££Ä&Væ&÷VæE7F'DWfVçCâ‚’’ÀÐ¢‚$&Væv–åæVÂ"Â6—¦Uööc££Ä&Væv–åæVÄWfVçCâ‚’’ÀÐ¢‚%fUv–åæVÂ"Â6—¦Uööc££ÅfUv–åæVÄWfVçCâ‚’’ÀÐ¢‚$—$F6‚"Â6—¦Uööc££Ä—$F6„WfVçCâ‚’’ÀÐ¢‚$ÆæFVB"Â6—¦Uööc££ÄÆæFVDWfVçCâ‚’’ÀÐ¢‚%Æ–W$FÖvTFöFvVB"Â6—¦Uööc££ÅÆ–W$FÖvTFöFvVDWfVçCâ‚’’ÀÐ¢‚%Æ–W%7GVææVB"Â6—¦Uööc££ÅÆ–W%7GVææVDWfVçCâ‚’’ÀÐ¢‚%66÷WDw&æE6ÆÒ"Â6—¦Uööc££Å66÷WDw&æE6ÆÔWfVçCâ‚’’ÀÐ¢‚%66÷WE6ÆÖFöÆÄÆæFVB"Â6—¦Uööc££Å66÷WE6ÆÖFöÆÄÆæFVDWfVçCâ‚’’ÀÐ¢‚$'&÷t–×7B"Â6—¦Uööc££Ä'&÷t–×7DWfVçCâ‚’’ÀÐ¢‚%Æ–W$¦&FVB"Â6—¦Uööc££ÅÆ–W$¦&FVDWfVçCâ‚’’ÀÐ¢‚%Æ–W$¦&FVDfFR"Â6—¦Uööc££ÅÆ–W$¦&FVDfFTWfVçCâ‚’’ÀÐ¢‚%Æ–W%6†–VÆD&Æö6¶VB"Â6—¦Uööc££ÅÆ–W%6†–VÆD&Æö6¶VDWfVçCâ‚’’ÀÐ¢‚%Æ–W%–ææVB"Â6—¦Uööc££ÅÆ–W%–ææVDWfVçCâ‚’’ÀÐ¢‚%Æ–W$†VÆVD'”ÖVF–2"Â6—¦Uööc££ÅÆ–W$†VÆVD'”ÖVF–4WfVçCâ‚’’ÀÐ¢‚%Æ–W%6VDö&¦V7B"Â6—¦Uööc££ÅÆ–W%6VDö&¦V7DWfVçCâ‚’’ÀÐ¢‚$—FVÔf÷VæB"Â6—¦Uööc££Ä—FVÔf÷VæDWfVçCâ‚’’ÀÐ¢‚%6†÷tææ÷FF–öâ"Â6—¦Uööc££Å6†÷tææ÷FF–öäWfVçCâ‚’’ÀÐ¢‚$†–FTææ÷FF–öâ"Â6—¦Uööc££Ä†–FTææ÷FF–öäWfVçCâ‚’’ÀÐ¢€Ð¢%÷7D–çfVçF÷'”Æ–6F–öâ"ÀÐ¢6—¦Uööc££Å÷7D–çfVçF÷'”Æ–6F–öäWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6öçG&öÅö–çEVæÆö6µWFFVB"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çEVæÆö6µWFFVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$FWÆ÷”'Vfd&ææW""Â6—¦Uööc££ÄFWÆ÷”'Vfd&ææW$WfVçCâ‚’’ÀÐ¢‚%Æ–W$'Vfb"Â6—¦Uööc££ÅÆ–W$'VfdWfVçCâ‚’’ÀÐ¢‚$ÖVF–4FVF‚"Â6—¦Uööc££ÄÖVF–4FVF„WfVçCâ‚’’ÀÐ¢‚$÷fW'F–ÖTær"Â6—¦Uööc££Ä÷fW'F–ÖTætWfVçCâ‚’’ÀÐ¢‚%FV×46†ævVB"Â6—¦Uööc££ÅFV×46†ævVDWfVçCâ‚’’ÀÐ¢€Ð¢$†ÆÆ÷vVVåV×¶–äw&""ÀÐ¢6—¦Uööc££Ä†ÆÆ÷vVVåV×¶–äw&$WfVçCâ‚’ÀÐ¢’ÀÐ¢‚%&ö6¶WD§V×"Â6—¦Uööc££Å&ö6¶WD§V×WfVçCâ‚’’ÀÐ¢‚%&ö6¶WD§V×ÆæFVB"Â6—¦Uööc££Å&ö6¶WD§V×ÆæFVDWfVçCâ‚’’ÀÐ¢‚%7F–6·”§V×"Â6—¦Uööc££Å7F–6·”§V×WfVçCâ‚’’ÀÐ¢‚%7F–6·”§V×ÆæFVB"Â6—¦Uööc££Å7F–6·”§V×ÆæFVDWfVçCâ‚’’ÀÐ¢‚%&ö6¶WE6´ÆVæ6‚"Â6—¦Uööc££Å&ö6¶WE6´ÆVæ6„WfVçCâ‚’’ÀÐ¢‚%&ö6¶WE6´ÆæFVB"Â6—¦Uööc££Å&ö6¶WE6´ÆæFVDWfVçCâ‚’’ÀÐ¢‚$ÖVF–4FVfVæFVB"Â6—¦Uööc££ÄÖVF–4FVfVæFVDWfVçCâ‚’’ÀÐ¢‚$Æö6ÅÆ–W$†VÆVB"Â6—¦Uööc££ÄÆö6ÅÆ–W$†VÆVDWfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W$FW7G&÷–VE—T&öÖ""ÀÐ¢6—¦Uööc££ÅÆ–W$FW7G&÷–VE—T&öÖ$WfVçCâ‚’ÀÐ¢’ÀÐ¢‚$ö&¦V7DFVfÆV7FVB"Â6—¦Uööc££Äö&¦V7DFVfÆV7FVDWfVçCâ‚’’ÀÐ¢‚%Æ–W$×g"Â6—¦Uööc££ÅÆ–W$×gWfVçCâ‚’’ÀÐ¢‚%&–E7väÖö""Â6—¦Uööc££Å&–E7väÖö$WfVçCâ‚’’ÀÐ¢‚%&–E7vå7VB"Â6—¦Uööc££Å&–E7vå7VDWfVçCâ‚’’ÀÐ¢‚$æd&Æö6¶VB"Â6—¦Uööc££Äæd&Æö6¶VDWfVçCâ‚’’ÀÐ¢‚%F…G&6µ76VB"Â6—¦Uööc££ÅF…G&6µ76VDWfVçCâ‚’’ÀÐ¢‚$çVÔ6W'46†ævVB"Â6—¦Uööc££ÄçVÔ6W'46†ævVDWfVçCâ‚’’ÀÐ¢‚%Æ–W%&VvVæW&FR"Â6—¦Uööc££ÅÆ–W%&VvVæW&FTWfVçCâ‚’’ÀÐ¢‚%WFFU7FGW4—FVÒ"Â6—¦Uööc££ÅWFFU7FGW4—FVÔWfVçCâ‚’’ÀÐ¢‚%7FG5&W6WE&÷VæB"Â6—¦Uööc££Å7FG5&W6WE&÷VæDWfVçCâ‚’’ÀÐ¢€Ð¢%66÷&U7FG467V×VÆFVEWFFR"ÀÐ¢6—¦Uööc££Å66÷&U7FG467V×VÆFVEWFFTWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%66÷&U7FG467V×VÆFVE&W6WB"ÀÐ¢6—¦Uööc££Å66÷&U7FG467V×VÆFVE&W6WDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6†–WfVÖVçDV&æVDÆö6Â"ÀÐ¢6—¦Uööc££Ä6†–WfVÖVçDV&æVDÆö6ÄWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W$†VÆVB"Â6—¦Uööc££ÅÆ–W$†VÆVDWfVçCâ‚’’ÀÐ¢‚$'V–ÆF–æt†VÆVB"Â6—¦Uööc££Ä'V–ÆF–æt†VÆVDWfVçCâ‚’’ÀÐ¢‚$—FVÕ–6·W"Â6—¦Uööc££Ä—FVÕ–6·WWfVçCâ‚’’ÀÐ¢‚$GVVÅ7FGW2"Â6—¦Uööc££ÄGVVÅ7FGW4WfVçCâ‚’’ÀÐ¢‚$f—6„æ÷F–6R"Â6—¦Uööc££Äf—6„æ÷F–6TWfVçCâ‚’’ÀÐ¢‚$f—6„æ÷F–6T&Ò"Â6—¦Uööc££Äf—6„æ÷F–6T&ÔWfVçCâ‚’’ÀÐ¢‚%6Ææ÷F–6R"Â6—¦Uööc££Å6Ææ÷F–6TWfVçCâ‚’’ÀÐ¢‚%F‡&÷v&ÆT†—B"Â6—¦Uööc££ÅF‡&÷v&ÆT†—DWfVçCâ‚’’ÀÐ¢‚%V×¶–äÆ÷&E7VÖÖöæVB"Â6—¦Uööc££ÅV×¶–äÆ÷&E7VÖÖöæVDWfVçCâ‚’’ÀÐ¢‚%V×¶–äÆ÷&D¶–ÆÆVB"Â6—¦Uööc££ÅV×¶–äÆ÷&D¶–ÆÆVDWfVçCâ‚’’ÀÐ¢‚$ÖW&6×W57VÖÖöæVB"Â6—¦Uööc££ÄÖW&6×W57VÖÖöæVDWfVçCâ‚’’ÀÐ¢‚$ÖW&6×W4¶–ÆÆVB"Â6—¦Uööc££ÄÖW&6×W4¶–ÆÆVDWfVçCâ‚’’ÀÐ¢€Ð¢$ÖW&6×W4W66Uv&æ–ær"ÀÐ¢6—¦Uööc££ÄÖW&6×W4W66Uv&æ–ætWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$ÖW&6×W4W66VB"Â6—¦Uööc££ÄÖW&6×W4W66VDWfVçCâ‚’’ÀÐ¢‚$W–V&ÆÄ&÷757VÖÖöæVB"Â6—¦Uööc££ÄW–V&ÆÄ&÷757VÖÖöæVDWfVçCâ‚’’ÀÐ¢‚$W–V&ÆÄ&÷757GVææVB"Â6—¦Uööc££ÄW–V&ÆÄ&÷757GVææVDWfVçCâ‚’’ÀÐ¢‚$W–V&ÆÄ&÷74¶–ÆÆVB"Â6—¦Uööc££ÄW–V&ÆÄ&÷74¶–ÆÆVDWfVçCâ‚’’ÀÐ¢‚$W–V&ÆÄ&÷74¶–ÆÆW""Â6—¦Uööc££ÄW–V&ÆÄ&÷74¶–ÆÆW$WfVçCâ‚’’ÀÐ¢€Ð¢$W–V&ÆÄ&÷74W66T–ÖÖ–æVçB"ÀÐ¢6—¦Uööc££ÄW–V&ÆÄ&÷74W66T–ÖÖ–æVçDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$W–V&ÆÄ&÷74W66VB"Â6—¦Uööc££ÄW–V&ÆÄ&÷74W66VDWfVçCâ‚’’ÀÐ¢‚$ç4‡W'B"Â6—¦Uööc££Äç4‡W'DWfVçCâ‚’’ÀÐ¢€Ð¢$6öçG&öÅö–çEF–ÖW%WFFVB"ÀÐ¢6—¦Uööc££Ä6öçG&öÅö–çEF–ÖW%WFFVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W$†–v„f—fU7F'B"Â6—¦Uööc££ÅÆ–W$†–v„f—fU7F'DWfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W$†–v„f—fT6æ6VÂ"ÀÐ¢6—¦Uööc££ÅÆ–W$†–v„f—fT6æ6VÄWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%Æ–W$†–v„f—fU7V66W72"ÀÐ¢6—¦Uööc££ÅÆ–W$†–v„f—fU7V66W74WfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W$&öçW5ö–çG2"Â6—¦Uööc££ÅÆ–W$&öçW5ö–çG4WfVçCâ‚’’ÀÐ¢‚%Æ–W%Ww&FVB"Â6—¦Uööc££ÅÆ–W%Ww&FVDWfVçCâ‚’’ÀÐ¢‚%Æ–W$'W–&6²"Â6—¦Uööc££ÅÆ–W$'W–&6´WfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W%W6VE÷vW%W&÷GFÆR"ÀÐ¢6—¦Uööc££ÅÆ–W%W6VE÷vW%W&÷GFÆTWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$6‡&—7FÖ4v–gDw&""Â6—¦Uööc££Ä6‡&—7FÖ4v–gDw&$WfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W$¶–ÆÆVD6†–WfVÖVçE¦öæR"ÀÐ¢6—¦Uööc££ÅÆ–W$¶–ÆÆVD6†–WfVÖVçE¦öæTWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%'G•WFFVB"Â6—¦Uööc££Å'G•WFFVDWfVçCâ‚’’ÀÐ¢‚%'G•&Vd6†ævVB"Â6—¦Uööc££Å'G•&Vd6†ævVDWfVçCâ‚’’ÀÐ¢€Ð¢%'G”7&—FW&–6†ævVB"ÀÐ¢6—¦Uööc££Å'G”7&—FW&–6†ævVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%'G”–çf—FW46†ævVB"Â6—¦Uööc££Å'G”–çf—FW46†ævVDWfVçCâ‚’’ÀÐ¢€Ð¢%'G•VWVU7FFT6†ævVB"ÀÐ¢6—¦Uööc££Å'G•VWVU7FFT6†ævVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%'G”6†B"Â6—¦Uööc££Å'G”6†DWfVçCâ‚’’ÀÐ¢‚%'G”ÖVÖ&W$¦ö–â"Â6—¦Uööc££Å'G”ÖVÖ&W$¦ö–äWfVçCâ‚’’ÀÐ¢‚%'G”ÖVÖ&W$ÆVfR"Â6—¦Uööc££Å'G”ÖVÖ&W$ÆVfTWfVçCâ‚’’ÀÐ¢‚$ÖF6„–çf—FW5WFFVB"Â6—¦Uööc££ÄÖF6„–çf—FW5WFFVDWfVçCâ‚’’ÀÐ¢‚$Æö&'•WFFVB"Â6—¦Uööc££ÄÆö&'•WFFVDWfVçCâ‚’’ÀÐ¢‚$×fÔÖ—76–öåWFFR"Â6—¦Uööc££Ä×fÔÖ—76–öåWFFTWfVçCâ‚’’ÀÐ¢‚%&V6Æ7VÆFT†öÆ–F—2"Â6—¦Uööc££Å&V6Æ7VÆFT†öÆ–F—4WfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W$7W'&Væ7”6†ævVB"ÀÐ¢6—¦Uööc££ÅÆ–W$7W'&Væ7”6†ævVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$Föö×6F•&ö6¶WD÷Vâ"Â6—¦Uööc££ÄFöö×6F•&ö6¶WD÷VäWfVçCâ‚’’ÀÐ¢€Ð¢%&VÖ÷fTæVÖW6—5&VÆF–öç6†—2"ÀÐ¢6—¦Uööc££Å&VÖ÷fTæVÖW6—5&VÆF–öç6†—4WfVçCâ‚’ÀÐ¢’ÀÐ¢‚$×fÔ7&VF—D&öçW5vfR"Â6—¦Uööc££Ä×fÔ7&VF—D&öçW5vfTWfVçCâ‚’’ÀÐ¢‚$×fÔ7&VF—D&öçW4ÆÂ"Â6—¦Uööc££Ä×fÔ7&VF—D&öçW4ÆÄWfVçCâ‚’’ÀÐ¢€Ð¢$×fÔ7&VF—D&öçW4ÆÄGfæ6VB"ÀÐ¢6—¦Uööc££Ä×fÔ7&VF—D&öçW4ÆÄGfæ6VDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÕV–6µ6VçG'•Ww&FR"ÀÐ¢6—¦Uööc££Ä×fÕV–6µ6VçG'•Ww&FTWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÕFæ´FW7G&÷–VD'•Æ–W'2"ÀÐ¢6—¦Uööc££Ä×fÕFæ´FW7G&÷–VD'•Æ–W'4WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ""ÀÐ¢6—¦Uööc££Ä×fÔ¶–ÆÅ&ö&÷DFVÆ—fW&–æt&öÖ$WfVçCâ‚’ÀÐ¢’ÀÐ¢‚$×fÕ–6·W7W'&Væ7’"Â6—¦Uööc££Ä×fÕ–6·W7W'&Væ7”WfVçCâ‚’’ÀÐ¢€Ð¢$×fÔ&öÖ$6'&–W$¶–ÆÆVB"ÀÐ¢6—¦Uööc££Ä×fÔ&öÖ$6'&–W$¶–ÆÆVDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÕ6VçG'”'W7FW$FWFöæFR"ÀÐ¢6—¦Uööc££Ä×fÕ6VçG'”'W7FW$FWFöæFTWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÕ66÷WDÖ&¶VDf÷$FVF‚"ÀÐ¢6—¦Uööc££Ä×fÕ66÷WDÖ&¶VDf÷$FVF„WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÔÖVF–5÷vW%W6†&VB"ÀÐ¢6—¦Uööc££Ä×fÔÖVF–5÷vW%W6†&VDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$×fÔ&Vv–åvfR"Â6—¦Uööc££Ä×fÔ&Vv–åvfTWfVçCâ‚’’ÀÐ¢‚$×fÕvfT6ö×ÆWFR"Â6—¦Uööc££Ä×fÕvfT6ö×ÆWFTWfVçCâ‚’’ÀÐ¢‚$×fÔÖ—76–öä6ö×ÆWFR"Â6—¦Uööc££Ä×fÔÖ—76–öä6ö×ÆWFTWfVçCâ‚’’ÀÐ¢€Ð¢$×fÔ&öÖ%&W6WD'•Æ–W""ÀÐ¢6—¦Uööc££Ä×fÔ&öÖ%&W6WD'•Æ–W$WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÔ&öÖ$Æ&ÕG&–vvW&VB"ÀÐ¢6—¦Uööc££Ä×fÔ&öÖ$Æ&ÕG&–vvW&VDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W""ÀÐ¢6—¦Uööc££Ä×fÔ&öÖ$FWÆ÷•&W6WD'•Æ–W$WfVçCâ‚’ÀÐ¢’ÀÐ¢‚$×fÕvfTf–ÆVB"Â6—¦Uööc££Ä×fÕvfTf–ÆVDWfVçCâ‚’’ÀÐ¢‚$×fÕ&W6WE7FG2"Â6—¦Uööc££Ä×fÕ&W6WE7FG4WfVçCâ‚’’ÀÐ¢‚$FÖvU&W6—7FVB"Â6—¦Uööc££ÄFÖvU&W6—7FVDWfVçCâ‚’’ÀÐ¢‚%&Wf—fUÆ–W$æ÷F–g’"Â6—¦Uööc££Å&Wf—fUÆ–W$æ÷F–g”WfVçCâ‚’’ÀÐ¢‚%&Wf—fUÆ–W%7F÷VB"Â6—¦Uööc££Å&Wf—fUÆ–W%7F÷VDWfVçCâ‚’’ÀÐ¢€Ð¢%&Wf—fUÆ–W$6ö×ÆWFR"ÀÐ¢6—¦Uööc££Å&Wf—fUÆ–W$6ö×ÆWFTWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W%GW&æVEFôv†÷7B"Â6—¦Uööc££ÅÆ–W%GW&æVEFôv†÷7DWfVçCâ‚’’ÀÐ¢€Ð¢$ÖVF–wVå6†–VÆD&Æö6¶VDFÖvR"ÀÐ¢6—¦Uööc££ÄÖVF–wVå6†–VÆD&Æö6¶VDFÖvTWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÔGevfT6ö×ÆWFTæôvFW2"ÀÐ¢6—¦Uööc££Ä×fÔGevfT6ö×ÆWFTæôvFW4WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÕ6æ—W$†VG6†÷D7W'&Væ7’"ÀÐ¢6—¦Uööc££Ä×fÕ6æ—W$†VG6†÷D7W'&Væ7”WfVçCâ‚’ÀÐ¢’ÀÐ¢‚$×fÔÖææ†GFå—B"Â6—¦Uööc££Ä×fÔÖææ†GFå—DWfVçCâ‚’’ÀÐ¢€Ð¢$fÆt6'&–VD–äFWFV7F–öå¦öæR"ÀÐ¢6—¦Uööc££ÄfÆt6'&–VD–äFWFV7F–öå¦öæTWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$×fÔGevfT¶–ÆÆVE7GVå&F–ò"ÀÐ¢6—¦Uööc££Ä×fÔGevfT¶–ÆÆVE7GVå&F–ôWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W$F—&V7D†—E7GVâ"Â6—¦Uööc££ÅÆ–W$F—&V7D†—E7GVäWfVçCâ‚’’ÀÐ¢€Ð¢$×fÕ6VçG'”'W7FW$¶–ÆÆVB"ÀÐ¢6—¦Uööc££Ä×fÕ6VçG'”'W7FW$¶–ÆÆVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Ww&FW4f–ÆT6†ævVB"Â6—¦Uööc££ÅWw&FW4f–ÆT6†ævVDWfVçCâ‚’’ÀÐ¢‚%&EFVÕö–çG46†ævVB"Â6—¦Uööc££Å&EFVÕö–çG46†ævVDWfVçCâ‚’’ÀÐ¢‚%&E'VÆW57FFT6†ævVB"Â6—¦Uööc££Å&E'VÆW57FFT6†ævVDWfVçCâ‚’’ÀÐ¢‚%&E&ö&÷D¶–ÆÆVB"Â6—¦Uööc££Å&E&ö&÷D¶–ÆÆVDWfVçCâ‚’’ÀÐ¢‚%&E&ö&÷D–×7B"Â6—¦Uööc££Å&E&ö&÷D–×7DWfVçCâ‚’’ÀÐ¢€Ð¢%FVÕÆ•&U&÷VæEF–ÖTÆVgB"ÀÐ¢6—¦Uööc££ÅFVÕÆ•&U&÷VæEF–ÖTÆVgDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%&6‡WFTFWÆ÷’"Â6—¦Uööc££Å&6‡WFTFWÆ÷”WfVçCâ‚’’ÀÐ¢‚%&6‡WFT†öÇ7FW""Â6—¦Uööc££Å&6‡WFT†öÇ7FW$WfVçCâ‚’’ÀÐ¢‚$¶–ÆÅ&Vf–ÆÇ4ÖWFW""Â6—¦Uööc££Ä¶–ÆÅ&Vf–ÆÇ4ÖWFW$WfVçCâ‚’’ÀÐ¢‚%'5FVçDWfVçB"Â6—¦Uööc££Å'5FVçDWfVçDWfVçCâ‚’’ÀÐ¢‚$6öæv¶–ÆÂ"Â6—¦Uööc££Ä6öæv¶–ÆÄWfVçCâ‚’’ÀÐ¢‚%Æ–W$–æ—F–Å7vâ"Â6—¦Uööc££ÅÆ–W$–æ—F–Å7väWfVçCâ‚’’ÀÐ¢‚$6ö×WF—F—fUf–7F÷'’"Â6—¦Uööc££Ä6ö×WF—F—fUf–7F÷'”WfVçCâ‚’’ÀÐ¢€Ð¢$6ö×WF—F—fU7FG5WFFR"ÀÐ¢6—¦Uööc££Ä6ö×WF—F—fU7FG5WFFTWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$Ö–æ”vÖUv–â"Â6—¦Uööc££ÄÖ–æ”vÖUv–äWfVçCâ‚’’ÀÐ¢‚%6VçG'”öävô7F—fR"Â6—¦Uööc££Å6VçG'”öävô7F—fTWfVçCâ‚’’ÀÐ¢‚$GV6µ‡ÆWfVÅW"Â6—¦Uööc££ÄGV6µ‡ÆWfVÅWWfVçCâ‚’’ÀÐ¢‚%VW7DÆöt÷VæVB"Â6—¦Uööc££ÅVW7DÆöt÷VæVDWfVçCâ‚’’ÀÐ¢‚%66†VÖWFFVB"Â6—¦Uööc££Å66†VÖWFFVDWfVçCâ‚’’ÀÐ¢€Ð¢$Æö6ÅÆ–W%–6·WvVöâ"ÀÐ¢6—¦Uööc££ÄÆö6ÅÆ–W%–6·WvVöäWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%&EÆ–W%66÷&Uö–çG2"Â6—¦Uööc££Å&EÆ–W%66÷&Uö–çG4WfVçCâ‚’’ÀÐ¢‚$FVÖöÖäFWE7F–6¶–W2"Â6—¦Uööc££ÄFVÖöÖäFWE7F–6¶–W4WfVçCâ‚’’ÀÐ¢€Ð¢%VW7Dö&¦V7F—fT6ö×ÆWFVB"ÀÐ¢6—¦Uööc££ÅVW7Dö&¦V7F—fT6ö×ÆWFVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W%66÷&T6†ævVB"Â6—¦Uööc££ÅÆ–W%66÷&T6†ævVDWfVçCâ‚’’ÀÐ¢‚$¶–ÆÆVD6–æuÆ–W""Â6—¦Uööc££Ä¶–ÆÆVD6–æuÆ–W$WfVçCâ‚’’ÀÐ¢‚$Vçf—&öæÖVçFÄFVF‚"Â6—¦Uööc££ÄVçf—&öæÖVçFÄFVF„WfVçCâ‚’’ÀÐ¢‚%&ö¦V7F–ÆTF—&V7D†—B"Â6—¦Uööc££Å&ö¦V7F–ÆTF—&V7D†—DWfVçCâ‚’’ÀÐ¢‚%74vWB"Â6—¦Uööc££Å74vWDWfVçCâ‚’’ÀÐ¢‚%7566÷&R"Â6—¦Uööc££Å7566÷&TWfVçCâ‚’’ÀÐ¢‚%74g&VR"Â6—¦Uööc££Å74g&VTWfVçCâ‚’’ÀÐ¢‚%75746Vv‡B"Â6—¦Uööc££Å75746Vv‡DWfVçCâ‚’’ÀÐ¢‚%74&ÆÅ7FöÆVâ"Â6—¦Uööc££Å74&ÆÅ7FöÆVäWfVçCâ‚’’ÀÐ¢‚%74&ÆÄ&Æö6¶VB"Â6—¦Uööc££Å74&ÆÄ&Æö6¶VDWfVçCâ‚’’ÀÐ¢‚$FÖvU&WfVçFVB"Â6—¦Uööc££ÄFÖvU&WfVçFVDWfVçCâ‚’’ÀÐ¢‚$†ÆÆ÷vVVä&÷74¶–ÆÆVB"Â6—¦Uööc££Ä†ÆÆ÷vVVä&÷74¶–ÆÆVDWfVçCâ‚’’ÀÐ¢‚$W66VDÆö÷D—6ÆæB"Â6—¦Uööc££ÄW66VDÆö÷D—6ÆæDWfVçCâ‚’’ÀÐ¢‚%FvvVEÆ–W$4—B"Â6—¦Uööc££ÅFvvVEÆ–W$4—DWfVçCâ‚’’ÀÐ¢‚$ÖW&6×W57GVææVB"Â6—¦Uööc££ÄÖW&6×W57GVææVDWfVçCâ‚’’ÀÐ¢‚$ÖW&6×W5&÷f÷VæB"Â6—¦Uööc££ÄÖW&6×W5&÷f÷VæDWfVçCâ‚’’ÀÐ¢€Ð¢$†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVB"ÀÐ¢6—¦Uööc££Ä†ÆÆ÷vVVå6¶VÆWFöä¶–ÆÆVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%6¶VÆWFöä¶–ÆÆVEVW7B"Â6—¦Uööc££Å6¶VÆWFöä¶–ÆÆVEVW7DWfVçCâ‚’’ÀÐ¢€Ð¢%6¶VÆWFöä¶–æt¶–ÆÆVEVW7B"ÀÐ¢6—¦Uööc££Å6¶VÆWFöä¶–æt¶–ÆÆVEVW7DWfVçCâ‚’ÀÐ¢’ÀÐ¢‚$W66T†VÆÂ"Â6—¦Uööc££ÄW66T†VÆÄWfVçCâ‚’’ÀÐ¢‚$7&÷757V7G&Ä'&–FvR"Â6—¦Uööc££Ä7&÷757V7G&Ä'&–FvTWfVçCâ‚’’ÀÐ¢‚$Ö–æ”vÖUvöâ"Â6—¦Uööc££ÄÖ–æ”vÖUvöäWfVçCâ‚’’ÀÐ¢‚%&W7väv†÷7B"Â6—¦Uööc££Å&W7väv†÷7DWfVçCâ‚’’ÀÐ¢‚$¶–ÆÄ–ä†VÆÂ"Â6—¦Uööc££Ä¶–ÆÄ–ä†VÆÄWfVçCâ‚’’ÀÐ¢€Ð¢$†ÆÆ÷vVVäGV6´6öÆÆV7FVB"ÀÐ¢6—¦Uööc££Ä†ÆÆ÷vVVäGV6´6öÆÆV7FVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%7V6–Å66÷&R"Â6—¦Uööc££Å7V6–Å66÷&TWfVçCâ‚’’ÀÐ¢‚%FVÔÆVFW$¶–ÆÆVB"Â6—¦Uööc££ÅFVÔÆVFW$¶–ÆÆVDWfVçCâ‚’’ÀÐ¢€Ð¢$†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVB"ÀÐ¢6—¦Uööc££Ä†ÆÆ÷vVVå6÷VÄ6öÆÆV7FVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%&V6Æ7VÆFUG'V6R"Â6—¦Uööc££Å&V6Æ7VÆFUG'V6TWfVçCâ‚’’ÀÐ¢€Ð¢$FVE&–ævW$6†VDFVF‚"ÀÐ¢6—¦Uööc££ÄFVE&–ævW$6†VDFVF„WfVçCâ‚’ÀÐ¢’ÀÐ¢‚$7&÷76&÷t†VÂ"Â6—¦Uööc££Ä7&÷76&÷t†VÄWfVçCâ‚’’ÀÐ¢‚$FÖvTÖ—F–vFVB"Â6—¦Uööc££ÄFÖvTÖ—F–vFVDWfVçCâ‚’’ÀÐ¢‚%–ÆöEW6†VB"Â6—¦Uööc££Å–ÆöEW6†VDWfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W$&æFöæVDÖF6‚"ÀÐ¢6—¦Uööc££ÅÆ–W$&æFöæVDÖF6„WfVçCâ‚’ÀÐ¢’ÀÐ¢‚$6ÄG&vÆ–æR"Â6—¦Uööc££Ä6ÄG&vÆ–æTWfVçCâ‚’’ÀÐ¢‚%&W7F'EF–ÖW%F–ÖR"Â6—¦Uööc££Å&W7F'EF–ÖW%F–ÖTWfVçCâ‚’’ÀÐ¢‚%v–äÆ–Ö—D6†ævVB"Â6—¦Uööc££Åv–äÆ–Ö—D6†ævVDWfVçCâ‚’’ÀÐ¢‚%v–åæVÅ6†÷u66÷&W2"Â6—¦Uööc££Åv–åæVÅ6†÷u66÷&W4WfVçCâ‚’’ÀÐ¢€Ð¢%F÷7G&V×5&WVW7Df–æ—6†VB"ÀÐ¢6—¦Uööc££ÅF÷7G&V×5&WVW7Df–æ—6†VDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$6ö×WF—F—fU7FFT6†ævVB"ÀÐ¢6—¦Uööc££Ä6ö×WF—F—fU7FFT6†ævVDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢$vÆö&Åv$FFWFFVB"ÀÐ¢6—¦Uööc££ÄvÆö&Åv$FFWFFVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%7F÷vF6„6†ævVB"Â6—¦Uööc££Å7F÷vF6„6†ævVDWfVçCâ‚’’ÀÐ¢‚$G57F÷"Â6—¦Uööc££ÄG57F÷WfVçCâ‚’’ÀÐ¢‚$G567&VVç6†÷B"Â6—¦Uööc££ÄG567&VVç6†÷DWfVçCâ‚’’ÀÐ¢‚%6†÷tÖF6…7VÖÖ'’"Â6—¦Uööc££Å6†÷tÖF6…7VÖÖ'”WfVçCâ‚’’ÀÐ¢‚$W‡W&–Væ6T6†ævVB"Â6—¦Uööc££ÄW‡W&–Væ6T6†ævVDWfVçCâ‚’’ÀÐ¢‚$&Vv–å‡ÆW'"Â6—¦Uööc££Ä&Vv–å‡ÆW'WfVçCâ‚’’ÀÐ¢€Ð¢$ÖF6†Ö¶W%7FG5WFFVB"ÀÐ¢6—¦Uööc££ÄÖF6†Ö¶W%7FG5WFFVDWfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%&VÖF6…f÷FUW&–öD÷fW""ÀÐ¢6—¦Uööc££Å&VÖF6…f÷FUW&–öD÷fW$WfVçCâ‚’ÀÐ¢’ÀÐ¢€Ð¢%&VÖF6„f–ÆVEFô7&VFR"ÀÐ¢6—¦Uööc££Å&VÖF6„f–ÆVEFô7&VFTWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%Æ–W%&VÖF6„6†ævR"Â6—¦Uööc££ÅÆ–W%&VÖF6„6†ævTWfVçCâ‚’’ÀÐ¢‚%–æuWFFVB"Â6—¦Uööc££Å–æuWFFVDWfVçCâ‚’’ÀÐ¢‚$ÔÕ7FG5WFFVB"Â6—¦Uööc££ÄÔÕ7FG5WFFVDWfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W$æW‡DÖf÷FT6†ævR"ÀÐ¢6—¦Uööc££ÅÆ–W$æW‡DÖf÷FT6†ævTWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%f÷FTÖ46†ævVB"Â6—¦Uööc££Åf÷FTÖ46†ævVDWfVçCâ‚’’ÀÐ¢‚%&÷FôFVd6†ævVB"Â6—¦Uööc££Å&÷FôFVd6†ævVDWfVçCâ‚’’ÀÐ¢‚%Æ–W$FöÖ–æF–öâ"Â6—¦Uööc££ÅÆ–W$FöÖ–æF–öäWfVçCâ‚’’ÀÐ¢€Ð¢%Æ–W%&ö6¶WE6µW6†VB"ÀÐ¢6—¦Uööc££ÅÆ–W%&ö6¶WE6µW6†VDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%VW7E&WVW7B"Â6—¦Uööc££ÅVW7E&WVW7DWfVçCâ‚’’ÀÐ¢‚%VW7E&W7öç6R"Â6—¦Uööc££ÅVW7E&W7öç6TWfVçCâ‚’’ÀÐ¢‚%VW7E&öw&W72"Â6—¦Uööc££ÅVW7E&öw&W74WfVçCâ‚’’ÀÐ¢‚%&ö¦V7F–ÆU&VÖ÷fVB"Â6—¦Uööc££Å&ö¦V7F–ÆU&VÖ÷fVDWfVçCâ‚’’ÀÐ¢‚%VW7DÖFF6†ævVB"Â6—¦Uööc££ÅVW7DÖFF6†ævVDWfVçCâ‚’’ÀÐ¢€Ð¢$v4F÷W6VEÆ–W$–væ—FVB"ÀÐ¢6—¦Uööc££Äv4F÷W6VEÆ–W$–væ—FVDWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%VW7EGW&ä–å7FFR"Â6—¦Uööc££ÅVW7EGW&ä–å7FFTWfVçCâ‚’’ÀÐ¢‚$—FV×46¶æ÷vÆVFvVB"Â6—¦Uööc££Ä—FV×46¶æ÷vÆVFvVDWfVçCâ‚’’ÀÐ¢‚$6W$¶–ÆÆVB"Â6—¦Uööc££Ä6W$¶–ÆÆVDWfVçCâ‚’’ÀÐ¢‚$Ö–äÖVçU7F&–Æ—¦VB"Â6—¦Uööc££ÄÖ–äÖVçU7F&–Æ—¦VDWfVçCâ‚’’ÀÐ¢‚%v÷&ÆE7FGW46†ævVB"Â6—¦Uööc££Åv÷&ÆE7FGW46†ævVDWfVçCâ‚’’ÀÐ¢‚$„ÅEe7FGW2"Â6—¦Uööc££Ä„ÅEe7FGW4WfVçCâ‚’’ÀÐ¢‚$„ÅEd6ÖW&Öâ"Â6—¦Uööc££Ä„ÅEd6ÖW&ÖäWfVçCâ‚’’ÀÐ¢‚$„ÅEe&æ´6ÖW&"Â6—¦Uööc££Ä„ÅEe&æ´6ÖW&WfVçCâ‚’’ÀÐ¢‚$„ÅEe&æ´VçF—G’"Â6—¦Uööc££Ä„ÅEe&æ´VçF—G”WfVçCâ‚’’ÀÐ¢‚$„ÅEdf—†VB"Â6—¦Uööc££Ä„ÅEdf—†VDWfVçCâ‚’’ÀÐ¢‚$„ÅEd6†6R"Â6—¦Uööc££Ä„ÅEd6†6TWfVçCâ‚’’ÀÐ¢‚$„ÅEdÖW76vR"Â6—¦Uööc££Ä„ÅEdÖW76vTWfVçCâ‚’’ÀÐ¢‚$„ÅEeF—FÆR"Â6—¦Uööc££Ä„ÅEeF—FÆTWfVçCâ‚’’ÀÐ¢‚$„ÅEd6†B"Â6—¦Uööc££Ä„ÅEd6†DWfVçCâ‚’’ÀÐ¢‚%&WÆ•7F'E&V6÷&B"Â6—¦Uööc££Å&WÆ•7F'E&V6÷&DWfVçCâ‚’’ÀÐ¢‚%&WÆ•6W76–öä–æfò"Â6—¦Uööc££Å&WÆ•6W76–öä–æfôWfVçCâ‚’’ÀÐ¢‚%&WÆ”VæE&V6÷&B"Â6—¦Uööc££Å&WÆ”VæE&V6÷&DWfVçCâ‚’’ÀÐ¢€Ð¢%&WÆ•&WÆ—4f–Æ&ÆR"ÀÐ¢6—¦Uööc££Å&WÆ•&WÆ—4f–Æ&ÆTWfVçCâ‚’ÀÐ¢’ÀÐ¢‚%&WÆ•6W'fW$W'&÷""Â6—¦Uööc££Å&WÆ•6W'fW$W'&÷$WfVçCâ‚’’ÀÐ¢ÐÐ¢æ—FW"‚Ð¢æ6÷–VB‚Ð¢æ6öÆÆV7B‚Ð§ÐÐ