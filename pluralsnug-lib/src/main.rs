#![allow(unused)]
#![warn(unused_imports)]

use pluralsnug_lib::{format::System, system_save};
use rgb::Rgb;
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let sys = System {
		name: Some("Fnige".to_owned()),
		description: None,
		pronouns: Some("She/Her".to_owned()),
		colour: Rgb { r: 0, g: 117, b: 255},
		avatar_url: Url::parse("https://cdn.discordapp.com/avatars/315203411488276482/7d60330a065503858ebae73bfd19f4f9.png?size=256")?
	};

	system_save(&sys);
   Ok(())
}
