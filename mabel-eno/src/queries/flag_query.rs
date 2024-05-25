use crate::elements::{Document, Element, Flag, Section};
use crate::queries::{SectionQuery, SectionQueryImpl};
use crate::{Error, Printer};

pub struct FlagQuery<'a> {
    element_option: Option<&'a Flag>,
    key: Option<String>,
    parent: FlagQueryParent<'a>,
}

pub trait FlagQueryImpl<'a> {
    #[allow(clippy::new_ret_no_self)]
    fn new(
        element_option: Option<&'a Flag>,
        key: Option<String>,
        parent: FlagQueryParent<'a>,
    ) -> FlagQuery<'a>;
}

pub enum FlagQueryParent<'a> {
    Document(&'a Document),
    Section(&'a Section),
    SectionQuery(&'a SectionQuery<'a>),
}

impl<'a> FlagQuery<'a> {
    pub fn missing_error(&self) -> Error {
        match self.parent {
            FlagQueryParent::Document(_) => Error::new(
                format!(
                    "Flag {} not found",
                    self.key.as_deref().unwrap_or("(can have any key)")
                ),
                Document::LINE_NUMBER,
            ),
            FlagQueryParent::Section(section) => Error::new(
                format!(
                    "Flag {} not found",
                    self.key.as_deref().unwrap_or("(can have any key)")
                ),
                section.line_number,
            ),
            FlagQueryParent::SectionQuery(section_query) => match section_query.element() {
                Some(section) => Error::new(
                    format!(
                        "Flag {} not found",
                        self.key.as_deref().unwrap_or("(can have any key)")
                    ),
                    section.line_number,
                ),
                None => section_query.missing_error(),
            },
        }
    }

    pub fn snippet(&self) -> Result<String, Error> {
        match self.element_option {
            Some(flag) => Ok(flag.snippet()),
            None => Err(self.missing_error()),
        }
    }

    pub fn snippet_with_options(
        &self,
        printer: &dyn Printer,
        gutter: bool,
    ) -> Result<String, Error> {
        match self.element_option {
            Some(flag) => Ok(flag.snippet_with_options(printer, gutter)),
            None => Err(self.missing_error()),
        }
    }
}

impl<'a> FlagQueryImpl<'a> for FlagQuery<'a> {
    fn new(
        element_option: Option<&'a Flag>,
        key: Option<String>,
        parent: FlagQueryParent<'a>,
    ) -> FlagQuery<'a> {
        FlagQuery {
            element_option,
            key,
            parent,
        }
    }
}
