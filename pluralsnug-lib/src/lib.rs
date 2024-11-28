#![allow(unused)]

use std::io;

use format::System;
pub mod format;
pub mod types;

pub fn system_init() -> Result<System, io::Error> {
	System::import()
}

pub fn system_save(system: System) -> Result<(), io::Error> {
	system.export()?;
	Ok(())
}