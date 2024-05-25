use super::Printer;

#[derive(Debug)]
pub struct TextPrinter;

impl Printer for TextPrinter {
    fn comment(&self, comment: &str) -> String {
        comment.into()
    }

    fn gutter(&self, line_number: u32) -> String {
        format!(" {:>3} | ", line_number)
    }

    fn key(&self, key: &str) -> String {
        key.into()
    }

    fn operator(&self, operator: &str) -> String {
        operator.into()
    }

    fn value(&self, value: &str) -> String {
        value.into()
    }
}
