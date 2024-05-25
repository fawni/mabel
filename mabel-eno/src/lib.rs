mod elements;
mod error;
mod locale;
mod lookup;
mod parser;
mod queries;

#[cfg(test)]
mod tests;

pub mod prelude;
mod printer;

use parser::Parser;
use printer::Printer;

pub use printer::{HtmlPrinter, TerminalPrinter, TextPrinter};

pub use elements::{
    Attribute, Document, Element, Embed, Field, Flag, Item, Section, SectionElement,
};
pub use error::Error;
pub use locale::{DefaultLocale, Locale};
pub use lookup::lookup_line;
pub use queries::{AttributeQuery, EmbedQuery, FieldQuery, FlagQuery, SectionQuery};

pub fn parse(input: &str) -> Result<Document, Error> {
    Parser::<DefaultLocale>::parse(input, Box::new(TextPrinter))
}

pub fn parse_with_locale<L: Locale>(input: &str) -> Result<Document, Error> {
    Parser::<L>::parse(input, Box::new(TextPrinter))
}

pub fn parse_with_locale_and_printer<L: Locale>(
    input: &str,
    printer: Box<dyn Printer>,
) -> Result<Document, Error> {
    Parser::<L>::parse(input, printer)
}

pub fn parse_with_printer(input: &str, printer: Box<dyn Printer>) -> Result<Document, Error> {
    Parser::<DefaultLocale>::parse(input, printer)
}
