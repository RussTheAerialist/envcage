#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;
pub mod models;

use crate::models::{Device, NewDevice};

pub fn db_connect(url: &str) -> Result<PgConnection, ConnectionError> {
    debug!("Connection String: {}", url);
    PgConnection::establish(url)
}

pub fn create_device<'a>(db: &PgConnection, location: &'a str) -> Result<Device, diesel::result::Error> {
    use schema::device;

    let new = NewDevice {
        location,
    };

    diesel::insert_into(device::table)
        .values(&new)
        .get_result(db)
}
