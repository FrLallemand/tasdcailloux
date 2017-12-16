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
use tasdcailloux::models::{Message, MessageType};
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
            let elements = controls::mineral::get_minerals(db.conn(), std::ops::Range{start: from, end: to})?;
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
    //let element = controls::mineral::get_mineral(get_db().conn(), 1);
    let mut socket_pull = Socket::new(Protocol::Pull).unwrap();
    socket_pull.bind(&"tcp://127.0.0.1:5555").expect("Fail to bind to tcp port");



    loop {
        let mut msg = Vec::new();
        socket_pull.read_to_end(&mut msg).unwrap();
        let decoded: Message = deserialize(&msg).unwrap();
        match decoded.message_type {
            MessageType::GetOne => {
                println!("GetOne !");
            },
            MessageType::GetRange{from, to} => {
                println!("GetRange from {} to {} !", from, to);
            },
            MessageType::GetAll => {
                println!("GetAll !");
            }
        };
    }
}