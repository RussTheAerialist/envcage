#[macro_use]
extern crate log;

use dotenv::dotenv;
use std::env;
use std::error::Error;

use envcage_domain::*;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")?;
    let db = db_connect(&database_url)?;
    info!("Connected to database");

    let results = get_all_device(&db)?;
    if results.is_empty() {
        create_device(&db, "002596123456", "Main Filament Storage")?;
    }
    for d in results {
        println!("{}: {} ({})", d.id, d.location, d.approved);
    }

    Ok(())
}
