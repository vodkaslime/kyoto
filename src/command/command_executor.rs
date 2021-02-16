use crate::command::command_table::Command;
use crate::data::Server;

use bytes::Bytes;

pub struct CommandExecutor { }

impl CommandExecutor {
    pub fn execute_command(cmd: Command, server: &mut Server) -> crate::Result<Bytes> {
        match cmd {
            Command::Get { key } => {
                match server.get_db().get(&key) {
                    Some(res) => {
                        Ok(res)
                    },
                    None => {
                        Ok("Key not found.".into())
                    }
                }
            },
            Command::Set { key, value } => {
                server.get_db().set(&key, value)?;
                Ok("OK.".into())
            }
        }
    }
}