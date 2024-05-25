use indoc::formatdoc;

use crate::parse;

#[test]
fn test_untouched_elements_attribute() {
    let document = parse(&formatdoc!(
        r#"
        field:
        attribute =
    "#
    ))
    .unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 1);

    let field = document.field("field").unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 2);

    let _attribute = field.attribute("attribute");

    assert!(document.untouched_elements().is_empty());
}

#[test]
fn test_untouched_elements_elements() {
    let document = parse(&formatdoc!(
        r#"
        flag1
        flag2
    "#
    ))
    .unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 2);
    assert_eq!(untouched_elements[0].line_number(), 1);
    assert_eq!(untouched_elements[1].line_number(), 2);

    let _elements = document.elements();

    assert!(document.untouched_elements().is_empty());
}

#[test]
fn test_untouched_elements_embed() {
    let document = parse(&formatdoc!(
        r#"
        -- embed
        -- embed
    "#
    ))
    .unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 1);

    let _embed = document.embed("embed").unwrap();

    assert!(document.untouched_elements().is_empty());
}

#[test]
fn test_untouched_elements_field() {
    let document = parse("field:").unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 1);

    let _field = document.field("field").unwrap();

    assert!(document.untouched_elements().is_empty());
}

#[test]
fn test_untouched_elements_flag() {
    let document = parse("flag").unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 1);

    let _flag = document.flag("flag").unwrap();

    assert!(document.untouched_elements().is_empty());
}

#[test]
fn test_untouched_elements_item() {
    let document = parse(&formatdoc!(
        r#"
        field:
        -
    "#
    ))
    .unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 1);

    let field = document.field("field").unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 2);

    let _items = field.items();

    assert!(document.untouched_elements().is_empty());
}

#[test]
fn test_untouched_elements_section() {
    let document = parse("# section").unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 1);

    let _section = document.section("section").unwrap();

    assert!(document.untouched_elements().is_empty());
}

#[test]
fn test_untouched_elements_section_flag() {
    let document = parse(&formatdoc!(
        r#"
        # section
        flag
    "#
    ))
    .unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 1);

    let section = document.section("section").unwrap();

    let untouched_elements = document.untouched_elements();

    assert_eq!(untouched_elements.len(), 1);
    assert_eq!(untouched_elements[0].line_number(), 2);

    let _flag = section.flag("flag").unwrap();

    assert!(document.untouched_elements().is_empty());
}
