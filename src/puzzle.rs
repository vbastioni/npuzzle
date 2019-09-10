use crate::errors::{Error, ParseError};

pub struct Puzzle {
	width: usize,
	content: Vec<Vec<u32>>,
}

fn get_puzzle(v: Vec<Vec<u32>>) -> Result<Puzzle, Error> {
	let width = v
		.get(0)
		.ok_or(Error::ParseError(ParseError::MissingDimension))
		.and_then(|v| {
			if v.len() != 1 {
				Err(Error::ParseError(ParseError::MissingDimension))
			} else {
				v
					.get(0)
					.ok_or(Error::ParseError(ParseError::MissingDimension))
			}
		})?;

	let rem = v
		.get(1..)
		.ok_or(Error::ParseError(ParseError::MissingPuzzle))?;

	if (rem
		.iter()
		.filter(|row| row.len() == *width as usize).collect::<Vec<_>>().len()
	) != (*width as usize) {
		Err(Error::ParseError(ParseError::InvalidDimension))?;
	}

	Ok(Puzzle {
		width: *width as usize,
		content: rem.to_vec(),
	})
}

fn validate_line(s: &str) -> Result<Vec<u32>, Error> {
	Ok(s
		.split(" ")
		.map(|e| e.parse::<u32>())
		.collect::<Result<Vec<u32>, std::num::ParseIntError>>()?)
}

pub fn parse_puzzle(s: &str) -> Result<Puzzle, Error> {
	let nums = s
		.lines()
		.filter(|s| !s.starts_with('#'))
		.map(validate_line)
		.collect::<Result<Vec<Vec<u32>>, Error>>()?;
	get_puzzle(nums)
}

