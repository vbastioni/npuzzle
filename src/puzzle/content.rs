use std::ops::Deref;
use std::fmt;

use crate::errors::{Error, ParseError};
use crate::puzzle::{row::PuzzleRow, header::PuzzleHeader};

pub struct PuzzleContent(Vec<PuzzleRow>);

impl PuzzleContent {
	pub fn from_opt(opt: &Option<&[Vec<u32>]>, width: &PuzzleHeader) -> Result<Self, Error> {
		let w = **width;
		let rows: Vec<PuzzleRow> = opt
			.ok_or(Error::ParseError(ParseError::MissingPuzzle))?
			.iter()
			.filter_map(|r| if r.len() != w { None } else { Some(PuzzleRow::new(r)) })
			.collect();
		if rows.len() == w {
			Ok(PuzzleContent(rows))
		} else {
			Err(Error::ParseError(ParseError::InvalidDimension))
		}
	}

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = self.0
			.iter()
			.map(|r| r.to_string())
			.collect::<Vec<String>>()
			.join("\n");
		write!(f, "{}", s)
	}
}

impl fmt::Display for PuzzleContent {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.fmt(f)
	}
}

impl Deref for PuzzleContent {
	type Target = Vec<PuzzleRow>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
