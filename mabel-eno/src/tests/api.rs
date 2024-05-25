use indoc::formatdoc;

use crate::parse;
use crate::prelude::*;

#[test]
fn test_attributes() {
    let result = parse(&formatdoc!(
        r#"
        field:
        attribute = value
    "#
    ));

    match result.unwrap().elements().first().unwrap().as_field() {
        Some(field) => {
            assert_eq!(field.key(), "field");
            assert_eq!(field.line_number, 1);

            let attributes = field.attributes().unwrap();

            assert_eq!(attributes.len(), 1);

            let attribute = attributes.first().unwrap();

            assert_eq!(attribute.key(), "attribute");
            assert_eq!(attribute.line_number, 2);
            assert_eq!(
                &attribute.optional_value::<String>().unwrap().unwrap(),
                "value"
            );
        }
        _ => panic!("Field expected"),
    }
}

#[test]
fn test_embed() {
    let result = parse(&formatdoc!(
        r#"
        -- embed
        value
        -- embed
    "#
    ));

    match result.unwrap().elements().first().unwrap().as_embed() {
        Some(embed) => {
            assert_eq!(embed.key(), "embed");
            assert_eq!(embed.line_number, 1);
            assert_eq!(&embed.optional_value::<String>().unwrap().unwrap(), "value");
        }
        _ => panic!("Embed with value expected"),
    }
}

#[test]
fn test_flag() {
    let result = parse("flag");

    match result.unwrap().elements().first().unwrap().as_flag() {
        Some(flag) => {
            assert_eq!(flag.key(), "flag");
            assert_eq!(flag.line_number, 1);
        }
        _ => panic!("Flag expected"),
    }
}

#[test]
fn test_flag_with_escaped_key() {
    let result = parse("`` `flag` ``");

    match result.unwrap().elements().first().unwrap().as_flag() {
        Some(flag) => {
            assert_eq!(flag.key(), "`flag`");
            assert_eq!(flag.line_number, 1);
        }
        _ => panic!("Flag expected"),
    }
}

#[test]
fn test_items() {
    let result = parse(&formatdoc!(
        r#"
        field:
        - item1
        - item2
    "#
    ));

    match result.unwrap().elements().first().unwrap().as_field() {
        Some(field) => {
            assert_eq!(field.key(), "field");
            assert_eq!(field.line_number, 1);

            let items = field.items().unwrap();

            assert_eq!(items.len(), 2);

            let mut iter = items.iter();

            let first = iter.next().expect("First item expected");
            assert_eq!(first.line_number, 2);
            assert_eq!(&first.required_value::<String>().unwrap(), "item1");

            let second = iter.next().expect("Second item expected");
            assert_eq!(second.line_number, 3);
            assert_eq!(&second.required_value::<String>().unwrap(), "item2");
        }
        _ => panic!("Field expected"),
    }
}

#[test]
fn test_section() {
    let result = parse(&formatdoc!(
        r#"
        # section
        field: value
    "#
    ));

    match result.unwrap().elements().first().unwrap().as_section() {
        Some(section) => {
            assert_eq!(section.key(), "section");
            assert_eq!(section.len(), 1);

            match section.elements().first().unwrap().as_field() {
                Some(field) => {
                    assert_eq!(field.key(), "field");
                    assert_eq!(field.line_number, 2);
                    assert_eq!(&field.required_value::<String>().unwrap(), "value");
                }
                _ => panic!("Field expected"),
            }
        }
        _ => panic!("Section expected"),
    }
}

#[test]
fn test_parse() {
    let result = parse("field: value");

    assert!(result.is_ok());
}
