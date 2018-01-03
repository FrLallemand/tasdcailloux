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
use std::io::{BufReader, Read};
use chrono::naive;

pub fn get_mineral(connection: &SqliteConnection, mineral_id: i32) -> Result<Element, Error>{
    elements
        .find(mineral_id)
        .first::<Element>(connection)
        .map_err(wrap_diesel_error)
}

pub fn get_mineral_range(connection: &SqliteConnection, from: i32, to: i32) -> Result<Vec<Element>, Error>{
    let range = std::ops::Range{start: from, end: to};
    elements
        .filter(id.between(range))
        .get_results::<Element>(connection)
        .map_err(wrap_diesel_error)
}

pub fn get_last_updated(connection: &SqliteConnection, since: naive::NaiveDateTime) -> Result<Vec<Element>, Error>{
    elements
        .filter(last_updated.gt(since))
        .get_results::<Element>(connection)
        .map_err(wrap_diesel_error)
}


pub fn get_mineral_all(connection: &SqliteConnection) -> Result<Vec<Element>, Error>{
    elements
        .load::<Element>(connection)
        .map_err(wrap_diesel_error)
}

pub fn get_mineral_count(connection: &SqliteConnection) -> Result<i32, Error>{
    elements
        .count()
        .get_result(connection)
        .map_err(wrap_diesel_error)
        .map(|a: i64| {
            a as i32
        })
}

pub fn get_images_count(connection: &SqliteConnection, mineral_id: i32) -> Result<i32, Error>{
    images
        .find(mineral_id)
        .first::<Image>(connection)
        .map_err(wrap_diesel_error)
        .and_then( |img|{
            fs::read_dir(img.dir)
                .map(|paths| { paths.count() as i32})
                .map_err(|_| Error::InternalError)
        })
}

pub fn get_image(connection: &SqliteConnection, mineral_id: i32, image_number: i32) -> Result<Vec<u8>, Error>{

    images
        .find(mineral_id)
        .first::<Image>(connection)
        .map_err(wrap_diesel_error)
        .and_then( |img| {
            fs::read_dir(img.dir)
                .map_err(|_| Error::InternalError)
        })
        .and_then( |mut paths| {
            paths.nth(image_number as usize)
                .ok_or(Error::ImageNotFound)
        })
        .and_then( |image_path| {
            image_path
                .map_err(|_| Error::InternalError)
        })
        .and_then( |image_path| {
            File::open(image_path.path())
                .map_err(|_| Error::InternalError)
        })
        .and_then( |img| {
            let mut reader = BufReader::new(img);
            let mut content = Vec::new();
            reader.read_to_end(&mut content)
                .map_err(|_| Error::InternalError)
                .and(Ok(content))
        })
}
