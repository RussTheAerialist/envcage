use super::error::Error;
use bigdecimal::BigDecimal;
use chrono::prelude::*;
use chrono::Utc;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub(crate) struct Payload {
    pub created: DateTime<Utc>,
    pub temperature: BigDecimal,
    pub humidity: BigDecimal,
}

impl FromStr for Payload {
    fn from_str(s: &str) -> Result<Payload, Self::Err> {
        serde_json::from_str(s).map_err(|e| Error::PayloadParse(format!("{}\n--\n{:?}", s, e)))
    }

    type Err = Error;
}
