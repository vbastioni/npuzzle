use std::fmt;

use crate::errors::Error;

mod content;
mod row;
mod header;

pub struct Puzzle {
	width: header::PuzzleHeader,
	content: content::PuzzleContent,
}

impl Puzzle {
	pub fn from_str(s: &str) -> Result<Puzzle, Error> {
		let nums = s
			.lines()
			.filter(|s| !s.starts_with('#'))
			.map(validate_line)
			.collect::<Result<Vec<Vec<u32>>, Error>>()?;

		let h = header::PuzzleHeader::from_opt(&nums.get(0))?;
		let c = content::PuzzleContent::from_opt(&nums.get(1..), &h)?;
	
		Ok(Puzzle {
			width: h,
			content: c,
		})
	}

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Width: {}\n----------\n{}\n----------\n",
				self.width,
				self.content)
	}
}

impl fmt::Display for Puzzle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.fmt(f)
	}
}

fn validate_line(s: &str) -> Result<Vec<u32>, Error> {
	Ok(s
		.split(" ")
		.map(|e| e.parse::<u32>())
		.collect::<Result<Vec<u32>, std::num::ParseIntError>>()?)
}
