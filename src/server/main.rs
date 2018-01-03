#![recursion_limit="128"]
#![feature(plugin)]
#![feature(plugin, custom_derive)]

extern crate tasdcailloux;

#[macro_use]extern crate lazy_static;

#[macro_use]extern crate diesel;
#[macro_use]extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate chrono;
extern crate dotenv;
extern crate nanomsg;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

pub mod db;
pub mod controls;

use nanomsg::{Socket, Protocol};
use bincode::{serialize, deserialize, Infinite};
use tasdcailloux::models::{Message, MessageType, Response, ResponseType};
use db::get_db;
use std::io::{Read, Write};

fn main() {
    let mut socket = Socket::new(Protocol::Rep).unwrap();
    socket.bind(&"tcp://127.0.0.1:5555").expect("Fail to bind to tcp port");

    loop {
        let mut msg = Vec::new();
        socket.read_to_end(&mut msg).unwrap();
        let decoded: Message = deserialize(&msg).unwrap();
        match decoded.message_type {
            MessageType::IsReady => {
                let encoded: Vec<u8> = serialize(&MessageType::IsReady, Infinite).unwrap();
                socket.write(&encoded).unwrap();

            },
            MessageType::GetOne{id} => {
                let element = controls::mineral::get_mineral(get_db().conn(), id);
                let response = Response{response_type : ResponseType::GetOne{ data: element}};
                let encoded: Vec<u8> = serialize(&response, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetRange{from, to} => {
                let elements = controls::mineral::get_mineral_range(get_db().conn(), from, to);
                let response = Response{response_type : ResponseType::GetRange{ data: elements}};
                let encoded: Vec<u8> = serialize(&response, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetAll => {
                let elements = controls::mineral::get_mineral_all(get_db().conn());
                let response = Response{response_type : ResponseType::GetAll{ data: elements}};
                let encoded: Vec<u8> = serialize(&response, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetCount => {
                let count = controls::mineral::get_mineral_count(get_db().conn());
                let response = Response{response_type : ResponseType::GetCount{ data: count}};
                let encoded: Vec<u8> = serialize(&response, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetImagesCount{id} => {
                let count = controls::mineral::get_images_count(get_db().conn(), id);
                let response = Response{response_type : ResponseType::GetImagesCount{ data: count}};
                let encoded: Vec<u8> = serialize(&response, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetImage{id, image} => {
                let image = controls::mineral::get_image(get_db().conn(), id, image);
                let response = Response{response_type : ResponseType::GetImage{ data: image}};
                let encoded: Vec<u8> = serialize(&response, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetLastUpdates{since} => {
                let elements = controls::mineral::get_last_updated(get_db().conn(), since);
                let response = Response{response_type : ResponseType::GetLastUpdates{ data: elements}};
                let encoded: Vec<u8> = serialize(&response, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            }
        };
    }
}
