// Copyright 2018 Althea Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Message(String),
    TrailingBytes,
    EmptyBuffer,
    ListPrefixTooSmall,
    StringPrefixTooSmall,
    ExpectedList,
    ExpectedString,
    InvalidString,
    WrongPrefix,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn as_str(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            Error::TrailingBytes => "Trailing bytes found at the end of input",
            Error::EmptyBuffer => "Empty buffer detected",
            Error::ListPrefixTooSmall => "List prefix is bigger than the data",
            Error::StringPrefixTooSmall => "String prefix is bigger than the data",
            Error::ExpectedList => "Expected list data",
            Error::ExpectedString => "Expected string",
            Error::InvalidString => "Unable to decode valid string",
            Error::WrongPrefix => "Wrong prefix",
        }
    }
}
