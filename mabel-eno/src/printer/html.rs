use super::Printer;

#[derive(Debug)]
pub struct HtmlPrinter;

impl HtmlPrinter {
    fn escape(string: &str) -> String {
        string
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }
}

impl Printer for HtmlPrinter {
    fn comment(&self, comment: &str) -> String {
        let comment_escaped = HtmlPrinter::escape(comment);
        format!(r#"<span class="comment">{comment_escaped}</span>"#)
    }

    fn gutter(&self, line_number: u32) -> String {
        format!(r#"<span class="gutter"> {:>3} </span> "#, line_number)
    }

    fn key(&self, key: &str) -> String {
        let key_escaped = HtmlPrinter::escape(key);
        format!(r#"<span class="key">{key_escaped}</span>"#)
    }

    fn operator(&self, operator: &str) -> String {
        format!(r#"<span class="operator">{operator}</span>"#)
    }

    fn value(&self, value: &str) -> String {
        let value_escaped = HtmlPrinter::escape(value);
        format!(r#"<span class="value">{value_escaped}</span>"#)
    }
}
