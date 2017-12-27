extern crate std;

use db::schema::elements::dsl::*;
use db::schema::images::dsl::*;
use tasdcailloux::models::element::Element;
use tasdcailloux::models::image::Image;
use tasdcailloux::models::Error;
use super::wrap_diesel_error;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use std::os::unix;
use std::path::Path;


pub fn get_mineral(connection: &SqliteConnection, mineral_id: i32) -> Result<Element, Error>{
    match elements.
        find(mineral_id).
        first::<Element>(connection){
            Ok(query_result) => Ok(query_result),
            Err(e) => Err(wrap_diesel_error(e))
        }
}

pub fn get_mineral_range(connection: &SqliteConnection, from: i32, to: i32) -> Result<Vec<Element>, Error>{
    let range = std::ops::Range{start: from, end: to};
    match elements
        .filter(id.between(range))
        .get_results::<Element>(connection){
            Ok(query_result) => Ok(query_result),
            Err(e) => Err(wrap_diesel_error(e))
        }
}


pub fn get_mineral_all(connection: &SqliteConnection) -> Result<Vec<Element>, Error>{
    match elements.
        load::<Element>(connection){
            Ok(query_result) => Ok(query_result),
            Err(e) => Err(wrap_diesel_error(e))
        }
}

pub fn get_mineral_count(connection: &SqliteConnection) -> Result<i64, Error>{
    match elements.
        count().
        get_result(connection){
            Ok(query_result) => Ok(query_result),
            Err(e) => Err(wrap_diesel_error(e))
        }
}

pub fn get_images_count(connection: &SqliteConnection, mineral_id: i32) -> Result<i32, Error>{
    match images.
        find(mineral_id).
        first::<Image>(connection){
            Ok(img) => {
                match fs::read_dir(img.dir) {
                    Ok(paths) => {
                        Ok(paths.count() as i32)
                    },
                    Err(e) => Err(Error::InternalError)
                }
            },
            Err(e) => Err(wrap_diesel_error(e))
        }
}

pub fn get_image(connection: &SqliteConnection, mineral_id: i32, image_number: i32) -> Result<Vec<u8>, Error>{

    match images.
        find(mineral_id).
        first::<Image>(connection){
            Ok(img) => {
                match fs::read_dir(img.dir) {
                    Ok(mut paths) => {
                        if let Some(image_path) = paths.nth(image_number as usize){
                            match image_path {
                                Ok(image_path) => {
                                    let img = File::open(image_path.path());
                                    let mut reader = BufReader::new(img.unwrap());
                                    let mut content = Vec::new();
                                    reader.read_to_end(&mut content);
                                    Ok(content)
                                },
                                Err(_) => Err(Error::InternalError)
                            }
                        } else {
                            Err(Error::ImageNotFound)
                        }
                    },
                    Err(e) => Err(Error::InternalError)
                }
            },
            Err(e) => Err(wrap_diesel_error(e))
        }
}
