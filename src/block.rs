extern crate serde;
extern crate sha2;
extern crate uuid;

use std::cell::RefCell;
use std::str;

use serde::ser::{Serialize, Serializer, SerializeStruct};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::hash::BlockHash;

#[derive(Clone, Debug)]
pub struct Block {
    previous_block_hash: BlockHash,
    data: String,
    signature: String,
    pub proof_of_work: RefCell<String>,
    pub hash: RefCell<BlockHash>,
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_struct("Block", 5)?;

        s.serialize_field("previous_block_hash", &self.previous_block_hash)?;
        s.serialize_field("data", &self.data)?;
        s.serialize_field("signature", &self.signature)?;
        s.serialize_field("proof_of_work", &self.proof_of_work)?;
        s.serialize_field("hash", &self.hash)?;
        s.end()
    }
}


impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.hash.borrow().get_hash_as_string() == other.hash.borrow().get_hash_as_string()
    }
}

impl Eq for Block {}

impl Block {
    /// Create a new block by a given author and some data.
    pub fn new(author: Uuid, data: &str, previous_block_hash: BlockHash) -> Block {
        let mut block = Block {
            previous_block_hash,
            data: data.to_owned(),
            signature: author.to_string(),
            proof_of_work: RefCell::new(Uuid::new_v4().to_string()),
            hash: RefCell::new(BlockHash::default()),
        };

        *block.hash.borrow_mut() = BlockHash::new(data.to_owned().clone(), block.signature.clone(), block.proof_of_work.clone());
        block
    }

    /// Check if the prof of work is finish.
    /// The prof of work is finish when the hash of the
    /// block starts with two zero.
    pub fn proof_of_work_is_finish(&self) -> bool {
        let hash = &self.hash.borrow().get_hash_as_string();
        hash.starts_with("00")
    }

    /// Updating the proof_of_work field and thereby the hash field.
    pub fn update_proof_of_work(&self) {
        *self.proof_of_work.borrow_mut() = Uuid::new_v4().to_string();
        *self.hash.borrow_mut() = BlockHash::new(
            self.data.clone(),
            self.signature.clone(),
            self.proof_of_work.clone(),
        );
    }
}
