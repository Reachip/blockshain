#![allow(dead_code, unused_mut, unused_imports, unused_variables)]

use std::{env, thread, time, io::Stdin, io::{self, Read}};
use std::sync::{Arc, Mutex, RwLock};

use block::Block;
use cli::CLI;
use client::Client;
use crate::network::signal::Signal;
use std::path::PathBuf;
use std::borrow::Borrow;

mod block;
mod client;
mod network;
mod cli;
mod hash;

fn main() {
    /*let clients_sockets_location = match env::var("BLOCKSHAIN_SOCKETS_LOCATION") {
        Ok(var) => var,
        Err(_) => {
            panic!("BLOCKSHAIN_SOCKETS_LOCATION didn't provided so we use /tmp location, make sure that you have permission");
        }
    };*/


    let clients_sockets_location = "/home/rached/bs/".to_owned();
    let client = Arc::new(Client::new(clients_sockets_location.clone()).unwrap());
    let client_responder = client.clone();
    let client_reader = client.clone();

    let client_reader_task = thread::spawn(move || {
        loop {
            client_reader.fetch_blocks(clients_sockets_location.clone()).unwrap();
            thread::sleep(time::Duration::from_nanos(1));
        }
    });

    let client_responder_task = thread::spawn(move || {
        loop {    
            client_responder.respond_to_node(Signal::is_okay(&(*client_responder), true)).unwrap();
            thread::sleep(time::Duration::from_nanos(1));
        }
    });

   
    client_reader_task.join().unwrap();
    client_responder_task.join().unwrap();
}