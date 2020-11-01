#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod models;
pub mod schema;

use crate::models::*;

pub fn db_connect(url: &str) -> Result<PgConnection, ConnectionError> {
    debug!("Connection String: {}", url);
    PgConnection::establish(url)
}

pub fn get_all_device(db: &PgConnection) -> Result<Vec<Device>, diesel::result::Error> {
    use crate::schema::devices::dsl::*;

    devices.load(db)
}

pub fn unapproved_devices(db: &PgConnection) -> QueryResult<Vec<Device>> {
    use crate::schema::devices::dsl::*;

    devices.filter(approved.eq(false)).load(db)
}

pub fn create_device<'a>(
    db: &PgConnection,
    mac_addr: &'a str,
    location: &'a str,
) -> Result<Device, diesel::result::Error> {
    use schema::devices;

    let new = NewDevice {
        id: mac_addr,
        location,
    };

    diesel::insert_into(devices::table)
        .values(&new)
        .get_result(db)
}

pub fn approve_device(db: &PgConnection, mac_addr: &str) -> QueryResult<Device> {
    set_device_approval(db, mac_addr, true)
}

pub fn disapprove_device(db: &PgConnection, mac_addr: &str) -> QueryResult<Device> {
    set_device_approval(db, mac_addr, false)
}

fn set_device_approval(db: &PgConnection, mac_addr: &str, state: bool) -> QueryResult<Device> {
    use schema::devices;

    let device = ApprovalDevice {
        id: mac_addr,
        approved: true,
    };

    diesel::update(&device)
        .set(devices::approved.eq(state))
        .get_result(db)
}
