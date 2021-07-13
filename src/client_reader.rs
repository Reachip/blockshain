use std::sync::Arc;
use crate::client::Client;
use std::io;
use std::io::{Error, Write, Read};
use crate::network::node::Node;
use std::os::unix::net::UnixStream;
use crate::network::signal::Signal;

pub trait UnixSocketReader {
    /// Fetch blocks provided by the network.
    fn fetch_blocks(&self, clients_sockets_location: String) -> Result<(), io::Error>;
}

pub struct ClientReader {
    client: Arc<Client>
}

impl ClientReader {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client
        }
    }
}

impl UnixSocketReader for ClientReader {
    fn fetch_blocks(&self, clients_sockets_location: String) -> Result<(), Error> {
        let mut socket_connections: usize = 0;
        let nodes = Node::get_nodes(clients_sockets_location, &self.client.clone()).unwrap();

        for node in nodes {
            if let Ok(stream) = UnixStream::connect(&node) {
                println!("comm with {:?}", node);
                socket_connections += 1;
                let mut streamer: UnixStream = stream;
                streamer.set_nonblocking(true).expect("Couldn't set nonblocking");

                if self.client.send_local_blockchain().is_empty() {
                    let signal = Signal::new_miner(&self.client.clone()).to_string()?;

                    if let Ok(writer) = streamer.write_all(signal.as_bytes()) {
                        let mut response = String::new();

                        if let Ok(_) = streamer.read_to_string(&mut response) {
                            println!("Receive : {}", response);
                        }
                    } else {
                        println!("Receive anything");
                    }
                }
            }
        }

        if socket_connections < 0 {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionRefused,
                "Connnection refused for all of the nodes in the network",
            ));

        }

        return Ok(());
    }
}