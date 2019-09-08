use std::fmt;

pub enum Error {
	Utf8Error(std::str::Utf8Error),
	IoError(std::io::Error),
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

impl Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Error::Utf8Error(ref e) => write!(f, "Utf8Error ({})", e),
			Error::IoError(ref e) => write!(f, "IoError ({})", e),
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
