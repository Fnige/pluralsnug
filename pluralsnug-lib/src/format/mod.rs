use crate::types::{rgb::Rgb, url::Url};
use dtt::datetime::DateTime;
use serde::{Deserialize, Serialize};
use std::{fs::File, io as std_io, path::PathBuf};

/* well we got a new spanner in the works for this,
	idiomatic rust means we validate in a funny way

	as itll get repetitive, nnv = needs no validation
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct System {
	pub name: Option<String>,        // nnv
	pub description: Option<String>, // nnv
	pub pronouns: Option<String>,    // nnv
	pub colour: Rgb<u8>,
	pub avatar_url: Url,
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
