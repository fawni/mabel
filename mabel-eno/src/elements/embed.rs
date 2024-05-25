use std::cell::Cell;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;
use std::str::FromStr;

use crate::elements::{DocumentInternals, ElementImpl};
use crate::{Element, Error, Printer, SectionElement};

#[derive(Debug)]
pub struct Embed {
    document_internals: Rc<DocumentInternals>,
    key_range: Range<usize>,
    line_begin_index: usize,
    pub line_number: u32,
    operator_range: Range<usize>,
    terminator_key_range: Range<usize>,
    terminator_line_begin_index: usize,
    terminator_line_number: u32,
    terminator_operator_range: Range<usize>,
    touched: Cell<bool>,
    value_range: Option<Range<usize>>,
}

pub trait EmbedImpl {
    fn get_value(&self) -> Option<&str>;
    #[allow(clippy::new_ret_no_self, clippy::too_many_arguments)]
    fn new(
        document_internals: Rc<DocumentInternals>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
        operator_range: Range<usize>,
        terminator_key_range: Range<usize>,
        terminator_line_begin_index: usize,
        terminator_line_number: u32,
        terminator_operator_range: Range<usize>,
        value_range: Option<Range<usize>>,
    ) -> Embed;
}

impl Embed {
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
    {
        match self.get_value() {
            Some(value) => match value.parse::<T>() {
                Ok(converted) => Ok(converted),
                Err(err) => Err(Error::new(format!("{}", err), self.line_number)),
            },
            None => Err(Error::new("Value expected".to_string(), self.line_number)),
        }
    }
}

impl EmbedImpl for Embed {
    fn get_value(&self) -> Option<&str> {
        match &self.value_range {
            Some(value_range) => Some(&self.document_internals.content[value_range.clone()]),
            None => None,
        }
    }

    fn new(
        document_internals: Rc<DocumentInternals>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
        operator_range: Range<usize>,
        terminator_key_range: Range<usize>,
        terminator_line_begin_index: usize,
        terminator_line_number: u32,
        terminator_operator_range: Range<usize>,
        value_range: Option<Range<usize>>,
    ) -> Embed {
        Embed {
            document_internals,
            key_range,
            line_begin_index,
            line_number,
            operator_range,
            terminator_key_range,
            terminator_line_begin_index,
            terminator_line_number,
            terminator_operator_range,
            touched: Cell::new(false),
            value_range,
        }
    }
}

impl Element for Embed {
    fn as_embed(&self) -> Option<&Embed> {
        Some(self)
    }

    fn is_embed(&self) -> bool {
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

        out.push_str(
            &self.document_internals.content[self.line_begin_index..self.operator_range.start],
        );
        out.push_str(
            &printer.operator(&self.document_internals.content[self.operator_range.clone()]),
        );
        out.push_str(
            &self.document_internals.content[self.operator_range.end..self.key_range.start],
        );
        out.push_str(&printer.key(&self.document_internals.content[self.key_range.clone()]));
        out.push('\n');

        if let Some(value_range) = &self.value_range {
            let mut line_number = self.line_number + 1;

            for line in self.document_internals.content[value_range.clone()].lines() {
                if gutter {
                    out.push_str(&printer.gutter(line_number));
                }

                out.push_str(&printer.value(line));
                out.push('\n');

                line_number += 1;
            }
        }

        if gutter {
            out.push_str(&printer.gutter(self.terminator_line_number));
        }

        out.push_str(
            &self.document_internals.content
                [self.terminator_line_begin_index..self.terminator_operator_range.start],
        );
        out.push_str(
            &printer
                .operator(&self.document_internals.content[self.terminator_operator_range.clone()]),
        );
        out.push_str(
            &self.document_internals.content
                [self.terminator_operator_range.end..self.terminator_key_range.start],
        );
        out.push_str(
            &printer.key(&self.document_internals.content[self.terminator_key_range.clone()]),
        );

        out
    }

    fn touch(&self) {
        self.touched.set(true);
    }
}

impl ElementImpl for Embed {
    fn line_range(&self) -> RangeInclusive<u32> {
        self.line_number..=self.terminator_line_number
    }

    fn touched(&self) -> bool {
        self.touched.get()
    }
}

impl SectionElement for Embed {
    fn as_element(&self) -> &dyn Element {
        self
    }

    fn as_mut_embed(&mut self) -> Option<&mut Embed> {
        Some(self)
    }

    fn key(&self) -> &str {
        &self.document_internals.content[self.key_range.clone()]
    }
}
