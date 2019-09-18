use std::fmt;

pub enum ParseError {
	MissingDimension,
	InvalidDimension,
	MissingPuzzle,
	InvalidPuzzle,
	MissingStart,
	InvalidNumber(std::num::ParseIntError),
}

impl ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			ParseError::MissingDimension => write!(f, "MissingDimension"),
			ParseError::InvalidDimension => write!(f, "InvalidDimension"),
			ParseError::MissingPuzzle => write!(f, "MissingPuzzle"),
			ParseError::InvalidPuzzle => write!(f, "InvalidPuzzle"),
			ParseError::MissingStart => write!(f, "MissingStart"),
			ParseError::InvalidNumber(ref e) => write!(f, "InvalidNumber ({})", e),
			#[allow(unreachable_patterns)]
			_ => write!(f, "Generic ParseError"),
		}
	}
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.fmt(f)
	}
}

pub enum Error {
	Utf8Error(std::str::Utf8Error),
	IoError(std::io::Error),
	ParseError(ParseError)
}

impl From<std::str::Utf8Error> for Error {
	fn from(error: std::str::Utf8Error) -> Self {
		Error::Utf8Error(error)
	}
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Self {
		Error::IoError(error)
	}
}

impl From<std::num::ParseIntError> for Error {
	fn from(error: std::num::ParseIntError) -> Self {
		Error::ParseError(ParseError::InvalidNumber(error))
	}
}

impl Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Error::Utf8Error(ref e) => write!(f, "Utf8Error ({})", e),
			Error::IoError(ref e) => write!(f, "IoError ({})", e),
			Error::ParseError(ref pe) => write!(f, "ParseError ({})", pe),
			#[allow(unreachable_patterns)]
			_ => write!(f, "Generic Error written")
		}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.fmt(f)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.fmt(f)
    }
}
