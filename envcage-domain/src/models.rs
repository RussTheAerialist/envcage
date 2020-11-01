use super::schema::devices;
use super::schema::envlogs;
use bigdecimal::BigDecimal;
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
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

#[derive(Queryable, Debug)]
pub struct EnvLog {
    pub id: Uuid,
    pub device_id: Option<String>,
    pub created: DateTime<Utc>, // Always UTC
    pub temperature: BigDecimal,
    pub humidity: BigDecimal,
}

#[derive(Insertable)]
#[table_name = "envlogs"]
pub struct NewEnvLog<'a> {
    pub id: Uuid,
    pub device_id: Option<&'a str>,
    pub created: DateTime<Utc>,
    pub temperature: &'a BigDecimal,
    pub humidity: &'a BigDecimal,
}
