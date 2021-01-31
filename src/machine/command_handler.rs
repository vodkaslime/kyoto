use crate::data_structure::db::Db;
use crate::machine::command::Command;
use crate::machine::command_parser::CommandParser;

use bytes::{ Bytes, BytesMut };

#[derive(Debug)]
pub struct CommandHandler {
    db: Db,
}

impl CommandHandler {
    pub fn new(db: Db) -> Self {
        CommandHandler {
            db: db,
        }
    }

    pub fn handle_buffer(&mut self, buffer: BytesMut) -> crate::Result<Option<Bytes>> {
        match CommandParser::parse_command(buffer) {
            Ok(option) => {
                match option {
                    Some(command) => {
                        Ok(self.execute_command(command).unwrap().into())
                    },
                    None => {
                        Ok(Some(Bytes::from("parsing")))
                    }
                }
            },
            Err(err) => {
                Err(err.into())
            }
        }
        
    }

    pub fn execute_command(&mut self, command: Command) -> crate::Result<Bytes> {
        match command {
            Command::Get { key } => {
                Ok(Bytes::from("Get command"))
            },
            Command::Set { key, value } => {
                Ok(Bytes::from("Not supported yet"))
            }
        }
    }
}