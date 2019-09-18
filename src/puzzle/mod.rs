use std::fmt;

use crate::SVec;
use crate::point::Point;

pub struct Puzzle {
	width: u32,
	start: Point,
	content: Vec<u32>,
}

impl Puzzle {
	pub fn new(width: u32, start: Point, content: Vec<u32>) -> Self {
		Puzzle {
			width,
			start,
			content,
		}
	}

	fn format_table(&self) -> String {
		self.content
			.chunks(self.width as usize)
			.map(|row| row.iter().map(|e| e.to_string()).collect::<SVec>().join(" "))
			.collect::<SVec>()
			.join("\n")
	}

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Width: {}\nStart at: {}\n----------\n{}\n----------\n",
				self.width,
				self.start,
				self.format_table())
	}
}

impl fmt::Display for Puzzle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.fmt(f)
	}
}
