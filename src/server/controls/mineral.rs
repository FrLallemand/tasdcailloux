extern crate std;

use db::schema::elements::dsl::*;
use tasdcailloux::models::element::Element;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

pub fn get_mineral(connection: &SqliteConnection, mineral_id: i32) -> Result<Element, Error>{
    elements
        .find(mineral_id)
        .first::<Element>(connection)
}

pub fn get_minerals(connection: &SqliteConnection, range: std::ops::Range<i32>) -> Result<Vec<Element>, Error>{
    elements
        .filter(id.between(range))
        .get_results::<Element>(connection)
}
