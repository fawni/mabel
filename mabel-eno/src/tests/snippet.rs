use indoc::formatdoc;

use crate::{parse, TextPrinter};

#[test]
fn test_document_snippet() {
    let input = formatdoc!(
        r#"
        # ailment

        border: creed dent
        elephant: flamingo goose

        hope:
        iridescent = judge kite
        layman = moose

        north
        opalescent
        power: quartz

        -- reserve
        solitude tremor
        unnerving veal
        wonder.
        

        xanadu young zeal!
        -- reserve
    "#
    );

    let expected = r#"   1 | # ailment
   2 | 
   3 | border: creed dent
   4 | elephant: flamingo goose
   5 | 
   6 | hope:
   7 | iridescent = judge kite
   8 | layman = moose
   9 | 
  10 | north
  11 | opalescent
  12 | power: quartz
  13 | 
  14 | -- reserve
  15 | solitude tremor
  16 | unnerving veal
  17 | wonder.
  18 | 
  19 | 
  20 | xanadu young zeal!
  21 | -- reserve
  22 | "#;

    let snippet = parse(&input)
        .unwrap()
        .snippet_with_options(&TextPrinter, true);

    assert_eq!(snippet, expected);
}

#[test]
fn test_embed_snippet() {
    let document = parse(&formatdoc!(
        r#"
        -- embed
        one
        two
        -- embed
    "#
    ))
    .unwrap();

    let snippet = document
        .embed("embed")
        .unwrap()
        .snippet_with_options(&TextPrinter, false)
        .unwrap();

    assert_eq!(snippet, "-- embed\none\ntwo\n-- embed");
}

#[test]
fn test_field_snippet() {
    let document = parse(&formatdoc!(
        r#"
        field:
    "#
    ))
    .unwrap();

    let snippet = document
        .field("field")
        .unwrap()
        .snippet_with_options(&TextPrinter, false)
        .unwrap();

    assert_eq!(snippet, "field:");
}

#[test]
fn test_field_with_attributes_snippet() {
    let document = parse(&formatdoc!(
        r#"
        field:
        one = value
        two = value
    "#
    ))
    .unwrap();

    let snippet = document
        .field("field")
        .unwrap()
        .snippet_with_options(&TextPrinter, false)
        .unwrap();

    assert_eq!(snippet, "field:\none = value\ntwo = value");
}

#[test]
fn test_field_with_items_snippet() {
    let document = parse(&formatdoc!(
        r#"
        field:
        - one
        - two
    "#
    ))
    .unwrap();

    let snippet = document
        .field("field")
        .unwrap()
        .snippet_with_options(&TextPrinter, false)
        .unwrap();

    assert_eq!(snippet, "field:\n- one\n- two");
}

#[test]
fn test_field_with_value_snippet() {
    let document = parse(&formatdoc!(
        r#"
        field: value
    "#
    ))
    .unwrap();

    let snippet = document
        .field("field")
        .unwrap()
        .snippet_with_options(&TextPrinter, false)
        .unwrap();

    assert_eq!(snippet, "field: value");
}

#[test]
fn test_flag_snippet() {
    let document = parse(&formatdoc!(
        r#"
        flag
    "#
    ))
    .unwrap();

    let snippet = document
        .flag("flag")
        .unwrap()
        .snippet_with_options(&TextPrinter, false)
        .unwrap();

    assert_eq!(snippet, "flag");
}

#[test]
fn test_section_snippet() {
    let document = parse(&formatdoc!(
        r#"
        # section
    "#
    ))
    .unwrap();

    let snippet = document
        .section("section")
        .unwrap()
        .snippet_with_options(&TextPrinter, false)
        .unwrap();

    assert_eq!(snippet, "# section");
}
