use std::cell::Cell;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;
use std::str::FromStr;

use crate::elements::{AttributeImpl, CommentImpl, DocumentInternals, ElementImpl, ItemImpl};
use crate::{Attribute, Element, Error, Item, Printer, SectionElement};

#[derive(Debug)]
pub struct Field {
    content: FieldContent,
    document_internals: Rc<DocumentInternals>,
    escape_operator_ranges: Option<(Range<usize>, Range<usize>)>,
    key_range: Range<usize>,
    line_begin_index: usize,
    pub line_number: u32,
    operator_index: usize,
    touched: Cell<bool>,
}

#[derive(Debug)]
pub enum FieldContent {
    Attributes(Vec<Attribute>),
    Items(Vec<Item>),
    None,
    Value(Range<usize>),
}

pub trait FieldImpl {
    fn get_content(&self) -> &FieldContent;
    fn get_content_mut(&mut self) -> &mut FieldContent;
    fn get_document_content(&self) -> &str;
    #[allow(clippy::new_ret_no_self)]
    fn new(
        content: FieldContent,
        document_internals: Rc<DocumentInternals>,
        escape_operator_ranges: Option<(Range<usize>, Range<usize>)>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
        operator_index: usize,
    ) -> Field;
    fn set_content(&mut self, content: FieldContent);
}

impl Field {
    pub fn attributes(&self) -> Result<&[Attribute], Error> {
        match &self.content {
            FieldContent::Attributes(attributes) => {
                for attribute in attributes.iter() {
                    attribute.touch();
                }
                Ok(attributes.as_slice())
            }
            FieldContent::Items(_) => Err(Error::new(
                "Attributes expected, Items found".to_string(),
                self.line_number,
            )),
            FieldContent::Value { .. } => Err(Error::new(
                "Attributes expected, Value found".to_string(),
                self.line_number,
            )),
            FieldContent::None => Ok(&[]),
        }
    }

    pub fn items(&self) -> Result<&[Item], Error> {
        match &self.content {
            FieldContent::Attributes(_) => Err(Error::new(
                "Items expected, Attributes found".to_string(),
                self.line_number,
            )),
            FieldContent::Items(items) => {
                for item in items.iter() {
                    item.touch();
                }
                Ok(items.as_slice())
            }
            FieldContent::Value { .. } => Err(Error::new(
                "Items expected, Value found".to_string(),
                self.line_number,
            )),
            FieldContent::None => Ok(&[]),
        }
    }

    pub fn optional_value(&self) -> Result<Option<String>, Error> {
        match &self.content {
            FieldContent::Attributes(attributes) => Err(Error::new(
                "Value expected, Attributes found".to_string(),
                attributes[0].line_number,
            )),
            FieldContent::Items(items) => Err(Error::new(
                "Value expected, Items found".to_string(),
                items[0].line_number,
            )),
            FieldContent::Value(range) => Ok(Some(
                self.document_internals.content[range.clone()].to_owned(),
            )),
            FieldContent::None => Ok(None),
        }
    }

    pub fn required_attribute(&self, key: &str) -> Result<&Attribute, Error> {
        match &self.content {
            FieldContent::Attributes(attributes) => {
                let mut attribute_option = None;

                for attribute in attributes {
                    if attribute.key() == key {
                        match attribute_option {
                            Some(_) => {
                                return Err(Error::new(
                                    format!("Multiple attributes with key {} found", key),
                                    self.line_number,
                                ))
                            }
                            None => attribute_option = Some(attribute),
                        }
                    }
                }

                match attribute_option {
                    Some(attribute) => {
                        attribute.touch();
                        Ok(attribute)
                    }
                    None => Err(Error::new(
                        format!("Missing attribute {}", key),
                        self.line_number,
                    )),
                }
            }
            FieldContent::Items(_) => Err(Error::new(
                "Attributes expected, Items found".to_string(),
                self.line_number,
            )),
            FieldContent::Value { .. } => Err(Error::new(
                "Attributes expected, Value found".to_string(),
                self.line_number,
            )),
            FieldContent::None => Err(Error::new(
                format!("Missing attribute {}", key),
                self.line_number,
            )),
        }
    }

