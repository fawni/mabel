pub trait Locale {
    fn attribute_outside_field(line_number: u32, line: &str) -> String;
    fn attribute_without_key(line_number: u32, line: &str) -> String;
    fn embed_without_key(line_number: u32, line: &str) -> String;
    fn escape_without_key(line_number: u32, line: &str) -> String;
    fn field_without_key(line_number: u32, line: &str) -> String;
    fn invalid_after_escape(line_number: u32, line: &str) -> String;
    fn item_outside_field(line_number: u32, line: &str) -> String;
    fn mixed_field_content(line_number: u32) -> String;
    fn section_level_skip(line_number: u32, line: &str) -> String;
    fn section_without_key(line_number: u32, line: &str) -> String;
    fn unterminated_embed(key: &str, line_number: u32) -> String;
    fn unterminated_escaped_key(line_number: u32, line: &str) -> String;
}

pub struct DefaultLocale;

impl Locale for DefaultLocale {
    fn attribute_outside_field(line_number: u32, line: &str) -> String {
        format!(
            "The attribute in line {} is not contained within a field. ('{}')",
            line_number, line
        )
    }

    fn attribute_without_key(line_number: u32, line: &str) -> String {
        format!(
            "The attribute in line {} has no key. ('{}')",
            line_number, line
        )
    }

    fn embed_without_key(line_number: u32, line: &str) -> String {
        format!("The embed in line {} has no key. ('{}')", line_number, line)
    }

    fn escape_without_key(line_number: u32, line: &str) -> String {
        format!(
            "The escape sequence in line {} specifies no key. ('{}')",
            line_number, line
        )
    }

    fn field_without_key(line_number: u32, line: &str) -> String {
        format!("The field in line {} has no key. ('{}')", line_number, line)
    }

    fn invalid_after_escape(line_number: u32, line: &str) -> String {
        format!("The escape sequence in line {} can only be followed by an attribute or field operator. ('{}')", line_number, line)
    }

    fn item_outside_field(line_number: u32, line: &str) -> String {
        format!(
            "The item in line {} is not contained within a field. ('{}')",
            line_number, line
        )
    }

    fn mixed_field_content(line_number: u32) -> String {
        format!("The field in line {} must contain either only attributes, only items, or only a value.", line_number)
    }

    fn section_level_skip(line_number: u32, line: &str) -> String {
        format!("The section in line {} is more than one level deeper than the one it is contained in. ('{}')", line_number, line)
    }

    fn section_without_key(line_number: u32, line: &str) -> String {
        format!(
            "The section in line {} has no key. ('{}')",
            line_number, line
        )
    }

    fn unterminated_embed(key: &str, line_number: u32) -> String {
        format!(
            "The embed '{}' starting in line {} is not terminated until the end of the document.",
            key, line_number
        )
    }

    fn unterminated_escaped_key(line_number: u32, line: &str) -> String {
        format!("The key escape sequence in line {} is not terminated before the end of the line. ('{}')", line_number, line)
    }
}
