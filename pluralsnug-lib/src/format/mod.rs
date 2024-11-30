use crate::types::{rgb::Rgb, url::Url, uuid::Uuid};
use dtt::datetime::DateTime;
use serde::{Deserialize, Serialize};
use std::{fs::File, io as std_io, path::PathBuf};

/* well we got a new spanner in the works for this,
	idiomatic rust means we validate in a funny way

	most types wont need custom validation at least
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct System {
	pub name: Option<String>,
	pub description: Option<String>,
	pub pronouns: Option<String>,
	pub colour: Option<Rgb<u8>>,
	pub avatar_url: Option<Url>,
	pub members: Option<Vec<Member>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
	pub uuid: Uuid,
	pub name: String,
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
	pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Switch {
	pub date: DateTime,
}
*/
