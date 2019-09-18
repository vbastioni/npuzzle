use std::fmt;

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