use crate::{parse_with_locale, Locale};

struct FrenglishLocale;

impl Locale for FrenglishLocale {
    fn attribute_outside_field(line_number: u32, line: &str) -> String {
        format!(
            "Le attribute in line {} is not contained within a field. ('{}')",
            line_number, line
        )
    }

    fn attribute_without_key(line_number: u32, line: &str) -> String {
        format!(
            "Le attribute in line {} has no key. ('{}')",
            line_number, line
        )
    }

    fn embed_without_key(line_number: u32, line: &str) -> String {
        format!("Le embed in line {} has no key. ('{}')", line_number, line)
    }

    fn escape_without_key(line_number: u32, line: &str) -> String {
        format!(
            "Le escape sequence in line {} specifies no key. ('{}')",
            line_number, line
        )
    }

    fn field_without_key(line_number: u32, line: &str) -> String {
        format!("Le field in line {} has no key. ('{}')", line_number, line)
    }

    fn invalid_after_escape(line_number: u32, line: &str) -> String {
        format!("Le escape sequence in line {} can only be followed by an attribute or field operator. ('{}')", line_number, line)
    }

    fn item_outside_field(line_number: u32, line: &str) -> String {
        format!(
            "Le item in line {} is not contained within a field. ('{}')",
            line_number, line
        )
    }

    fn mixed_field_content(line_number: u32) -> String {
        format!(
            "Le field in line {} must contain either only attributes, only items, or only a value.",
            line_number
        )
    }

    fn section_level_skip(line_number: u32, line: &str) -> String {
        format!("Le section in line {} is more than one level deeper than le one it is contained in. ('{}')", line_number, line)
    }

    fn section_without_key(line_number: u32, line: &str) -> String {
        format!(
            "Le section in line {} has no key. ('{}')",
            line_number, line
        )
    }

    fn unterminated_embed(key: &str, line_number: u32) -> String {
        format!(
            "Le embed '{}' starting in line {} is not terminated until le end of le document.",
            key, line_number
        )
    }

    fn unterminated_escaped_key(line_number: u32, line: &str) -> String {
        format!(
            "Le key escape sequence in line {} is not terminated before le end of le line. ('{}')",
            line_number, line
        )
    }
}

#[test]
fn test_custom_locale() {
    let error = parse_with_locale::<FrenglishLocale>(": value").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(error.message, "Le field in line 1 has no key. (': value')");
}
