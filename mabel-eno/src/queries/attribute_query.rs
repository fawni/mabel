use std::str::FromStr;

use crate::elements::{Attribute, AttributeImpl, Element};
use crate::queries::{FieldQuery, FieldQueryImpl};
use crate::{Error, Printer};

pub struct AttributeQuery<'a> {
    element_option: Option<&'a Attribute>,
    key: Option<String>,
    parent: &'a FieldQuery<'a>,
}

pub trait AttributeQueryImpl<'a> {
    #[allow(clippy::new_ret_no_self)]
    fn new(
        element_option: Option<&'a Attribute>,
        key: Option<String>,
        parent: &'a FieldQuery<'a>,
    ) -> AttributeQuery<'a>;
}

impl<'a> AttributeQuery<'a> {
    fn missing_error(&self) -> Error {
        match self.parent.element() {
            Some(field) => Error::new(
                format!(
                    "Attribute {} not found",
                    self.key.as_deref().unwrap_or("(can have any key)")
                ),
                field.line_number,
            ),
            None => self.parent.missing_error(),
        }
    }

    pub fn optional_value(&self) -> Result<Option<String>, Error> {
        match &self.element_option {
            Some(attribute) => match &attribute.get_value() {
                Some(value) => Ok(Some(value.to_string())),
                None => Ok(None),
            },
            None => Err(self.missing_error()),
        }
    }

    pub fn required_value<T: FromStr>(&self) -> Result<T, Error>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        match &self.element_option {
            Some(attribute) => match &attribute.get_value() {
                Some(value) => match value.parse::<T>() {
                    Ok(converted) => Ok(converted),
                    Err(err) => Err(Error::new(format!("{}", err), attribute.line_number)),
                },
                None => Err(Error::new(
                    "Missing value".to_string(),
                    attribute.line_number,
                )),
            },
            None => Err(self.missing_error()),
        }
    }

    pub fn snippet(&self) -> Result<String, Error> {
        match self.element_option {
            Some(attribute) => Ok(attribute.snippet()),
            None => Err(self.missing_error()),
        }
    }

    pub fn snippet_with_options(
        &self,
        printer: &dyn Printer,
        gutter: bool,
    ) -> Result<String, Error> {
        match self.element_option {
            Some(attribute) => Ok(attribute.snippet_with_options(printer, gutter)),
            None => Err(self.missing_error()),
        }
    }
}

impl<'a> AttributeQueryImpl<'a> for AttributeQuery<'a> {
    fn new(
        element_option: Option<&'a Attribute>,
        key: Option<String>,
        parent: &'a FieldQuery<'a>,
    ) -> AttributeQuery<'a> {
        AttributeQuery {
            element_option,
            key,
            parent,
        }
    }
}
