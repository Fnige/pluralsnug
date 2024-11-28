use crate::types::{self, rgb};
use super::io::{file_read, file_write};
use rgb::Rgb;
use serde::{Deserialize, Serialize};
use std::{
	io,
	path::{Path, PathBuf},
};

/* well we got a new spanner in the works for this, 
	idiomatic rust means we validate in a funny way

	as itll get repetitive, nnv = needs no validation
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct System {
	name: Option<String>,			// nnv
	description: Option<String>,	// nnv
	pronouns: Option<String>,		// nnv
	colour: rgb<u8>,
}

impl System {
	pub(crate) fn export(&self) -> Result<(), io::Error> {
		file_write(self, PathBuf::from("System/system.json"))?;
		Ok(())
	}
	pub(crate) fn import() -> Result<Self, io::Error> {
		let read: System = file_read(PathBuf::from("System/system.json"))?;
		Ok(read)
	}
}