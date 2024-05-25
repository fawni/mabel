use std::cell::Cell;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;

use crate::elements::{DocumentInternals, ElementImpl};
use crate::{Element, Printer, SectionElement};

#[derive(Debug)]
pub struct Flag {
    document_internals: Rc<DocumentInternals>,
    escape_operator_ranges: Option<(Range<usize>, Range<usize>)>,
    key_range: Range<usize>,
    line_begin_index: usize,
    pub line_number: u32,
    touched: Cell<bool>,
}

pub trait FlagImpl {
    #[allow(clippy::new_ret_no_self)]
    fn new(
        document_internals: Rc<DocumentInternals>,
        escape_operator_ranges: Option<(Range<usize>, Range<usize>)>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
    ) -> Flag;
}

impl Element for Flag {
    fn as_flag(&self) -> Option<&Flag> {
        Some(self)
    }

    fn is_flag(&self) -> bool {
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
        } else {
            out.push_str(
                &self.document_internals.content[self.line_begin_index..self.key_range.start],
            );
            out.push_str(&printer.key(&self.document_internals.content[self.key_range.clone()]));
        }

        out
    }

    fn touch(&self) {
        self.touched.set(true);
    }
}

impl ElementImpl for Flag {
    fn line_range(&self) -> RangeInclusive<u32> {
        self.line_number..=self.line_number
    }

    fn touched(&self) -> bool {
        self.touched.get()
    }
}

impl FlagImpl for Flag {
    fn new(
        document_internals: Rc<DocumentInternals>,
        escape_operator_ranges: Option<(Range<usize>, Range<usize>)>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
    ) -> Flag {
        Flag {
            document_internals,
            escape_operator_ranges,
            key_range,
            line_begin_index,
            line_number,
            touched: Cell::new(false),
        }
    }
}

impl SectionElement for Flag {
    fn as_element(&self) -> &dyn Element {
        self
    }

    fn as_mut_flag(&mut self) -> Option<&mut Flag> {
        Some(self)
    }

    fn key(&self) -> &str {
        &self.document_internals.content[self.key_range.clone()]
    }
}
