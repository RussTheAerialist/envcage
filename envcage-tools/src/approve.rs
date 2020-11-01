use std::{fmt::Display, str::FromStr};

use envcage_domain::{approve_device, disapprove_device};
use envcage_tools::cmdline;
use structopt::StructOpt;

#[derive(Debug)]
enum ApprovalState {
	Approve,
	Disapprove,
}

#[derive(Debug)]
struct InvalidState {
	state: String,
}

impl Display for InvalidState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state)
    }
}

impl Error for InvalidState {

}

impl FromStr for ApprovalState {
    type Err = InvalidState;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
					"approve" => Ok(ApprovalState::Approve),
					"disapprove" => Ok(ApprovalState::Disapprove),
					_ => Err(InvalidState { state: s.to_owned() })
				}
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
	state: ApprovalState,

	mac_addr: String,
}

cmdline! { db Opt opt
	{
		use ApprovalState::*;

		let mac = &opt.mac_addr;
		match opt.state {

			Approve => approve_device(&db, mac),
			Disapprove => disapprove_device(&db, mac),
		}?;
	}
}
