#![recursion_limit="128"]
#![feature(plugin)]
#![feature(plugin, custom_derive)]

#[macro_use]extern crate lazy_static;

#[macro_use]extern crate diesel;
#[macro_use]extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

extern crate dotenv;
extern crate chrono;

pub mod models;

use bincode::{serialize, deserialize, Infinite};
use models::element::Element;

fn test() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}
