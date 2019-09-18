use std::fmt;

use crate::errors::{Error, ParseError};

type SVec = Vec<String>;

pub struct Point {
	x: u32,
	y: u32,
}

impl Point {
	pub fn identity() -> Self {
		Point::new(0u32, 0u32)
	}

	pub fn new(x: u32, y: u32) -> Self {
		Point { x, y, }
	}
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[x: {}, y: {}]", self.x, self.y)
	}
}

pub struct Puzzle {
	width: u32,
	start: Point,
	content: Vec<u32>,
}

fn get_width(opt: &Option<&Vec<u32>>) -> Result<u32, Error> {
	opt
		.filter(|iv| iv.len() == 1)
		.and_then(|iv| iv.get(0))
		.map(|u| *u)
		.ok_or(Error::ParseError(ParseError::MissingDimension))
}

fn fold_val_curr(max: u32) -> impl Fn(u32, &u32) -> u32 {
	move |mut acc: u32, e: &u32| { if *e < max { acc += 1 }; acc }
}

fn get_content(opt: &Option<&[Vec<u32>]>, width: u32) -> Result<Vec<u32>, Error> {
		let max = width * width;
		let fold_val = fold_val_curr(max);
		let rows: Vec<u32> = opt
			.ok_or(Error::ParseError(ParseError::MissingPuzzle))?
			.iter()
			.filter(|r| r.iter().fold(0u32, &fold_val) == width)
			.flatten()
			.map(|u| *u)
			.collect::<Vec<_>>();
		if rows.len() == max as usize {
			Ok(rows)
		} else {
			Err(Error::ParseError(ParseError::InvalidDimension))
		}
	}

fn fold_bool(mut acc: u32, rb: &bool) -> u32 {
	if *rb { acc += 1 }
	acc
}

fn get_start_and_validate(v: &[u32], width: u32) -> Result<Point, Error> {
	let l = v.len();
	let mut checker: Vec<bool> = v.iter().map(|_| false).collect();
	let mut start: Option<Point> = None;
	for (i, e) in v.iter().enumerate() {
		match *e {
			e if (e as usize) < l && !checker[e as usize] => {
				if e == 0 {
					let i_u32 = i as u32;
					start = Some(Point::new(i_u32 % width, i_u32 / width));
				}
				checker[e as usize] = true;
			},
			_ => Err(Error::ParseError(ParseError::InvalidPuzzle))?,
		}
	}
	start
		.filter(|_| checker.iter().fold(0u32, fold_bool) as usize == v.len())
		.ok_or(Error::ParseError(ParseError::MissingStart))
}

impl Puzzle {
	pub fn from_str(s: &str) -> Result<Puzzle, Error> {
		let nums = s
			.lines()
			.filter(|s| !s.starts_with('#'))
			.map(validate_line)
			.collect::<Result<Vec<Vec<u32>>, Error>>()?;

		let w = get_width(&nums.get(0))?;
		let c = get_content(&nums.get(1..), w)?;
		let start = get_start_and_validate(&c, w)?;

		Ok(Puzzle {
			width: w,
			content: c,
			start: start,
		})
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

fn validate_line(s: &str) -> Result<Vec<u32>, Error> {
	Ok(s
		.split(" ")
		.map(|e| e.parse::<u32>())
		.collect::<Result<Vec<u32>, std::num::ParseIntError>>()?)
}
