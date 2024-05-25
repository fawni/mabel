use indoc::formatdoc;

use crate::parse;

#[test]
fn test_attribute_outside_field() {
    let error = parse("attribute = value").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The attribute in line 1 is not contained within a field. ('attribute = value')"
    );
}

#[test]
fn test_attribute_without_key() {
    let error = parse("= value").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The attribute in line 1 has no key. ('= value')"
    );
}

#[test]
fn test_embed_without_key() {
    let error = parse("--").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(error.message, "The embed in line 1 has no key. ('--')");
}

#[test]
fn test_embed_without_key_variant2() {
    let error = parse("-- ").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(error.message, "The embed in line 1 has no key. ('-- ')");
}

#[test]
fn test_escape_without_key() {
    let error = parse("` `").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The escape sequence in line 1 specifies no key. ('` `')"
    );
}

#[test]
fn test_field_without_key() {
    let error = parse(": value").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(error.message, "The field in line 1 has no key. (': value')");
}

#[test]
fn test_invalid_after_escape() {
    let error = parse("`key` value").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(error.message, "The escape sequence in line 1 can only be followed by an attribute or field operator. ('`key` value')");
}

#[test]
fn test_item_outside_field() {
    let error = parse("- item").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The item in line 1 is not contained within a field. ('- item')"
    );
}

#[test]
fn test_mixed_field_content_value_attribute() {
    let error = parse(&formatdoc!(
        r#"
        field: value
        attribute = value
    "#
    ))
    .unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The field in line 1 must contain either only attributes, only items, or only a value."
    );
}

#[test]
fn test_mixed_field_content_item_attribute() {
    let error = parse(&formatdoc!(
        r#"
        field:
        - item
        attribute = value
    "#
    ))
    .unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The field in line 1 must contain either only attributes, only items, or only a value."
    );
}

#[test]
fn test_mixed_field_content_value_item() {
    let error = parse(&formatdoc!(
        r#"
        field: value
        - item
    "#
    ))
    .unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The field in line 1 must contain either only attributes, only items, or only a value."
    );
}

#[test]
fn test_mixed_field_content_attribute_item() {
    let error = parse(&formatdoc!(
        r#"
        field:
        attribute = value
        - item
    "#
    ))
    .unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The field in line 1 must contain either only attributes, only items, or only a value."
    );
}

#[test]
fn test_section_level_skip_0_to_2() {
    let error = parse("## section").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(error.message, "The section in line 1 is more than one level deeper than the one it is contained in. ('## section')");
}

#[test]
fn test_section_level_skip_1_to_3() {
    let error = parse(&formatdoc!(
        r#"
        # section
        ### section
    "#
    ))
    .unwrap_err();

    assert_eq!(error.line, 2);
    assert_eq!(error.message, "The section in line 2 is more than one level deeper than the one it is contained in. ('### section')");
}

#[test]
fn test_section_without_key() {
    let error = parse("#").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(error.message, "The section in line 1 has no key. ('#')");
}

#[test]
fn test_section_without_key_variant2() {
    let error = parse("# ").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(error.message, "The section in line 1 has no key. ('# ')");
}

#[test]
fn test_unterminated_embed() {
    let error = parse(&formatdoc!(
        r#"
        -- embed
        ...
    "#
    ))
    .unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The embed 'embed' starting in line 1 is not terminated until the end of the document."
    );
}

#[test]
fn test_unterminated_escaped_key() {
    let error = parse("`key").unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(
        error.message,
        "The key escape sequence in line 1 is not terminated before the end of the line. ('`key')"
    );
}
