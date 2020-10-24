use std::{env, io};
use std::fs::{File, read_dir};
use std::path::PathBuf;
use uuid::Uuid;

pub struct Node;

impl Node {
    /// Fetch all sockets located on the file provided
    /// by the environnement variable BLOCKSHAIN_SOCKETS_LOCATION.
    pub fn get_nodes(blockchain_sockets_location: String) -> io::Result<Vec<PathBuf>> {
        return read_dir(blockchain_sockets_location)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>();
    }
}
