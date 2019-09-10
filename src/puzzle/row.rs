use std::fmt;

pub struct PuzzleRow(Vec<u32>);

impl PuzzleRow {
    pub fn new(v: &Vec<u32>) -> Self {
        PuzzleRow(v.to_vec())
    }
    
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = self.0
			.iter()
			.map(|x| x.to_string())
			.collect::<Vec<String>>()
			.join(" ");
		write!(f, "{}", s)
	}
}

impl fmt::Display for PuzzleRow {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.fmt(f)
	}
}
