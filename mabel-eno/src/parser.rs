use std::iter::Peekable;
use std::marker::PhantomData;
use std::mem;
use std::ops::Range;
use std::str::Chars;

use crate::elements::{
    AttributeImpl, Comment, CommentImpl, DocumentImpl, EmbedImpl, FieldContent, FieldImpl,
    FlagImpl, ItemImpl, SectionImpl,
};
use crate::{
    error::{self, Error},
    locale::Locale,
    Attribute, Document, Embed, Field, Flag, Item, Printer, Section, SectionElement,
};

pub struct Parser<'input, L: Locale> {
    pub chars: Peekable<Chars<'input>>,
    pub document: Document,
    pub line_begin_index: usize,
    pub line_number: u32,
    pub index: usize,
    pub input: &'input str,
    pub section_depth: usize,
    pub section_elements: Vec<Box<dyn SectionElement>>,
    pub section_key_range: Range<usize>,
    pub section_line_begin_index: usize,
    pub section_line_number: u32,
    pub section_operator_range: Range<usize>,
    locale_type: PhantomData<L>,
}

impl<'input, L: Locale> Parser<'input, L> {
    fn attach_section_elements(&mut self) {
        if self.section_depth == 0 {
            self.document.append_elements(&mut self.section_elements);
        } else {
            let section = Section::new(
                self.document.clone_internals(),
                mem::take(&mut self.section_elements),
                mem::take(&mut self.section_key_range),
                self.section_line_begin_index,
                self.section_line_number,
                mem::take(&mut self.section_operator_range),
            );

            fn deep_append(
                depth: usize,
                elements: &mut Vec<Box<dyn SectionElement>>,
                section: Section,
            ) {
                if depth == 0 {
                    elements.push(Box::new(section));
                } else {
                    // we know the last element exists and must be a section
                    let traversed_section: &mut Section =
                        elements.last_mut().unwrap().as_mut_section().unwrap();
                    deep_append(depth - 1, traversed_section.get_elements_mut(), section)
                }
            }

            deep_append(
                self.section_depth - 1,
                self.document.get_elements_mut(),
                section,
            );
        }
    }

