#![recursion_limit="128"]
#![feature(plugin)]
#![feature(plugin, custom_derive)]

extern crate tasdcailloux;

#[macro_use]extern crate lazy_static;

#[macro_use]extern crate diesel;
#[macro_use]extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate dotenv;
extern crate nanomsg;

#[macro_use]
extern crate serde_derive;
extern crate bincode;


pub mod db;
pub mod controls;

use nanomsg::{Socket, Protocol};
use bincode::{serialize, deserialize, Infinite};
use tasdcailloux::models::element::Element;
use tasdcailloux::models::{Message, MessageType, Error};
use db::get_db;
use std::io::{Read, Write};
/*
#[derive(FromForm)]
struct Range {
    from: Option<i32>,
    to: Option<i32>
}

#[get("/minerals/<id>")]
fn mineral_get(db: DB, id: i32) -> Result<Json<Element>, Error> {
    let element = controls::mineral::get_mineral(db.conn(), id)?;
    Ok(Json(element))
}

#[get("/minerals?<range>")]
fn minerals_get(range: Range, db: DB) -> Result<Json<Vec<Element>>, Error> {
    let mut result = Err(Error::InternalServerError);
    if let Some(from) = range.from {
        if let Some(to) = range.to {
            let elements = controls::mineral::get_minerals(db.conn(), )?;
            result = Ok(Json(elements));
        }
    }
    result
}


#[get("/minerals/<id>/origin")]
fn origin_get(db: DB, id: i32) -> Result<Json<Element>, Error> {
    let element = controls::mineral::get_mineral(db.conn(), id)?;
    Ok(Json(element))
}

#[get("/minerals/<id>/dimensions")]
fn dimension_get(db: DB, id: i32) -> Result<Json<Element>, Error> {
    let element = controls::mineral::get_mineral(db.conn(), id)?;
    Ok(Json(element))
}
*/
fn main() {
    let mut socket = Socket::new(Protocol::Rep).unwrap();
    socket.bind(&"tcp://127.0.0.1:5555").expect("Fail to bind to tcp port");

    loop {
        let mut msg = Vec::new();
        socket.read_to_end(&mut msg).unwrap();
        let decoded: Message = deserialize(&msg).unwrap();
        match decoded.message_type {
            MessageType::IsReady => {
                let result: Result<bool, &str> = Ok(true);
                let encoded: Vec<u8> = serialize(&result, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetOne{id} => {
                let element = controls::mineral::get_mineral(get_db().conn(), id);
                let encoded: Vec<u8> = serialize(&element, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetRange{from, to} => {
                let element = controls::mineral::get_mineral_range(get_db().conn(), from, to);
                let encoded: Vec<u8> = serialize(&element, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetAll => {
                let element = controls::mineral::get_mineral_all(get_db().conn());
                let encoded: Vec<u8> = serialize(&element, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            },
            MessageType::GetCount => {
                let element = controls::mineral::get_mineral_count(get_db().conn());
                let encoded: Vec<u8> = serialize(&element, Infinite).unwrap();
                socket.write(&encoded).unwrap();
            }
        };
    }
}
