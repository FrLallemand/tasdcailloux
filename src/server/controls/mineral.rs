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
            Ok(element) => Ok(element),
            Err(e) => Err(wrap_diesel_error(e))
        }
}

pub fn get_minerals(connection: &SqliteConnection, range: std::ops::Range<i32>) -> Result<Vec<Element>, Error>{
    match elements
        .filter(id.between(range))
        .get_results::<Element>(connection){
            Ok(minerals) => Ok(minerals),
            Err(e) => Err(wrap_diesel_error(e))
        }

}
