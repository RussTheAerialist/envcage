
use std::fmt::Display;
#[derive(Debug)]
pub(crate) enum Error {
	Split(String),
	PayloadParse(String),
}

impl std::error::Error for Error { }
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			use Error::*;

			match self {
				Split(x) => write!(f, "Unable to parse topic: {}", x),
				PayloadParse(x) => write!(f, "Unable to parse payload: {}", x),
			}
    }
}