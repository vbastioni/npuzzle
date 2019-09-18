use crate::errors::{Error, ParseError};
use crate::puzzle::Puzzle;
use crate::point::Point;

pub fn parse(s: &str) -> Result<Puzzle, Error> {
    let nums = s
        .lines()
        .filter(|s| !s.starts_with('#'))
        .map(map_line)
        .collect::<Result<Vec<Vec<_>>, Error>>()?;

    let w = get_width(&nums.get(0))?;
    let c = get_content(&nums.get(1..), w)?;
    let start = get_start_and_validate(&c, w)?;

    Ok(Puzzle::new(w, start, c))
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

fn map_line(s: &str) -> Result<Vec<u32>, Error> {
	Ok(s
		.split(" ")
		.filter_map(|e| if e == "" { None } else { Some(e.parse::<u32>()) })
		.collect::<Result<Vec<_>, std::num::ParseIntError>>()?)
}
