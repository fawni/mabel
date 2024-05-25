use std::str::FromStr;

use crate::elements::{Document, Element, Field, FieldContent, FieldImpl, Item, Section};
use crate::queries::{AttributeQuery, AttributeQueryImpl, SectionQuery, SectionQueryImpl};
use crate::{Error, Printer};

pub struct FieldQuery<'a> {
    element_option: Option<&'a Field>,
    key: Option<String>,
    parent: FieldQueryParent<'a>,
}

pub trait FieldQueryImpl<'a> {
    fn element(&self) -> Option<&Field>;
    #[allow(clippy::new_ret_no_self)]
    fn new(
        element_option: Option<&'a Field>,
        key: Option<String>,
        parent: FieldQueryParent<'a>,
    ) -> FieldQuery<'a>;
}

pub enum FieldQueryParent<'a> {
    Document(&'a Document),
    Section(&'a Section),
    SectionQuery(&'a SectionQuery<'a>),
}

impl<'a> FieldQuery<'a> {
    pub fn attribute(&self, key: &str) -> Result<AttributeQuery, Error> {
        let element_option = match self.element_option {
            Some(field) => match &field.get_content() {
                FieldContent::Attributes(attributes) => {
                    let mut attribute_option = None;
                    for attribute in attributes {
                        if attribute.key() == key {
                            match attribute_option {
                                Some(_) => {
                                    return Err(Error::new(
                                        format!("Multiple attributes with key {} found", key),
                                        attribute.line_number,
                                    ))
                                }
                                None => {
                                    attribute.touch();
                                    attribute_option = Some(attribute);
                                }
                            }
                        }
                    }
                    attribute_option
                }
                FieldContent::Items(items) => {
                    return Err(Error::new(
                        "Attributes expected, Items found".to_string(),
                        items[0].line_number,
                    ))
                }
                FieldContent::Value { .. } => {
                    return Err(Error::new(
                        "Attributes expected, Value found".to_string(),
                        field.line_number,
                    ))
                }
                FieldContent::None => None,
            },
            None => None,
        };

        if let Some(element) = element_option {
            element.touch();
        }

        Ok(AttributeQuery::new(
            element_option,
            Some(key.to_string()),
            self,
        ))
    }

    pub fn attributes(&self) -> Result<Vec<crate::Attribute>, Error> {
        match self.element_option {
            Some(field) => match field.get_content() {
                FieldContent::Attributes(attributes) => Ok(attributes.to_vec()),
                FieldContent::None => Ok(vec![]),
                FieldContent::Items(items) => Err(Error::new(
                    "Attributes expected, Items found".to_owned(),
                    items[0].line_number,
                )),
                FieldContent::Value { .. } => Err(Error::new(
                    "Attributes expected, Value found".to_owned(),
                    field.line_number,
                )),
            },
            None => Ok(vec![]),
        }
    }

    pub fn items(&self) -> Result<&[Item], Error> {
        match self.element_option {
            Some(field) => match &field.get_content() {
                FieldContent::Attributes(attributes) => Err(Error::new(
                    "Items expected, Attributes found".to_string(),
                    attributes[0].line_number,
                )),
                FieldContent::Items(items) => {
                    for item in items.iter() {
                        item.touch();
                    }
                    Ok(items.as_slice())
                }
                FieldContent::Value { .. } => Err(Error::new(
                    "Items expected, Value found".to_string(),
                    field.line_number,
                )),
                FieldContent::None => Ok(&[]),
            },
            None => Ok(&[]),
        }
    }

    pub fn missing_error(&self) -> Error {
        match self.parent {
            FieldQueryParent::Document(_) => Error::new(
                format!(
                    "Field {} not found",
                    self.key.as_deref().unwrap_or("(can have any key)")
                ),
                Document::LINE_NUMBER,
            ),
            FieldQueryParent::Section(section) => Error::new(
                format!(
                    "Field {} not found",
                    self.key.as_deref().unwrap_or("(can have any key)")
                ),
                section.line_number,
            ),
            FieldQueryParent::SectionQuery(section_query) => match section_query.element() {
                Some(section) => Error::new(
                    format!(
                        "Field {} not found",
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
            Some(field) => match &field.get_content() {
                FieldContent::Attributes(attributes) => Err(Error::new(
                    "Value expected, Attributes found".to_string(),
                    attributes[0].line_number,
                )),
                FieldContent::Items(items) => Err(Error::new(
                    "Value expected, Items found".to_string(),
                    items[0].line_number,
                )),
                FieldContent::Value(range) => {
                    Ok(Some(field.get_document_content()[range.clone()].to_owned()))
                }
                FieldContent::None => Ok(None),
            },
            None => Ok(None),
        }
    }

    pub fn required_value<T: FromStr>(&self) -> Result<T, Error>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        match &self.element_option {
            Some(field) => match &field.get_content() {
                FieldContent::Attributes(attributes) => Err(Error::new(
                    "Value expected, Attributes found".to_string(),
                    attributes[0].line_number,
                )),
                FieldContent::Items(items) => Err(Error::new(
                    "Value expected, Items found".to_string(),
                    items[0].line_number,
                )),
                FieldContent::Value(range) => {
                    let value = field.get_document_content()[range.clone()].to_owned();
                    match value.parse::<T>() {
                        Ok(converted) => Ok(converted),
                        Err(err) => Err(Error::new(format!("{}", err), field.line_number)),
                    }
                }
                FieldContent::None => {
                    Err(Error::new("Missing value".to_string(), field.line_number))
                }
            },
            None => Err(self.missing_error()),
        }
    }

    pub fn snippet(&self) -> Result<String, Error> {
        match self.element_option {
            Some(field) => Ok(field.snippet()),
            None => Err(self.missing_error()),
        }
    }

    pub fn snippet_with_options(
        &self,
        printer: &dyn Printer,
        gutter: bool,
    ) -> Result<String, Error> {
        match self.element_option {
            Some(field) => Ok(field.snippet_with_options(printer, gutter)),
            None => Err(self.missing_error()),
        }
    }
}

impl<'a> FieldQueryImpl<'a> for FieldQuery<'a> {
    fn element(&self) -> Option<&Field> {
        self.element_option
    }

    fn new(
        element_option: Option<&'a Field>,
        key: Option<String>,
        parent: FieldQueryParent<'a>,
    ) -> FieldQuery<'a> {
        FieldQuery {
            element_option,
            key,
            parent,
        }
    }
}
