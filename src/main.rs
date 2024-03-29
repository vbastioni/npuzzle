#[macro_use]
extern crate clap;
extern crate atty;

const VERSION: &str = "0.1.0";
const DESCRIPTION: &str = "Does awesome things";
const AUTHORS: &str = "Vincent B. <vbastion@sutdent.42.fr\nOther P. <opeople@student.42.fr";

use std::io::Read;

mod errors;
mod puzzle;
mod parser;
mod point;

type SVec = Vec<String>;

// TODO: Maybe impl From<&str> for Puzzle
// use puzzle::Puzzle;

use parser::parse;
use errors::Error;

fn from_stdin() -> Result<String, Error> {
	let mut v: Vec<u8> = Vec::new();
	let stdin = std::io::stdin();
	let mut handle = stdin.lock();
	handle.read_to_end(&mut v)?;
	Ok(std::str::from_utf8(&v)?.to_string())
}

fn main() -> Result<(), Error> {
	let matches = clap_app!(n_puzzle =>
		(version: VERSION)
		(author: AUTHORS)
		(about: DESCRIPTION)
		(@arg file: -f --file +takes_value "Reads puzzle from a file instead of stdin")
	).get_matches();

	let c = match matches.value_of("file") {
		Some(m) => std::fs::read_to_string(m).map_err(Error::from),
		_ => from_stdin(),
	}?;

	let p = parse(&c)?;
	println!("{}", p);
	Ok(())
}
