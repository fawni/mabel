use std::cell::RefCell;
use std::ops::RangeInclusive;
use std::rc::Rc;

use crate::elements::{Comment, CommentImpl};
use crate::queries::{
    EmbedQuery, EmbedQueryImpl, EmbedQueryParent, FieldQuery, FieldQueryImpl, FieldQueryParent,
    FlagQuery, FlagQueryImpl, FlagQueryParent, Matches, SectionElements, SectionQuery,
    SectionQueryImpl, SectionQueryParent,
};
use crate::{Element, Embed, Error, Field, Flag, Printer, Section, SectionElement};

#[derive(Debug)]
pub struct Document {
    document_internals: Rc<DocumentInternals>,
    elements: Vec<Box<dyn SectionElement>>,
    number_of_lines: u32,
}

#[derive(Debug)]
pub struct DocumentInternals {
    pub comments: RefCell<Vec<Comment>>,
    pub content: String,
    pub default_printer: Box<dyn Printer>,
}

pub trait DocumentImpl {
    fn append_comment(&mut self, comment: Comment);
    fn append_elements(&mut self, elements: &mut Vec<Box<dyn SectionElement>>);
    fn clone_internals(&self) -> Rc<DocumentInternals>;
    fn get_elements(&self) -> &[Box<dyn SectionElement>];
    fn get_elements_mut(&mut self) -> &mut Vec<Box<dyn SectionElement>>;
    fn get_number_of_lines(&self) -> u32;
    #[allow(clippy::new_ret_no_self)]
    fn new(content: &str, default_printer: Box<dyn Printer>) -> Document;
    fn set_number_of_lines(&mut self, number_of_lines: u32);
}

impl Document {
    pub const LINE_NUMBER: u32 = 1;

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
                    "Only a single embed was expected".to_owned(),
                    second.line_number,
                ))
            }
            Matches::WrongType(element) => {
                return Err(Error::new(
                    "An embed was expected".to_owned(),
                    element.line_number(),
                ))
            }
        };

        // TODO: Revisit whether Some(key.to_string()) ever makes sense (as in: does the key need to be an option? Isn't it always present maybe?)
        Ok(EmbedQuery::new(
            element_option,
            Some(key.to_string()),
            EmbedQueryParent::Document(self),
        ))
    }

    pub fn field(&self, key: &str) -> Result<FieldQuery, Error> {
        let element_option = match self.elements.single_field_with_key(key) {
            Matches::None => None,
            Matches::One(field) => Some(field),
            Matches::Multiple(_first, second) => {
                return Err(Error::new(
                    "Only a single field was expected".to_owned(),
                    second.line_number,
                ))
            }
            Matches::WrongType(element) => {
                return Err(Error::new(
                    "A field was expected".to_owned(),
                    element.line_number(),
                ))
            }
        };

        Ok(FieldQuery::new(
            element_option,
            Some(key.to_string()),
            FieldQueryParent::Document(self),
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
            FlagQueryParent::Document(self),
        ))
    }

    pub fn line_range(&self) -> RangeInclusive<u32> {
        1..=self.number_of_lines
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
            Matches::One(section) => Ok(Some(section)),
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

    pub fn required_section(&self, key: &str) -> Result<&Section, Error> {
        match self.elements.single_section_with_key(key) {
            Matches::None => Err(Error::new("Not found".to_string(), Document::LINE_NUMBER)),
            Matches::One(section) => Ok(section),
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
            SectionQueryParent::Document(self),
        ))
    }

    pub fn snippet(&self) -> String {
        self.snippet_with_options(&*self.document_internals.default_printer, true)
    }

    pub fn snippet_with_options(&self, printer: &dyn Printer, gutter: bool) -> String {
        let mut out = String::new();
        let mut line_number = 1;

        for element in &self.elements {
            if line_number > 1 {
                out.push('\n');
            }

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

        while line_number <= self.number_of_lines {
            if line_number > 1 {
                out.push('\n');
            }

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

            line_number += 1;
        }

        out
    }

    pub fn untouched_elements(&self) -> Vec<&dyn Element> {
        self.elements.untouched()
    }
}

impl DocumentImpl for Document {
    fn append_comment(&mut self, comment: Comment) {
        self.document_internals.comments.borrow_mut().push(comment);
    }

    fn append_elements(&mut self, elements: &mut Vec<Box<dyn SectionElement>>) {
        self.elements.append(elements);
    }

    fn clone_internals(&self) -> Rc<DocumentInternals> {
        self.document_internals.clone()
    }

    fn get_elements(&self) -> &[Box<dyn SectionElement>] {
        self.elements.as_slice()
    }

    fn get_elements_mut(&mut self) -> &mut Vec<Box<dyn SectionElement>> {
        &mut self.elements
    }

    fn get_number_of_lines(&self) -> u32 {
        self.number_of_lines
    }

    fn new(content: &str, default_printer: Box<dyn Printer>) -> Document {
        Document {
            document_internals: Rc::new(DocumentInternals::new(content, default_printer)),
            elements: Vec::new(),
            number_of_lines: 1,
        }
    }

    fn set_number_of_lines(&mut self, number_of_lines: u32) {
        self.number_of_lines = number_of_lines;
    }
}

impl DocumentInternals {
    fn new(content: &str, default_printer: Box<dyn Printer>) -> DocumentInternals {
        DocumentInternals {
            comments: RefCell::new(Vec::new()),
            content: content.to_owned(),
            default_printer,
        }
    }
}
