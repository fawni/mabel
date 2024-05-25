use std::cell::Cell;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;
use std::str::FromStr;

use crate::elements::{DocumentInternals, ElementImpl};
use crate::{Element, Error, Printer};

#[derive(Debug)]
pub struct Item {
    document_internals: Rc<DocumentInternals>,
    line_begin_index: usize,
    pub line_number: u32,
    operator_index: usize,
    touched: Cell<bool>,
    value_range: Option<Range<usize>>,
}

#[allow(dead_code)]
pub trait ItemImpl {
    fn as_element(&self) -> &dyn Element;
    fn get_value(&self) -> Option<String>;
    #[allow(clippy::new_ret_no_self)]
    fn new(
        document_internals: Rc<DocumentInternals>,
        line_begin_index: usize,
        line_number: u32,
        operator_index: usize,
        value_range: Option<Range<usize>>,
    ) -> Item;
}

impl Item {
    pub fn optional_value<T: FromStr>(&self) -> Result<Option<T>, Error>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        match &self.value_range {
            Some(range) => {
                let value = &self.document_internals.content[range.clone()].to_owned();
                match value.parse::<T>() {
                    Ok(converted) => Ok(Some(converted)),
                    Err(err) => Err(Error::new(format!("{}", err), self.line_number)),
                }
            }
            None => Ok(None),
        }
    }

    pub fn required_value<T: FromStr>(&self) -> Result<T, Error>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        match &self.value_range {
            Some(range) => {
                let value = &self.document_internals.content[range.clone()].to_owned();
                match value.parse::<T>() {
                    Ok(converted) => Ok(converted),
                    Err(err) => Err(Error::new(format!("{}", err), self.line_number)),
                }
            }
            None => Err(Error::new("Value expected".to_string(), self.line_number)),
        }
    }
}

impl Element for Item {
    fn as_item(&self) -> Option<&Item> {
        Some(self)
    }

    fn is_item(&self) -> bool {
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

        out.push_str(&self.document_internals.content[self.line_begin_index..self.operator_index]);
        out.push_str(&printer.operator("-"));

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

impl ElementImpl for Item {
    fn line_range(&self) -> RangeInclusive<u32> {
        self.line_number..=self.line_number
    }

    fn touched(&self) -> bool {
        self.touched.get()
    }
}

impl ItemImpl for Item {
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
        line_begin_index: usize,
        line_number: u32,
        operator_index: usize,
        value_range: Option<Range<usize>>,
    ) -> Item {
        Item {
            document_internals,
            line_begin_index,
            line_number,
            operator_index,
            touched: Cell::new(false),
            value_range,
        }
    }
}