    fn next_char(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.index += c.len_utf8();
                Some(c)
            }
            None => None,
        }
    }

    pub fn parse(input: &str, printer: Box<dyn Printer>) -> Result<Document, Error> {
        let mut parser = Parser::<L> {
            chars: input.chars().peekable(),
            document: Document::new(input, printer),
            index: 0,
            input,
            line_begin_index: 0,
            line_number: 1,
            section_depth: 0,
            section_elements: Vec::new(),
            section_key_range: 0..0,
            section_line_begin_index: 0,
            section_line_number: 0,
            section_operator_range: 0..0,
            locale_type: PhantomData::<L>,
        };

        loop {
            parser.skip_whitespace();

            match parser.chars.peek() {
                Some('>') => parser.parse_comment(),
                Some('-') => {
                    parser.next_char();

                    match parser.chars.peek() {
                        Some('-') => parser.parse_embed()?,
                        _ => parser.parse_item()?,
                    }
                }
                Some('`') => parser.parse_escaped_key()?,
                Some('#') => parser.parse_section()?,
                Some(':') => {
                    parser.seek_eol();
                    return Err(error::field_without_key::<L>(
                        parser.line_number,
                        &parser.input[parser.line_begin_index..parser.index],
                    ));
                }
                Some('=') => {
                    parser.seek_eol();
                    return Err(error::attribute_without_key::<L>(
                        parser.line_number,
                        &parser.input[parser.line_begin_index..parser.index],
                    ));
                }
                Some('\n') => (),
                Some(c) => {
                    let first_char = *c;
                    parser.parse_key(first_char)?
                }
                None => break,
            }

            match parser.next_char() {
                Some('\n') => {
                    parser.line_number += 1;
                    parser.line_begin_index = parser.index;
                }
                None => break,
                _ => unreachable!(),
            }
        }

        parser.attach_section_elements();

        parser.document.set_number_of_lines(parser.line_number);

        Ok(parser.document)
    }

    // TODO: Encapsulation of this "remainder" parsing only makes sense if it's used by
    //       both parse_key and parse_escaped_key (which it currently isn't). Follow up.
    fn parse_after_escaped_key(
        &mut self,
        escape_operator_ranges: (Range<usize>, Range<usize>),
        key_range: Range<usize>,
    ) -> Result<(), Error> {
        self.skip_whitespace();

        match self.chars.peek() {
            None | Some('\n') => {
                let flag = Flag::new(
                    self.document.clone_internals(),
                    Some(escape_operator_ranges),
                    key_range,
                    self.line_begin_index,
                    self.line_number,
                );
                self.section_elements.push(Box::new(flag));
            }
            Some(':') => {
                let operator_index = self.index;

                self.next_char();

                let field_content = match self.read_token() {
                    Some(value_range) => FieldContent::Value(value_range),
                    None => FieldContent::None,
                };

                let field = Field::new(
                    field_content,
                    self.document.clone_internals(),
                    Some(escape_operator_ranges),
                    key_range,
                    self.line_begin_index,
                    self.line_number,
                    operator_index,
                );

                self.section_elements.push(Box::new(field));
            }
            Some('=') => {
                let operator_index = self.index;

                self.next_char();

                let value_range = self.read_token();

                let attribute = Attribute::new(
                    self.document.clone_internals(),
                    Some(escape_operator_ranges),
                    key_range,
                    self.line_begin_index,
                    self.line_number,
                    operator_index,
                    value_range,
                );

                match self.section_elements.last_mut() {
                    Some(element) => {
                        match element.as_mut_field() {
                            Some(field) => match &mut field.get_content_mut() {
                                FieldContent::Attributes(attributes) => attributes.push(attribute),
                                FieldContent::None => {
                                    field.set_content(FieldContent::Attributes(vec![attribute]))
                                }
                                _ => {
                                    return Err(error::mixed_field_content::<L>(field.line_number))
                                }
                            },
                            None => {
                                return Err(error::attribute_outside_field::<L>(
                                    // TODO: Refactor to unify branch logic with branch below
                                    self.line_number,
                                    &self.input[self.line_begin_index..self.index],
                                ));
                            }
                        }
                    }
                    _ => {
                        return Err(error::attribute_outside_field::<L>(
                            self.line_number,
                            &self.input[self.line_begin_index..self.index],
                        ))
                    }
                }
            }
            _ => {
                self.seek_eol(); // TODO: Possibly (here and elsewhere) implement a "parse after error" routine (maybe attach remaining instructions to some meta field that is therefore not publicly available in the document tree!), then use that to print snippets
                return Err(error::invalid_after_escape::<L>(
                    self.line_number,
                    &self.input[self.line_begin_index..self.index],
                ));
            }
        }

        Ok(())
    }

    fn parse_comment(&mut self) {
        let operator_index = self.index;

        self.next_char();

        let value_range = self.read_token();

        let comment = Comment::new(
            self.document.clone_internals(),
            self.line_begin_index,
            self.line_number,
            operator_index,
            value_range,
        );

        self.document.append_comment(comment);
    }

    fn parse_embed(&mut self) -> Result<(), Error> {
        let line_begin_index = self.line_begin_index;
        let operator_index = self.index - 1;

        self.next_char();

        loop {
            match self.chars.peek() {
                Some('-') => {
                    self.next_char();
                }
                None | Some('\n') => {
                    return Err(error::embed_without_key::<L>(
                        self.line_number,
                        &self.input[self.line_begin_index..self.index],
                    ))
                }
                Some(_) => break,
            }
        }

        let operator_range = operator_index..self.index;

        let begin_line_number = self.line_number;

        let key_range = match self.read_token() {
            Some(range) => range,
            None => {
                return Err(error::embed_without_key::<L>(
                    self.line_number,
                    &self.input[self.line_begin_index..self.index],
                ))
            }
        };

        match self.next_char() {
            Some('\n') => {
                self.line_number += 1;
                self.line_begin_index = self.index;
            }
            None => {
                return Err(error::unterminated_embed::<L>(
                    &self.input[key_range],
                    begin_line_number,
                ))
            }
            _ => unreachable!(),
        }

        let mut value_range = self.index..self.index;

        loop {
            let terminator_line_begin_index = self.index;

            self.skip_whitespace();

            let terminator_operator_index = self.index;

            loop {
                match self.chars.peek() {
                    Some('-') => {
                        self.next_char();
                    }
                    None => {
                        return Err(error::unterminated_embed::<L>(
                            &self.input[key_range],
                            begin_line_number,
                        ))
                    }
                    _ => break,
                }
            }

            let terminator_operator_range = terminator_operator_index..self.index;

            if terminator_operator_range.len() != operator_range.len() {
                self.seek_eol();

                value_range.end = self.index;

                match self.next_char() {
                    Some('\n') => {
                        self.line_number += 1;
                        self.line_begin_index = self.index;
                    }
                    None => {
                        return Err(error::unterminated_embed::<L>(
                            &self.input[key_range],
                            begin_line_number,
                        ))
                    }
                    _ => unreachable!(),
                }

                continue;
            }

            let terminator_key_range = match self.read_token() {
                Some(terminator_key_range) => {
                    if self.input[terminator_key_range.clone()] != self.input[key_range.clone()] {
                        continue;
                    }

                    terminator_key_range
                }
                None => continue,
            };

            let embed = Embed::new(
                self.document.clone_internals(),
                key_range,
                line_begin_index,
                begin_line_number,
                operator_range,
                terminator_key_range,
                terminator_line_begin_index,
                self.line_number,
                terminator_operator_range,
                if value_range.is_empty() {
                    None
                } else {
                    Some(value_range)
                },
            );

            self.section_elements.push(Box::new(embed));
            return Ok(());
        }
    }

    fn parse_escaped_key(&mut self) -> Result<(), Error> {
        let escape_begin_operator_index = self.index;

        self.next_char();

        loop {
            match self.chars.peek() {
                Some('`') => self.next_char(),
                None | Some('\n') => {
                    return Err(error::unterminated_escaped_key::<L>(
                        self.line_number,
                        &self.input[self.line_begin_index..self.index],
                    ))
                }
                Some(_) => break,
            };
        }

        let escape_begin_operator_range = escape_begin_operator_index..self.index;

        self.skip_whitespace();

        let mut key_range = self.index..self.index;
        let mut escape_end_operator_range;

        'parse_key_and_terminator: loop {
            match self.chars.peek() {
                Some('`') => {
                    escape_end_operator_range = self.index..(self.index + 1);

                    self.next_char();

                    loop {
                        if escape_end_operator_range.len() == escape_begin_operator_range.len() {
                            if key_range.is_empty() {
                                self.seek_eol();
                                return Err(error::escape_without_key::<L>(
                                    self.line_number,
                                    &self.input[self.line_begin_index..self.index],
                                ));
                            }

                            break 'parse_key_and_terminator;
                        }
                        match self.chars.peek() {
                            Some('`') => {
                                escape_end_operator_range.end = self.index + 1;
                                self.next_char();
                            }
                            _ => {
                                key_range.end = self.index;
                                break;
                            }
                        }
                    }
                }
                Some(c) => {
                    if !c.is_whitespace() {
                        key_range.end = self.index + c.len_utf8();
                    }

                    self.next_char();
                }
                None => {
                    return Err(error::unterminated_escaped_key::<L>(
                        self.line_number,
                        &self.input[self.line_begin_index..],
                    ))
                }
            }
        }

        let escape_operator_ranges = (escape_begin_operator_range, escape_end_operator_range);

        self.parse_after_escaped_key(escape_operator_ranges, key_range)
    }

    fn parse_item(&mut self) -> Result<(), Error> {
        let operator_index = self.index - 1;

        let value_range = self.read_token();

        let item = Item::new(
            self.document.clone_internals(),
            self.line_begin_index,
            self.line_number,
            operator_index,
            value_range,
        );

        match self.section_elements.last_mut() {
            Some(element) => {
                match element.as_mut_field() {
                    Some(field) => match &mut field.get_content_mut() {
                        FieldContent::Items(items) => items.push(item),
                        FieldContent::None => field.set_content(FieldContent::Items(vec![item])),
                        _ => return Err(error::mixed_field_content::<L>(field.line_number)),
                    },
                    None => {
                        return Err(error::item_outside_field::<L>(
                            // TODO: Refactor to unify branch logic with branch below
                            self.line_number,
                            &self.input[self.line_begin_index..self.index],
                        ));
                    }
                }
            }
            _ => {
                return Err(error::item_outside_field::<L>(
                    self.line_number,
                    &self.input[self.line_begin_index..self.index],
                ))
            }
        }

        Ok(())
    }

    fn parse_key(&mut self, first_char: char) -> Result<(), Error> {
        let mut key_range = self.index..(self.index + first_char.len_utf8());

        self.next_char();

        loop {
            match self.chars.peek() {
                Some(':') => {
                    let operator_index = self.index;

                    self.next_char();

                    let field_content = match self.read_token() {
                        Some(value_range) => FieldContent::Value(value_range),
                        None => FieldContent::None,
                    };

                    let field = Field::new(
                        field_content,
                        self.document.clone_internals(),
                        None,
                        key_range,
                        self.line_begin_index,
                        self.line_number,
                        operator_index,
                    );

                    self.section_elements.push(Box::new(field));
                    break;
                }
                Some('=') => {
                    let operator_index = self.index;

                    self.next_char();

                    let value_range = self.read_token();

                    let attribute = Attribute::new(
                        self.document.clone_internals(),
                        None,
                        key_range,
                        self.line_begin_index,
                        self.line_number,
                        operator_index,
                        value_range,
                    );

                    match self.section_elements.last_mut() {
                        Some(element) => {
                            match element.as_mut_field() {
                                Some(field) => match &mut field.get_content_mut() {
                                    FieldContent::Attributes(attributes) => {
                                        attributes.push(attribute)
                                    }
                                    FieldContent::None => {
                                        field.set_content(FieldContent::Attributes(vec![attribute]))
                                    }
                                    _ => {
                                        return Err(error::mixed_field_content::<L>(
                                            field.line_number,
                                        ))
                                    }
                                },
                                None => {
                                    return Err(error::attribute_outside_field::<L>(
                                        // TODO: Refactor to unify branch logic with branch below
                                        self.line_number,
                                        &self.input[self.line_begin_index..self.index],
                                    ));
                                }
                            }
                        }
                        _ => {
                            return Err(error::attribute_outside_field::<L>(
                                self.line_number,
                                &self.input[self.line_begin_index..self.index],
                            ))
                        }
                    }

                    break;
                }
                None | Some('\n') => {
                    let flag = Flag::new(
                        self.document.clone_internals(),
                        None,
                        key_range,
                        self.line_begin_index,
                        self.line_number,
                    );

                    self.section_elements.push(Box::new(flag));

                    break;
                }
                Some(c) => {
                    if !c.is_whitespace() {
                        key_range.end = self.index + c.len_utf8();
                    }

                    self.next_char();
                }
            }
        }

        Ok(())
    }

    fn parse_section(&mut self) -> Result<(), Error> {
        let operator_begin_index = self.index;

        self.next_char();

        let mut operator_len = 1;
        loop {
            match self.chars.peek() {
                Some('#') => {
                    operator_len += 1;
                    self.next_char();
                }
                None | Some('\n') => {
                    return Err(error::section_without_key::<L>(
                        self.line_number,
                        &self.input[self.line_begin_index..self.index],
                    ))
                }
                Some(_) => break,
            }
        }

        let operator_range = operator_begin_index..self.index;

        if operator_len > self.section_depth + 1 {
            self.seek_eol();
            return Err(error::section_level_skip::<L>(
                self.line_number,
                &self.input[self.line_begin_index..self.index],
            ));
        }

        self.attach_section_elements();

        let key_range = match self.read_token() {
            Some(range) => range,
            None => {
                return Err(error::section_without_key::<L>(
                    self.line_number,
                    &self.input[self.line_begin_index..],
                ))
            }
        };

        self.section_depth = operator_len;
        self.section_key_range = key_range;
        self.section_line_begin_index = self.line_begin_index;
        self.section_line_number = self.line_number;
        self.section_operator_range = operator_range;

        Ok(())
    }

    fn read_token(&mut self) -> Option<Range<usize>> {
        self.skip_whitespace();

        match self.chars.peek() {
            None | Some('\n') => None,
            Some(c) => {
                let mut range = self.index..(self.index + c.len_utf8());

                self.next_char();

                loop {
                    match self.chars.peek() {
                        None | Some('\n') => return Some(range),
                        Some(c) => {
                            if !c.is_whitespace() {
                                range.end = self.index + c.len_utf8();
                            }

                            self.next_char();
                        }
                    }
                }
            }
        }
    }

    fn seek_eol(&mut self) {
        loop {
            match self.chars.peek() {
                None | Some('\n') => break,
                Some(_) => self.next_char(),
            };
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.chars.peek() {
                Some(c) => {
                    if *c == '\n' || !c.is_whitespace() {
                        return;
                    }

                    self.next_char();
                }
                None => return,
            }
        }
    }
}
