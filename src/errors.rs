use std:: {
    option::NoneError,
    error::Error as StdError,
    fmt::{self, Display}
};

#[derive(Debug)]
pub enum Error {
    None(NoneError)
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::None(error) => Error::None(error.clone())
        }
    }
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Error::None(ref left) => {
                if let Error::None(ref right) = other {
                    left == right
                } else {
                    false
                }
            }
        }
    }
}

impl From<NoneError> for Error {
    #[inline]
    fn from(none_error: NoneError) -> Error {
        Error::None(none_error)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::None(ref _e) => "Unwrapped a None",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match *self {
            Error::None(ref _e) => write!(f, "Unwrapped a None"),
        }
    }
}