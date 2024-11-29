mod io;
pub mod system;

use dtt::datetime::DateTime;
use io::{file_read, file_write};
use rgb::Rgb;
use serde::{Deserialize, Serialize};
use std::{fs::File, io as std_io, path::PathBuf};

/* well we got a new spanner in the works for this,
	idiomatic rust means we validate in a funny way

	as itll get repetitive, nnv = needs no validation
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct System {
	name: Option<String>,        // nnv
	description: Option<String>, // nnv
	pronouns: Option<String>,    // nnv
	colour: Rgb<u8>,
}

impl System {
	pub(crate) fn export(&self) -> Result<(), std_io::Error> {
		io::file_write(self, PathBuf::from("System/system.json"))?;
		Ok(())
	}
	pub(crate) fn import() -> Result<Self, std_io::Error> {
		let read: System = io::file_read(PathBuf::from("System/system.json"))?;
		Ok(read)
	}
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
	pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
	pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Switch {
	pub date: DateTime,
}
*/
