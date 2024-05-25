use std::cell::Cell;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;

use crate::elements::{CommentImpl, DocumentInternals, ElementImpl};
use crate::queries::{
    EmbedQuery, EmbedQueryImpl, EmbedQueryParent, FieldQuery, FieldQueryImpl, FieldQueryParent,
    FlagQuery, FlagQueryImpl, FlagQueryParent, Matches, SectionElements, SectionQuery,
    SectionQueryImpl, SectionQueryParent,
};
use crate::{Element, Embed, Error, Field, Flag, Printer};

#[derive(Debug)]
pub struct Section {
    document_internals: Rc<DocumentInternals>,
    elements: Vec<Box<dyn SectionElement>>,
    key_range: Range<usize>,
    line_begin_index: usize,
    pub line_number: u32,
    operator_range: Range<usize>,
    touched: Cell<bool>,
}

pub trait SectionElement: std::fmt::Debug + Element + ElementImpl {
    fn as_element(&self) -> &dyn Element;

    // TODO: Expose through an additional private trait
    fn as_mut_embed(&mut self) -> Option<&mut Embed> {
        None
    }
    fn as_mut_field(&mut self) -> Option<&mut Field> {
        None
    }
    fn as_mut_flag(&mut self) -> Option<&mut Flag> {
        None
    }
    fn as_mut_section(&mut self) -> Option<&mut Section> {
        None
    }

    fn key(&self) -> &str;
}

pub trait SectionImpl {
    fn get_elements(&self) -> &[Box<dyn SectionElement>];
    fn get_elements_mut(&mut self) -> &mut Vec<Box<dyn SectionElement>>;
    fn get_elements_ref(&self) -> &Vec<Box<dyn SectionElement>>;
    #[allow(clippy::new_ret_no_self)]
    fn new(
        document_internals: Rc<DocumentInternals>,
        elements: Vec<Box<dyn SectionElement>>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
        operator_range: Range<usize>,
    ) -> Section;
}

impl Section {
    pub fn elements(&self) -> &[Box<dyn SectionElement>] {
        for element in &self.elements {
            element.touch();
        }
        self.elements.as_slice()
    }

    pub fn embed(&self, key: &str) -> Result<EmbedQuery, Error> {
        let element_option = match self.elements.single_embed_with_key(key) {
            Matches::None => None,
            Matches::One(embed) => Some(embed),
            Matches::Multiple(_first, second) => {
                return Err(Error::new(
                    "Only a single embed was expected".to_string(),
                    second.line_number,
                ))
            }
            Matches::WrongType(element) => {
                return Err(Error::new(
                    "An embed was expected".to_string(),
                    element.line_number(),
                ))
            }
        };

        // TODO: Revisit whether Some(key.to_string()) ever makes sense (as in: does the key need to be an option? Isn't it always present maybe?)
        Ok(EmbedQuery::new(
            element_option,
            Some(key.to_string()),
            EmbedQueryParent::Section(self),
        ))
    }

    pub fn field(&self, key: &str) -> Result<FieldQuery, Error> {
        let element_option = match self.elements.single_field_with_key(key) {
            Matches::None => None,
            Matches::One(field) => Some(field),
            Matches::Multiple(_first, second) => {
                return Err(Error::new(
                    "Only a single field was expected".to_string(),
                    second.line_number,
                ))
            }
            Matches::WrongType(element) => {
                return Err(Error::new(
                    "A field was expected".to_string(),
                    element.line_number(),
                ))
            }
        };

        Ok(FieldQuery::new(
            element_option,
            Some(key.to_string()),
            FieldQueryParent::Section(self),
        ))
    }

