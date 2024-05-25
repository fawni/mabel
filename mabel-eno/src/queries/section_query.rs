use crate::elements::{
    Document, Element, Embed, Field, Flag, Section, SectionElement, SectionImpl,
};
use crate::queries::{
    EmbedQuery, EmbedQueryImpl, EmbedQueryParent, FieldQuery, FieldQueryImpl, FieldQueryParent,
    FlagQuery, FlagQueryImpl, FlagQueryParent,
};
use crate::{Error, Printer};

pub enum Matches<'a, T> {
    Multiple(&'a T, &'a T),
    One(&'a T),
    None,
    WrongType(&'a dyn SectionElement),
}

pub trait SectionElements {
    fn single_embed_with_key(&self, key: &str) -> Matches<Embed>;
    fn single_field_with_key(&self, key: &str) -> Matches<Field>;
    fn single_flag_with_key(&self, key: &str) -> Matches<Flag>;
    fn single_section_with_key(&self, key: &str) -> Matches<Section>;
    fn untouched(&self) -> Vec<&dyn Element>;
}

/// Allows chained queries deep down into a document hierarchy
/// without having to manually handle possibly missing sections
/// in between. If the terminal/leaf element/value of a query chain
/// is required, the first missing element in the chain will be
/// "bubbled" up as the cause of the error.
///
/// Note that chaining still returns a `Result<*, enolib::Error>`
/// at every step, due to the fact that a query step may not only
/// fail due to a (potentially graceful) missing element, but also
/// due to ambiguous results (e.g. two sections with the same key),
/// which is an immediate hard error. Hint: Use the idiomatic approach
/// with the ? operator to create nicely readable code with this API.
pub struct SectionQuery<'a> {
    element_option: Option<&'a Section>,
    key: Option<String>,
    parent: SectionQueryParent<'a>,
}

pub trait SectionQueryImpl<'a> {
    fn element(&self) -> Option<&Section>;
    #[allow(clippy::new_ret_no_self)]
    fn new(
        element_option: Option<&'a Section>,
        key: Option<String>,
        parent: SectionQueryParent<'a>,
    ) -> SectionQuery<'a>;
}

