use std::{env, io};
use std::fs::{File, read_dir};
use std::path::PathBuf;
use uuid::Uuid;
use crate::client::Client;

pub struct Node;

impl Node {
    /// Fetch all sockets located on the file provided
    /// by the environnement variable BLOCKSHAIN_SOCKETS_LOCATION.
    pub fn get_nodes(blockchain_sockets_location: String, _for: &Client) -> io::Result<Vec<PathBuf>> {
        return read_dir(blockchain_sockets_location)?
            .map(|res| res.map(|e| {
                let path = e.path();
                if path.as_path().display().to_string() != _for.client_socket_path  {
                    path
                } else {
                    PathBuf::default()
                }
            }))
            .collect::<Result<Vec<PathBuf>, io::Error>>();
    }
}
