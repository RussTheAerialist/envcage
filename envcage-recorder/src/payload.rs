use std::str::FromStr;
use serde::Deserialize;
use chrono::prelude::*;
use chrono::Utc;
use super::error::Error;

#[derive(Deserialize, Debug)]
pub(crate) struct Payload {
	pub created: DateTime<Utc>,
	pub temperature: f32,
	pub humidity: f32,
}

impl FromStr for Payload {
	fn from_str(s: &str) -> Result<Payload, Self::Err> {
		serde_json::from_str(s).map_err(|e| Error::PayloadParse(format!("{}\n--\n{:?}", s, e)))
	}

	type Err = Error;
}