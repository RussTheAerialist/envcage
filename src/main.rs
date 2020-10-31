#[macro_use]
extern crate log;

use dotenv::dotenv;
use std::env;
use std::error::Error;

use envcage::*;
use envcage::models::*;
use diesel::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
  use envcage::schema::device::dsl::*;

  dotenv().ok();
  env_logger::init();

  let database_url = env::var("DATABASE_URL")?;
  let db = db_connect(&database_url)?;
  info!("Connected to database");

  let results = device.load::<Device>(&db)?;
  if results.len() == 0 {
      create_device(&db, "Main Filament Storage")?;
  }
  for d in results {
      println!("{}: {}", d.id, d.location);
  }

  Ok(())
}
