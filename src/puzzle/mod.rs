use std::fmt;

use crate::SVec;
use crate::point::Point;

pub struct Puzzle {
	width: u32,
	start: Point,
	content: Vec<u32>,
}

fn get_width(u: u32) -> u32 {
	let mut i = 0u32;
	let mut mut_u = u;
	while mut_u != 0 {
		i += 1;
		mut_u /= 10;
	}
	i
}

#[test]
fn test_get_width() {
	assert_eq!(get_width(1u32), 1);
	assert_eq!(get_width(10u32), 2);
	assert_eq!(get_width(100u32), 3);
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
		use atty::Stream;

		let padding = self.content.iter()
			.max()
			.map(|m| get_width(*m))
			.unwrap_or(1) as usize;
		self.content
			.chunks(self.width as usize)
			.map(|row| row.iter()
				.map(|e| match e {
					0 if atty::is(Stream::Stdout) =>
						format!("{}{:^width$}{}", "\x1b[32m", "\u{2022}", "\x1b[0m", width=padding),
					0 => format!("{:^width$}", "\u{2022}", width=padding),
					_ => format!("{:>width$}", e.to_string(), width=padding),
				})
				.collect::<SVec>()
				.join(" "))
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
