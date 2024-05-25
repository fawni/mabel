use std::fmt;

use crate::locale::Locale;

/// `line` is 1-indexed (i.e. the first line is line 1, not 0)
#[derive(Debug)]
pub struct Error {
    pub line: u32,
    pub message: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new(message: String, line_number: u32) -> Error {
        Error {
            line: line_number,
            message,
            source: None,
        }
    }

    pub fn with_source(
        message: String,
        line_number: u32,
        source: Box<dyn std::error::Error>,
    ) -> Error {
        Error {
            line: line_number,
            message,
            source: Some(source),
        }
    }
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        error.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}

pub fn attribute_outside_field<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::attribute_outside_field(line_number, line), line_number)
}

pub fn attribute_without_key<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::attribute_without_key(line_number, line), line_number)
}

pub fn embed_without_key<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::embed_without_key(line_number, line), line_number)
}

pub fn escape_without_key<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::escape_without_key(line_number, line), line_number)
}

pub fn field_without_key<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::field_without_key(line_number, line), line_number)
}

pub fn invalid_after_escape<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::invalid_after_escape(line_number, line), line_number)
}

pub fn item_outside_field<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::item_outside_field(line_number, line), line_number)
}

pub fn mixed_field_content<T: Locale>(line_number: u32) -> Error {
    Error::new(T::mixed_field_content(line_number), line_number)
}

pub fn section_level_skip<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::section_level_skip(line_number, line), line_number)
}

pub fn section_without_key<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::section_without_key(line_number, line), line_number)
}

pub fn unterminated_embed<T: Locale>(key: &str, line_number: u32) -> Error {
    Error::new(T::unterminated_embed(key, line_number), line_number)
}

pub fn unterminated_escaped_key<T: Locale>(line_number: u32, line: &str) -> Error {
    Error::new(T::unterminated_escaped_key(line_number, line), line_number)
}
