use std::fmt::Debug;

mod html;
mod terminal;
mod text;

pub use html::HtmlPrinter;
pub use terminal::TerminalPrinter;
pub use text::TextPrinter;

pub trait Printer: Debug {
    fn comment(&self, comment: &str) -> String;
    fn gutter(&self, line_number: u32) -> String;
    fn key(&self, key: &str) -> String;
    fn operator(&self, perator: &str) -> String;
    fn value(&self, value: &str) -> String;
}
