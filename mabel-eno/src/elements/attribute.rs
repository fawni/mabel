use std::cell::Cell;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;
use std::str::FromStr;

use crate::elements::{DocumentInternals, ElementImpl};
use crate::{Element, Error, Printer};

#[derive(Debug, Clone)]
pub struct Attribute {
    document_internals: Rc<DocumentInternals>,
    escape_operator_ranges: Option<(Range<usize>, Range<usize>)>,
    key_range: Range<usize>,
    line_begin_index: usize,
    pub line_number: u32,
    operator_index: usize,
    touched: Cell<bool>,
    value_range: Option<Range<usize>>,
}

pub trait AttributeImpl {
    fn as_element(&self) -> &dyn Element;
    fn get_value(&self) -> Option<String>;
    #[allow(clippy::new_ret_no_self)]
    fn new(
        document_internals: Rc<DocumentInternals>,
        escape_operator_ranges: Option<(Range<usize>, Range<usize>)>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
        operator_index: usize,
        value_range: Option<Range<usize>>,
    ) -> Attribute;
}

impl Attribute {
    pub fn key(&self) -> &str {
        &self.document_internals.content[self.key_range.clone()]
    }

    pub fn optional_value<T: FromStr>(&self) -> Option<Result<T, Error>>
    where
        <T as FromStr>::Err: std::fmt::Display,
        <T as FromStr>::Err: std::error::Error,
        <T as FromStr>::Err: 'static,
    {
        match self.get_value() {
            Some(value) => match value.parse::<T>() {
                Ok(converted) => Some(Ok(converted)),
                Err(err) => Some(Err(Error::with_source(
                    format!("{}", err),
                    self.line_number,
                    Box::new(err),
                ))),
            },
            None => None,
        }
    }

    pub fn required_value<T: FromStr>(&self) -> Result<T, Error>
    where
        <T as FromStr>::Err: std::fmt::Display,
        <T as FromStr>::Err: std::error::Error,
        <T as FromStr>::Err: 'static,
    {
        match self.get_value() {
            Some(value) => match value.parse::<T>() {
                Ok(converted) => Ok(converted),
                Err(err) => Err(Error::with_source(
                    format!("{}", err),
                    self.line_number,
                    Box::new(err),
                )),
            },
            None => Err(Error::new("Value expected".to_owned(), self.line_number)),
        }
    }
}

impl AttributeImpl for Attribute {
    fn as_element(&self) -> &dyn Element {
        self
    }

    fn get_value(&self) -> Option<String> {
        self.value_range
            .as_ref()
            .map(|range| self.document_internals.content[range.clone()].to_owned())
    }

    fn new(
        document_internals: Rc<DocumentInternals>,
        escape_operator_ranges: Option<(Range<usize>, Range<usize>)>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
        operator_index: usize,
        value_range: Option<Range<usize>>,
    ) -> Attribute {
        Attribute {
            document_internals,
            escape_operator_ranges,
            key_range,
            line_begin_index,
            line_number,
            operator_index,
            touched: Cell::new(false),
            value_range,
        }
    }
}

impl Element for Attribute {
    fn as_attribute(&self) -> Option<&Attribute> {
        Some(self)
    }

    fn is_attribute(&self) -> bool {
        true
    }

    fn line_number(&self) -> u32 {
        self.line_number
    }

    fn snippet(&self) -> String {
        self.snippet_with_options(&*self.document_internals.default_printer, true)
    }

    fn snippet_with_options(&self, printer: &dyn Printer, gutter: bool) -> String {
        let mut out = String::new();

        if gutter {
            out.push_str(&printer.gutter(self.line_number));
        }

        if let Some((escape_begin_range, escape_end_range)) = &self.escape_operator_ranges {
            out.push_str(
                &self.document_internals.content[self.line_begin_index..escape_begin_range.start],
            );
            out.push_str(
                &printer.operator(&self.document_internals.content[escape_begin_range.clone()]),
            );
            out.push_str(
                &self.document_internals.content[escape_begin_range.end..self.key_range.start],
            );
            out.push_str(&printer.key(&self.document_internals.content[self.key_range.clone()]));
            out.push_str(
                &self.document_internals.content[self.key_range.end..escape_end_range.start],
            );
            out.push_str(
                &printer.operator(&self.document_internals.content[escape_end_range.clone()]),
            );
            out.push_str(
                &self.document_internals.content[escape_end_range.end..self.operator_index],
            );
        } else {
            out.push_str(
                &self.document_internals.content[self.line_begin_index..self.key_range.start],
            );
            out.push_str(&printer.key(&self.document_internals.content[self.key_range.clone()]));
            out.push_str(&self.document_internals.content[self.key_range.end..self.operator_index]);
        }

        out.push_str(&printer.operator("="));

        if let Some(value_range) = &self.value_range {
            out.push_str(
                &self.document_internals.content[(self.operator_index + 1)..value_range.start],
            );
            out.push_str(&printer.value(&self.document_internals.content[value_range.clone()]));
        }

        out
    }

    fn touch(&self) {
        self.touched.set(true);
    }
}

impl ElementImpl for Attribute {
    fn line_range(&self) -> RangeInclusive<u32> {
        self.line_number..=self.line_number
    }

    fn touched(&self) -> bool {
        self.touched.get()
    }
}
