use super::schema::device;

#[derive(Queryable)]
pub struct Device {
    pub id: i32,
    pub location: String
}

#[derive(Insertable)]
#[table_name="device"]
pub struct NewDevice<'a> {
    pub location: &'a str,
}
