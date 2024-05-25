use std::str::FromStr;

use crate::elements::{Document, Element, Embed, EmbedImpl, Section};
use crate::queries::{SectionQuery, SectionQueryImpl};
use crate::{Error, Printer};

pub struct EmbedQuery<'a> {
    element_option: Option<&'a Embed>,
    key: Option<String>,
    parent: EmbedQueryParent<'a>,
}

pub trait EmbedQueryImpl<'a> {
    #[allow(clippy::new_ret_no_self)]
    fn new(
        element_option: Option<&'a Embed>,
        key: Option<String>,
        parent: EmbedQueryParent<'a>,
    ) -> EmbedQuery<'a>;
}

pub enum EmbedQueryParent<'a> {
    Document(&'a Document),
    Section(&'a Section),
    SectionQuery(&'a SectionQuery<'a>),
}

impl<'a> EmbedQuery<'a> {
    pub fn missing_error(&self) -> Error {
        match self.parent {
            EmbedQueryParent::Document(_) => Error::new(
                format!(
                    "Embed {} not found",
                    self.key.as_deref().unwrap_or("(can have any key)")
                ),
                Document::LINE_NUMBER,
            ),
            EmbedQueryParent::Section(section) => Error::new(
                format!(
                    "Embed {} not found",
                    self.key.as_deref().unwrap_or("(can have any key)")
                ),
                section.line_number,
            ),
            EmbedQueryParent::SectionQuery(section_query) => match section_query.element() {
                Some(section) => Error::new(
                    format!(
                        "Embed {} not found",
                        self.key.as_deref().unwrap_or("(can have any key)")
                    ),
                    section.line_number,
                ),
                None => section_query.missing_error(),
            },
        }
    }

    pub fn optional_value(&self) -> Result<Option<String>, Error> {
        match &self.element_option {
            Some(embed) => match &embed.get_value() {
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
            Some(embed) => match &embed.get_value() {
                Some(value) => match value.parse::<T>() {
                    Ok(converted) => Ok(converted),
                    Err(err) => Err(Error::new(format!("{}", err), embed.line_number)),
                },
                None => Err(Error::new("Missing value".to_string(), embed.line_number)),
            },
            None => Err(self.missing_error()),
        }
    }

    pub fn snippet(&self) -> Result<String, Error> {
        match self.element_option {
            Some(embed) => Ok(embed.snippet()),
            None => Err(self.missing_error()),
        }
    }

    pub fn snippet_with_options(
        &self,
        printer: &dyn Printer,
        gutter: bool,
    ) -> Result<String, Error> {
        match self.element_option {
            Some(embed) => Ok(embed.snippet_with_options(printer, gutter)),
            None => Err(self.missing_error()),
        }
    }
}

impl<'a> EmbedQueryImpl<'a> for EmbedQuery<'a> {
    fn new(
        element_option: Option<&'a Embed>,
        key: Option<String>,
        parent: EmbedQueryParent<'a>,
    ) -> EmbedQuery<'a> {
        EmbedQuery {
            element_option,
            key,
            parent,
        }
    }
}
