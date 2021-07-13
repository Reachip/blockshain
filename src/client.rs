extern crate sha2;
extern crate uuid;

use std::{
    cell::RefCell,
    error::Error,
    fs::{File, read_dir, remove_file},
    io::prelude::*,
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
    rc::Rc,
};
use std::{env, io};

use serde::{Deserialize, Serialize};
use sha2::digest::generic_array::typenum::UInt;
use uuid::Uuid;

use crate::{block::Block};
use crate::hash::BlockHash;
use crate::network::{node::Node, signal::Signal};
use std::sync::{Mutex};

/// Represent a simple client on the blockchain network.
/// It can be a miner or a data provider.
pub struct Client {
    /// A unique ID for a client.
    pub client_socket_id: Uuid,
    /// TODO
    pub client_socket_path: String,
    /// The socket assigned for a client.
    pub client_socket: Mutex<UnixListener>,
    /// A vector which store all of the blocks provided by the network.
    blockchain: Mutex<Vec<Block>>,
}

impl Drop for Client {
    fn drop(&mut self) {
        remove_file(&self.client_socket_path).unwrap();
    }
}

impl Client {
    /// Create a new client.
    /// It create also a new socket assigned for the client.
    pub fn new(clients_sockets_location: String) -> io::Result<Client> {
        let client_socket_id = Uuid::new_v4();
        let client_socket_path = format!("{}{}.sock", clients_sockets_location, client_socket_id);
        let client = Client {
            client_socket_id,
            client_socket_path: client_socket_path.clone(),
            client_socket: Mutex::new(UnixListener::bind(client_socket_path)?),
            blockchain: Mutex::new(vec![]),
        };

        Ok(client)
    }

    /// Update "blockchain" field by given a new block.
    pub fn update_local_chain(&self, new_block: Block) {
        &self.blockchain.lock().unwrap().push(new_block);
    }

    /// Mine a block by changing the prof of work to find
    /// two zero in the current hash (BlockHash) of  the Block
    pub fn mine_block(&self, block: Block) {

    }

    /// Return the local blockchain on a vector
    pub fn send_local_blockchain(&self) -> Vec<Block> {
        let blockchain = &self.blockchain.lock().unwrap();
        blockchain.to_vec()
    }

    /// Check if a given block is valid
    pub fn is_a_valid_block(&self, block: Block) -> bool {
        true
    }
}
