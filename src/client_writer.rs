use std::sync::Arc;
use crate::client::Client;
use crate::block::Block;
use crate::network::signal::Signal;
use std::io;
use crate::hash::BlockHash;
use std::os::unix::net::UnixStream;
use std::io::{Write, Read};

pub trait UnixSocketWriter {
    /// Handle communication with other nodes
    fn respond_to_node(&self, signal: Signal) -> io::Result<()>;

    /// Send a message to a specific node
    fn send_to_node(&self, to: String, message: Signal) -> io::Result<()>;

    /// Send a block on the network by given a recipient and some data.
    fn send_block(&self, to: &Client, data: &str) -> io::Result<()>;
}

pub struct ClientWriter {
    client: Arc<Client>
}

impl ClientWriter {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client
        }
    }
}


impl UnixSocketWriter for ClientWriter {
    fn respond_to_node(&self, signal: Signal) -> io::Result<()> {
        let listener = &self.client.client_socket.lock().unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking");

        for mut stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                stream.set_nonblocking(true).expect("Couldn't set nonblocking");
                let mut buffer = String::new();

                if let Ok(_) = stream.read_to_string(&mut buffer) {
                    println!("receive : {}", buffer);
                    let received_signal: Signal = serde_json::from_str(buffer.as_str()).unwrap();
                    self.send_to_node(received_signal.from_socket_path, Signal::is_okay(&self.client.clone(), true)).unwrap();
                }
            }

            break;
        }

        Ok(())
    }

    fn send_to_node(&self, to: String, message: Signal) -> io::Result<()> {
        let mut streamer = UnixStream::connect(to)?;
        streamer.set_nonblocking(true).expect("Couldn't set nonblocking");
        let message_as_string = message.to_string().unwrap();
        streamer.write_all(message_as_string.as_bytes())?;

        Ok(())
    }

    fn send_block(&self, to: &Client, data: &str) -> io::Result<()> {
        let block = Block::new(self.client.client_socket_id, data, BlockHash::default());
        let message = Signal::add_a_block(&*self.client.clone(), block.clone());
        &self.send_to_node(to.client_socket_path.clone(), message)?;
        Ok(())
    }
}