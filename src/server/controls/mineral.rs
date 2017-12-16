extern crate std;

use db::schema::elements::dsl::*;
use tasdcailloux::models::element::Element;
use tasdcailloux::models::Error;
use super::wrap_diesel_error;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

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
