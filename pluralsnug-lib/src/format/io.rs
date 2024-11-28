use serde::{Deserialize, Serialize};
use std::{fs, io, path};

use super::system::System;

// Combine path and name, let path be fully handled in the function call instead
pub(super) fn file_write<T: Serialize, P: AsRef<path::Path>>(
	file: &T,
	path: P,
) -> Result<(), io::Error> {
	let contents = serde_json::to_string(file)?;
	fs::create_dir_all(path.as_ref().parent().unwrap())?;
	fs::write(path.as_ref(), contents.as_bytes())?;
	Ok(())
}

pub(super) fn file_read<T: for<'a> Deserialize<'a>, P: AsRef<path::Path>>(
	path: P,
) -> Result<T, io::Error> {
	let contents = fs::read_to_string(path.as_ref())?;
	let output = serde_json::from_str(&contents)?;
	Ok(output)
}
