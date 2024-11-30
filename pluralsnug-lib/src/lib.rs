#![allow(unused)]

use std::{fs, io as std_io, path::{self, PathBuf}};

use format::System;
use serde::{Deserialize, Serialize};

pub mod format;
pub mod types;

pub fn system_init() -> Result<System, std_io::Error> {
	let system: System = io::file_read(PathBuf::from("System/system.json"))?;
	Ok(system)
}

pub fn system_save(system: &format::System) -> Result<(), std_io::Error> {
	io::file_write(system, PathBuf::from("System/system.json"))?;
	Ok(())
}

mod io {
	use serde::{Deserialize, Serialize};
	use std::{fs, io as std_io, path};

	pub(super) fn file_write<T: Serialize, P: AsRef<path::Path>>(
		file: &T,
		path: P,
	) -> Result<(), std_io::Error> {
		let contents = serde_json::to_string(file)?;
		fs::create_dir_all(path.as_ref().parent().unwrap())?;
		fs::write(path.as_ref(), contents.as_bytes())?;
		Ok(())
	}

	pub(super) fn file_read<T: for<'a> Deserialize<'a>, P: AsRef<path::Path>>(
		path: P,
	) -> Result<T, std_io::Error> {
		let contents = fs::read_to_string(path.as_ref())?;
		let output = serde_json::from_str(&contents)?;
		Ok(output)
	}
}
