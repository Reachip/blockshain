use serde::{Deserialize, Serialize};

use crate::block::Block;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Key {
    IsOkay,
    AddBlock,
    IsThisBlockIsConform,
    FinishedMining,
    NewMiner,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Value {
    Message(String),
    BoolMessage(bool),
    Block(Block),
    Nothing,
}


#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    pub body: Key,
    pub message: Value,
}

impl Message {
    pub fn new(body: Key, message: Value) -> Self {
        Message {
            body,
            message,
        }
    }
}
