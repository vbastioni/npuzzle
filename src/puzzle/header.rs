use std::ops::Deref;
use std::fmt;

use crate::errors::{Error, ParseError};

pub struct PuzzleHeader(usize);

impl PuzzleHeader {
	pub fn from_opt(opt: &Option<&Vec<u32>>) -> Result<Self, Error> {
		Ok(Self(opt
			.and_then(|iv| if iv.len() != 1 { None } else { iv.get(0) })
			.map(|u| (*u) as usize)
			.ok_or(Error::ParseError(ParseError::MissingDimension))?))
	}

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl fmt::Display for PuzzleHeader {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.fmt(f)
	}
}

impl Deref for PuzzleHeader {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
