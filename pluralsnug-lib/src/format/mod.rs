mod io;
pub mod system;

use io::file_write;
pub use system::System;

use dtt::datetime::DateTime;
use std::{fs::File, io as std_io, path::PathBuf};

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