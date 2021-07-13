#![allow(dead_code, unused_mut, unused_imports, unused_variables)]

use std::{env, thread, time, io::Stdin, io::{self, Read}};
use std::sync::{Arc, Mutex, RwLock};

use block::Block;
use cli::CLI;
use client::Client;
use crate::network::signal::Signal;
use std::path::PathBuf;
use std::borrow::Borrow;
use crate::client_reader::UnixSocketReader;

mod block;
mod client;
mod network;
mod cli;
mod hash;
mod client_reader;
mod client_writer;

use client_reader::ClientReader;
use client_writer::ClientWriter;
use crate::client_writer::UnixSocketWriter;

fn main() {
    let clients_sockets_location = match env::var("BLOCKSHAIN_SOCKETS_LOCATION") {
        Ok(var) => var,
        Err(_) => {
            panic!("BLOCKSHAIN_SOCKETS_LOCATION didn't provided so we use /tmp location, make sure that you have permission");
        }
    };

    let client = Arc::new(Client::new(clients_sockets_location.clone()).unwrap());

    let client_responder = ClientWriter::new(client.clone());
    let client_reader = ClientReader::new(client.clone());

    println!("{} created",  client.client_socket_path);

    let client_reader_task = thread::spawn(move || {
        loop {
            client_reader.fetch_blocks(clients_sockets_location.clone()).unwrap();
            thread::sleep(time::Duration::from_nanos(1));
        }
    });

    let client_responder_task = thread::spawn(move || {
        loop {
            client_responder.respond_to_node(Signal::is_okay(&*client.clone(), true)).unwrap();
            thread::sleep(time::Duration::from_nanos(1));
        }
    });

    client_reader_task.join().unwrap();
    client_responder_task.join().unwrap();
}