use dtt::datetime::DateTime;
use crate::types::rgb::Rgb;
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
