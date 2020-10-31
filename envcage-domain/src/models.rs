use super::schema::devices;
use chrono::prelude::*;

#[derive(Queryable)]
pub struct Device {
    pub id: String,
    pub location: String,
    pub approved: bool,
}

#[derive(Insertable)]
#[table_name = "devices"]
pub struct NewDevice<'a> {
    pub id: &'a str,
    pub location: &'a str,
}

#[derive(Identifiable)]
#[table_name = "devices"]
pub struct ApprovalDevice<'a> {
    pub id: &'a str,
    pub approved: bool,
}

#[derive(Queryable)]
pub struct EnvLog {
    pub id: i32,
    pub device_id: String,
    pub created: NaiveDateTime, // Always UTC
    pub temperature: i32,
    pub humidity: i32,
}