pub enum SectionQueryParent<'a> {
    Document(&'a Document),
    Section(&'a Section),
    SectionQuery(&'a SectionQuery<'a>),
}

impl SectionElements for Vec<Box<dyn SectionElement>> {
    fn single_embed_with_key(&self, key: &str) -> Matches<Embed> {
        let mut embed_option: Option<&Embed> = None;

        for element in self.iter().filter(|element| element.key() == key) {
            match element.as_embed() {
                Some(embed) => match embed_option {
                    Some(prev_embed) => return Matches::Multiple(prev_embed, embed),
                    None => embed_option = Some(embed),
                },
                None => return Matches::WrongType(element.as_ref()),
            }
        }

        match embed_option {
            Some(embed) => {
                embed.touch();
                Matches::One(embed)
            }
            None => Matches::None,
        }
    }

    fn single_field_with_key(&self, key: &str) -> Matches<Field> {
        let mut field_option: Option<&Field> = None;

        for element in self.iter().filter(|element| element.key() == key) {
            match element.as_field() {
                Some(field) => match field_option {
                    Some(prev_field) => return Matches::Multiple(prev_field, field),
                    None => field_option = Some(field),
                },
                None => return Matches::WrongType(element.as_ref()),
            }
        }

        match field_option {
            Some(field) => {
                field.touch();
                Matches::One(field)
            }
            None => Matches::None,
        }
    }

    fn single_flag_with_key(&self, key: &str) -> Matches<Flag> {
        let mut flag_option: Option<&Flag> = None;

        for element in self.iter().filter(|element| element.key() == key) {
            match element.as_flag() {
                Some(flag) => match flag_option {
                    Some(prev_flag) => return Matches::Multiple(prev_flag, flag),
                    None => flag_option = Some(flag),
                },
                None => return Matches::WrongType(element.as_ref()),
            }
        }

        match flag_option {
            Some(flag) => {
                flag.touch();
                Matches::One(flag)
            }
            None => Matches::None,
        }
    }

    fn single_section_with_key(&self, key: &str) -> Matches<Section> {
        let mut section_option: Option<&Section> = None;

        for element in self.iter().filter(|element| element.key() == key) {
            match element.as_section() {
                Some(section) => match section_option {
                    Some(prev_section) => return Matches::Multiple(prev_section, section),
                    None => section_option = Some(section),
                },
                None => return Matches::WrongType(element.as_ref()),
            }
        }

        match section_option {
            Some(section) => {
                section.touch();
                Matches::One(section)
            }
            None => Matches::None,
        }
    }

    fn untouched(&self) -> Vec<&dyn Element> {
        let mut result = Vec::new();

        for element in self {
            if !element.touched() {
                result.push(element.as_element());
            } else if let Some(field) = element.as_field() {
                result.append(&mut field.untouched_elements());
            } else if let Some(section) = element.as_section() {
                result.append(&mut section.untouched_elements());
            }
        }

        result
    }
}

impl<'a> SectionQuery<'a> {
    pub fn elements(&self) -> &[Box<dyn SectionElement>] {
        match self.element_option {
            Some(section) => section.get_elements(),
            None => &[],
        }
    }

    pub fn embed(&self, key: &str) -> Result<EmbedQuery, Error> {
        let element_option = match self.element_option {
            Some(section) => match section.get_elements_ref().single_embed_with_key(key) {
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
            },
            None => None,
        };

        Ok(EmbedQuery::new(
            element_option,
            Some(key.to_string()),
            EmbedQueryParent::SectionQuery(self),
        ))
    }

    pub fn field(&self, key: &str) -> Result<FieldQuery, Error> {
        let element_option = match self.element_option {
            Some(section) => match section.get_elements_ref().single_field_with_key(key) {
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
            },
            None => None,
        };

        Ok(FieldQuery::new(
            element_option,
            Some(key.to_string()),
            FieldQueryParent::SectionQuery(self),
        ))
    }

    pub fn flag(&self, key: &str) -> Result<FlagQuery, Error> {
        let element_option = match self.element_option {
            Some(section) => match section.get_elements_ref().single_flag_with_key(key) {
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
            },
            None => None,
        };

        Ok(FlagQuery::new(
            element_option,
            Some(key.to_string()),
            FlagQueryParent::SectionQuery(self),
        ))
    }

    pub fn missing_error(&self) -> Error {
        match self.parent {
            SectionQueryParent::Document(_) => Error::new(
                format!(
                    "Section {} not found",
                    self.key.as_deref().unwrap_or("(can have any key)")
                ),
                Document::LINE_NUMBER,
            ),
            SectionQueryParent::Section(section) => Error::new(
                format!(
                    "Section {} not found",
                    self.key.as_deref().unwrap_or("(can have any key)")
                ),
                section.line_number,
            ),
            SectionQueryParent::SectionQuery(section_query) => match section_query.element_option {
                Some(section) => Error::new(
                    format!(
                        "Section {} not found",
                        self.key.as_deref().unwrap_or("(can have any key)")
                    ),
                    section.line_number,
                ),
                None => section_query.missing_error(),
            },
        }
    }

    pub fn section(&self, key: &str) -> Result<SectionQuery, Error> {
        let element_option = match self.element_option {
            Some(section) => match section.get_elements_ref().single_section_with_key(key) {
                Matches::None => None,
                Matches::One(subsection) => Some(subsection),
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
            },
            None => None,
        };

        Ok(SectionQuery::new(
            element_option,
            Some(key.to_string()),
            SectionQueryParent::SectionQuery(self),
        ))
    }

    pub fn snippet(&self) -> Result<String, Error> {
        match self.element_option {
            Some(section) => Ok(section.snippet()),
            None => Err(self.missing_error()),
        }
    }

    pub fn snippet_with_options(
        &self,
        printer: &dyn Printer,
        gutter: bool,
    ) -> Result<String, Error> {
        match self.element_option {
            Some(section) => Ok(section.snippet_with_options(printer, gutter)),
            None => Err(self.missing_error()),
        }
    }
}

impl<'a> SectionQueryImpl<'a> for SectionQuery<'a> {
    fn element(&self) -> Option<&Section> {
        self.element_option
    }

    fn new(
        element_option: Option<&'a Section>,
        key: Option<String>,
        parent: SectionQueryParent<'a>,
    ) -> SectionQuery<'a> {
        SectionQuery {
            element_option,
            key,
            parent,
        }
    }
}
