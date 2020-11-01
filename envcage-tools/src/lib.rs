#[macro_export]
macro_rules! cmdline {
 ($db:ident $Opt:ident $opt:ident { $( $body:tt ) *} ) => {
	cmdline! { $db {
		let $opt = $Opt::from_args();

		$( $body )*
	}}
 };
 ($db:ident { $( $body:tt )* }) => {

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
		let $db = db_connect(&database_url)?;

		$(
			$body
		)*

    Ok(())
}

	}
}
