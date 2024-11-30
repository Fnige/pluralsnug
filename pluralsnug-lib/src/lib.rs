#![allow(unused)]

use std::{fs, io as std_io, path};

use format::System;
use serde::{Deserialize, Serialize};

pub mod format;
pub mod types;

fn file_write<T: Serialize, P: AsRef<path::Path>>(
	file: &T,
	path: P,
) -> Result<(), std_io::Error> {
	let contents = serde_json::to_string(file)?;
	fs::create_dir_all(path.as_ref().parent().unwrap())?;
	fs::write(path.as_ref(), contents.as_bytes())?;
	Ok(())
}

fn file_read<T: for<'a> Deserialize<'a>, P: AsRef<path::Path>>(
	path: P,
) -> Result<T, std_io::Error> {
	let contents = fs::read_to_string(path.as_ref())?;
	let output = serde_json::from_str(&contents)?;
	Ok(output)
}

pub fn system_init() -> Result<System, std_io::Error> {
	let system: System = file_read(path::PathBuf::from("System/system.json"))?;
	Ok(system)
}

pub fn system_save(system: format::System) -> Result<(), std_io::Error> {
	file_write(&system, path::PathBuf::from("System/system.json"))?;
	Ok(())
}