    pub fn flag(&self, key: &str) -> Result<FlagQuery, Error> {
        let element_option = match self.elements.single_flag_with_key(key) {
            Matches::None => None,
            Matches::One(flag) => Some(flag),
            Matches::Multiple(_first, second) => {
                return Err(Error::new(
                    "Only a single flag was expected".to_string(),
                    second.line_number,
                ))
            }
            Matches::WrongType(element) => {
                return Err(Error::new(
                    "A flag was expected".to_string(),
                    element.line_number(),
                ))
            }
        };

        Ok(FlagQuery::new(
            element_option,
            Some(key.to_string()),
            FlagQueryParent::Section(self),
        ))
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn optional_embed(&self, key: &str) -> Result<Option<&Embed>, Error> {
        match self.elements.single_embed_with_key(key) {
            Matches::None => Ok(None),
            Matches::One(embed) => {
                embed.touch();
                Ok(Some(embed))
            }
            Matches::Multiple(_first, second) => Err(Error::new(
                "Only a single embed was expected".to_string(),
                second.line_number,
            )),
            Matches::WrongType(element) => Err(Error::new(
                "A embed was expected".to_string(),
                element.line_number(),
            )),
        }
    }

    pub fn optional_field(&self, key: &str) -> Result<Option<&Field>, Error> {
        match self.elements.single_field_with_key(key) {
            Matches::None => Ok(None),
            Matches::One(field) => {
                field.touch();
                Ok(Some(field))
            }
            Matches::Multiple(_first, second) => Err(Error::new(
                "Only a single field was expected".to_string(),
                second.line_number,
            )),
            Matches::WrongType(element) => Err(Error::new(
                "A field was expected".to_string(),
                element.line_number(),
            )),
        }
    }

    pub fn optional_flag(&self, key: &str) -> Result<Option<&Flag>, Error> {
        match self.elements.single_flag_with_key(key) {
            Matches::None => Ok(None),
            Matches::One(flag) => {
                flag.touch();
                Ok(Some(flag))
            }
            Matches::Multiple(_first, second) => Err(Error::new(
                "Only a single flag was expected".to_string(),
                second.line_number,
            )),
            Matches::WrongType(element) => Err(Error::new(
                "A flag was expected".to_string(),
                element.line_number(),
            )),
        }
    }

    pub fn optional_section(&self, key: &str) -> Result<Option<&Section>, Error> {
        match self.elements.single_section_with_key(key) {
            Matches::None => Ok(None),
            Matches::One(section) => {
                section.touch();
                Ok(Some(section))
            }
            Matches::Multiple(_first, second) => Err(Error::new(
                "Only a single section was expected".to_string(),
                second.line_number,
            )),
            Matches::WrongType(element) => Err(Error::new(
                "A section was expected".to_string(),
                element.line_number(),
            )),
        }
    }

    pub fn required_embed(&self, key: &str) -> Result<&Embed, Error> {
        match self.elements.single_embed_with_key(key) {
            Matches::None => Err(Error::new("Not found".to_string(), self.line_number)),
            Matches::One(embed) => {
                embed.touch();
                Ok(embed)
            }
            Matches::Multiple(_first, second) => Err(Error::new(
                format!("Only a single embed with key {} was expected", key),
                second.line_number,
            )),
            Matches::WrongType(element) => Err(Error::new(
                "A embed was expected".to_string(),
                element.line_number(),
            )),
        }
    }

    pub fn required_field(&self, key: &str) -> Result<&Field, Error> {
        match self.elements.single_field_with_key(key) {
            Matches::None => Err(Error::new("Not found".to_string(), self.line_number)),
            Matches::One(field) => {
                field.touch();
                Ok(field)
            }
            Matches::Multiple(_first, second) => Err(Error::new(
                format!("Only a single field with key {} was expected", key),
                second.line_number,
            )),
            Matches::WrongType(element) => Err(Error::new(
                "A field was expected".to_string(),
                element.line_number(),
            )),
        }
    }

    pub fn required_flag(&self, key: &str) -> Result<&Flag, Error> {
        match self.elements.single_flag_with_key(key) {
            Matches::None => Err(Error::new("Not found".to_string(), self.line_number)),
            Matches::One(flag) => {
                flag.touch();
                Ok(flag)
            }
            Matches::Multiple(_first, second) => Err(Error::new(
                format!("Only a single flag with key {} was expected", key),
                second.line_number,
            )),
            Matches::WrongType(element) => Err(Error::new(
                "A flag was expected".to_string(),
                element.line_number(),
            )),
        }
    }

    pub fn required_section(&self, key: &str) -> Result<&Section, Error> {
        match self.elements.single_section_with_key(key) {
            Matches::None => Err(Error::new("Not found".to_string(), self.line_number)),
            Matches::One(section) => {
                section.touch();
                Ok(section)
            }
            Matches::Multiple(_first, second) => Err(Error::new(
                format!("Only a single section with key {} was expected", key),
                second.line_number,
            )),
            Matches::WrongType(element) => Err(Error::new(
                "A section was expected".to_string(),
                element.line_number(),
            )),
        }
    }

    pub fn section(&self, key: &str) -> Result<SectionQuery, Error> {
        let element_option = match self.elements.single_section_with_key(key) {
            Matches::None => None,
            Matches::One(section) => Some(section),
            Matches::Multiple(_first, second) => {
                return Err(Error::new(
                    "Only a single section was expected".to_string(),
                    second.line_number,
                ))
            }
            Matches::WrongType(element) => {
                return Err(Error::new(
                    "A section was expected".to_string(),
                    element.line_number(),
                ))
            }
        };

        Ok(SectionQuery::new(
            element_option,
            Some(key.to_string()),
            SectionQueryParent::Section(self),
        ))
    }

    pub fn untouched_elements(&self) -> Vec<&dyn Element> {
        self.elements.untouched()
    }
}

impl Element for Section {
    fn as_section(&self) -> Option<&Section> {
        Some(self)
    }

    fn is_section(&self) -> bool {
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

        let mut line_number = self.line_number + 1;

        for element in &self.elements {
            out.push('\n');

            let element_line_range = element.line_range();

            while line_number < *element_line_range.start() {
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

            out.push_str(&element.snippet_with_options(printer, gutter));

            line_number = *element_line_range.end() + 1;
        }

        out
    }

    fn touch(&self) {
        self.touched.set(true);
    }
}

impl ElementImpl for Section {
    fn line_range(&self) -> RangeInclusive<u32> {
        match self.elements.last() {
            Some(element) => self.line_number..=*element.line_range().end(),
            None => self.line_number..=self.line_number,
        }
    }

    fn touched(&self) -> bool {
        self.touched.get()
    }
}

impl SectionElement for Section {
    fn as_element(&self) -> &dyn Element {
        self
    }

    fn as_mut_section(&mut self) -> Option<&mut Section> {
        Some(self)
    }

    fn key(&self) -> &str {
        &self.document_internals.content[self.key_range.clone()]
    }
}

impl SectionImpl for Section {
    fn get_elements(&self) -> &[Box<dyn SectionElement>] {
        self.elements.as_slice()
    }

    fn get_elements_mut(&mut self) -> &mut Vec<Box<dyn SectionElement>> {
        &mut self.elements
    }

    fn get_elements_ref(&self) -> &Vec<Box<dyn SectionElement>> {
        &self.elements
    }

    fn new(
        document_internals: Rc<DocumentInternals>,
        elements: Vec<Box<dyn SectionElement>>,
        key_range: Range<usize>,
        line_begin_index: usize,
        line_number: u32,
        operator_range: Range<usize>,
    ) -> Section {
        Section {
            document_internals,
            elements,
            key_range,
            line_begin_index,
            line_number,
            operator_range,
            touched: Cell::new(false),
        }
    }
}
