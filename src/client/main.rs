extern crate tasdcailloux;
extern crate nanomsg;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

use nanomsg::{Socket, Protocol};
use bincode::{serialize, deserialize, Infinite};
use tasdcailloux::models::*;
use std::io::{Read, Write};

fn main() {
    let mut socket_push = Socket::new(Protocol::Push).unwrap();
    socket_push.connect(&"tcp://127.0.0.1:5555").expect("Fail to bind to tcp port");

    let message = Message{ message_type: MessageType::GetRange{from: 0, to: 10} };
    let encoded: Vec<u8> = serialize(&message, Infinite).unwrap();
    socket_push.write(&encoded).unwrap();
}