    pub fn required_value<T: FromStr>(&self) -> Result<T, Error>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        match &self.content {
            FieldContent::Attributes(_) => Err(Error::new(
                "Value expected, Attributes found".to_string(),
                self.line_number,
            )),
            FieldContent::Items(_) => Err(Error::new(
                "Value expected, Items found".to_string(),
                self.line_number,
            )),
            FieldContent::Value(range) => {
                let value = &self.document_internals.content[range.clone()].to_owned();
                match value.parse::<T>() {
                    Ok(converted) => Ok(converted),
                    Err(err) => Err(Error::new(format!("{}", err), self.line_number)),
                }
            }
            FieldContent::None => Err(Error::new("Missing value".to_string(), self.line_number)),
        }
    }

    pub fn untouched_elements(&self) -> Vec<&dyn Element> {
        match &self.content {
            FieldContent::Attributes(attributes) => attributes
                .iter()
                .filter(|attribute| !attribute.touched())
                .map(|attribute| attribute.as_element())
                .collect(),
            FieldContent::Items(items) => items
                .iter()
                .filter(|item| !item.touched())
                .map(|item| item.as_element())
                .collect(),
            _ => Vec::new(),
        }
    }
}

impl Element for Field {
    fn as_field(&self) -> Option<&Field> {
        Some(self)
    }

    fn is_field(&self) -> bool {
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

        out.push_str(&self.document_internals.content[self.key_range.end..self.operator_index]);
        out.push_str(&printer.operator(":"));

        let mut line_number = self.line_number + 1;

        match &self.content {
            FieldContent::Attributes(attributes) => {
                for attribute in attributes {
                    out.push('\n');

                    let attribute_line_range = attribute.line_range();

                    while line_number < *attribute_line_range.start() {
                        if let Some(comment) = self
                            .document_internals
                            .comments
                            .borrow()
                            .iter()
                            .find(|comment| comment.line_number == line_number)
                        {
                            out.push_str(&comment.snippet_with_options(printer, gutter));
                        } else if gutter {
                            out.push_str(&printer.gutter(line_number));
                        }

                        out.push('\n');

                        line_number += 1;
                    }

                    out.push_str(&attribute.snippet_with_options(printer, gutter));

                    line_number = *attribute_line_range.end() + 1;
                }
            }
            FieldContent::Items(items) => {
                for item in items {
                    out.push('\n');

                    let item_line_range = item.line_range();

                    while line_number < *item_line_range.start() {
                        if let Some(comment) = self
                            .document_internals
                            .comments
                            .borrow()
                            .iter()
                            .find(|comment| comment.line_number == line_number)
                        {
                            out.push_str(&comment.snippet_with_options(printer, gutter));
                        } else if gutter {
                            out.push_str(&printer.gutter(line_number));
                        }

                        out.push('\n');

                        line_number += 1;
                    }

                    out.push_str(&item.snippet_with_options(printer, gutter));

                    line_number = *item_line_range.end() + 1;
                }
            }
            FieldContent::Value(value_range) => {
                out.push_str(
                    &self.document_internals.content[(self.operator_index + 1)..value_range.start],
                );
                out.push_str(&printer.value(&self.document_internals.content[value_range.clone()]));
            }
            FieldContent::None => (),
        }

        out
    }

    fn touch(&self) {
        self.touched.set(true);
    }
}

impl ElementImpl for Field {
    fn line_range(&self) -> RangeInclusive<u32> {
        let end = match &self.content {
            FieldContent::Attributes(attributes) => *attributes.last().unwrap().line_range().end(),
            FieldContent::Items(items) => *items.last().unwrap().line_range().end(),
            FieldContent::Value(_) | FieldContent::None => self.line_number,
        };

        self.line_number..=end
    }

    fn touched(&self) -> bool {
        self.touched.get()
    }
}

impl FieldImpl for Field {
    fn get_content(&self) -> &FieldContent {
        &self.content
    }

    fn get_content_mut(&mut self) -> &mut FieldContent {
        &mut self.content
    }

    fn get_document_content(&self) -> &str {
        &self.document_internals.content
    }

    fn new(
        content: FieldContent,
        document_internals: Rc<DocumentInternals>,
        escape_operator_ranges: Option<(Range<usize>, Range<usize>)>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
        operator_index: usize,
    ) -> Field {
        Field {
            content,
            document_internals,
            escape_operator_ranges,
            key_range,
            line_begin_index,
            line_number,
            operator_index,
            touched: Cell::new(false),
        }
    }

    fn set_content(&mut self, content: FieldContent) {
        self.content = content;
    }
}

impl SectionElement for Field {
    fn as_element(&self) -> &dyn Element {
        self
    }

    fn as_mut_field(&mut self) -> Option<&mut Field> {
        Some(self)
    }

    fn key(&self) -> &str {
        &self.document_internals.content[self.key_range.clone()]
    }
}
