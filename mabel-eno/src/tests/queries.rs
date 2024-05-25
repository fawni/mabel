use indoc::formatdoc;

use crate::{parse, Error};

#[test]
fn test_field_required_value_u32() {
    let document = parse("field: 23").expect("Document should parse");

    let value = || -> Result<u32, Error> { document.field("field")?.required_value() }().unwrap();

    assert_eq!(value, 23);
}

#[test]
fn test_field_required_value_u32_err() {
    let document = parse("field: thirtytwo").expect("Document should parse");

    let error =
        || -> Result<u32, Error> { document.field("field")?.required_value() }().unwrap_err();

    assert_eq!(error.line, 1);
    assert_eq!(error.message, "invalid digit found in string");
}

#[test]
fn test_field_attribute_required_value_string() {
    let document = parse(&formatdoc!(
        r#"
        > comment
        field:
        attribute = value
    "#
    ))
    .expect("Document should parse");

    let value = || -> Result<String, Error> {
        document
            .field("field")?
            .attribute("attribute")?
            .required_value()
    }()
    .unwrap();

    assert_eq!(value, "value");
}

#[test]
fn test_field_attribute_required_value_string_err() {
    let document = parse(&formatdoc!(
        r#"
        > comment
        field:
        attribute =
    "#
    ))
    .expect("Document should parse");

    let error = || -> Result<String, Error> {
        document
            .field("field")?
            .attribute("attribute")?
            .required_value()
    }()
    .unwrap_err();

    assert_eq!(error.line, 3);
    assert_eq!(error.message, "Missing value");
}

#[test]
fn test_section_embed_required_value_string() {
    let document = parse(&formatdoc!(
        r#"
        # section
        -- embed
        value
        -- embed
    "#
    ))
    .expect("Document should parse");

    let value = || -> Result<String, Error> {
        document
            .section("section")?
            .embed("embed")?
            .required_value()
    }()
    .unwrap();

    assert_eq!(value, "value");
}

#[test]
fn test_section_embed_required_value_string_err() {
    let document = parse(&formatdoc!(
        r#"
        # section
        -- embed
        -- embed
    "#
    ))
    .expect("Document should parse");

    let error = || -> Result<String, Error> {
        document
            .section("section")?
            .embed("embed")?
            .required_value()
    }()
    .unwrap_err();

    assert_eq!(error.line, 2);
    assert_eq!(error.message, "Missing value");
}
