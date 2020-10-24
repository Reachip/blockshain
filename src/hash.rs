extern crate sha2;

use std::cell::RefCell;
use serde::{Serialize, Serializer};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use serde::ser::SerializeStruct;

#[derive(Clone, Debug, Default)]
pub struct BlockHash {
    data: String,
    signature: String,
    proof_of_work: RefCell<String>,
}

impl Serialize for BlockHash {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct("BlockHash", 1)?;
        s.serialize_field("hash", &self.get_hash_as_string())?;
        s.end()
    }
}

impl BlockHash {
    pub fn new(data: String, signature: String, proof_of_work: RefCell<String>) -> Self {
        Self {
            data,
            signature,
            proof_of_work,
        }
    }

    pub fn get_hash(&self) -> Sha256 {
        let mut hash = Sha256::default();
        hash.input(
            format!(
                "{}{}{}",
                &self.data,
                &self.signature,
                *&self.proof_of_work.borrow())
        );

        hash
    }

    pub fn get_hash_as_string(&self) -> String {
        let hash = &self.get_hash();

        // Return hash as string
        hash.clone().result().as_slice().iter().map(|&c| c as char).collect::<String>()
    }
}