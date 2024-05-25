use indoc::formatdoc;

use crate::parse;

#[test]
fn test_field_key_unicode_ellipsis() {
    let document = parse(&formatdoc!(
        r#"
        …: value
        … 2 3: value_1
        1 … 3: value_2
        1 2 …: value_3
    "#
    ))
    .expect("Document should parse");

    assert_eq!(
        document
            .field("…")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value"
    );

    assert_eq!(
        document
            .field("… 2 3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_1"
    );

    assert_eq!(
        document
            .field("1 … 3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_2"
    );

    assert_eq!(
        document
            .field("1 2 …")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_3"
    );
}

#[test]
fn test_field_key_escaped_unicode_ellipsis() {
    let document = parse(&formatdoc!(
        r#"
        `…`: value
        `… 2 3`: value_1
        `1 … 3`: value_2
        `1 2 …`: value_3
    "#
    ))
    .expect("Document should parse");

    assert_eq!(
        document
            .field("…")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value"
    );

    assert_eq!(
        document
            .field("… 2 3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_1"
    );

    assert_eq!(
        document
            .field("1 … 3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_2"
    );

    assert_eq!(
        document
            .field("1 2 …")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_3"
    );
}

#[test]
fn test_field_value_unicode_ellipsis() {
    let document = parse(&formatdoc!(
        r#"
        field: …
        field_1: … 2 3
        field_2: 1 … 3
        field_3: 1 2 …
    "#
    ))
    .expect("Document should parse");

    assert_eq!(
        document
            .field("field")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "…"
    );

    assert_eq!(
        document
            .field("field_1")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "… 2 3"
    );

    assert_eq!(
        document
            .field("field_2")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "1 … 3"
    );

    assert_eq!(
        document
            .field("field_3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "1 2 …"
    );
}

#[test]
fn test_item_value_unicode_ellipsis() {
    let document = parse(&formatdoc!(
        r#"
        field:
        - …
        - … 2 3
        - 1 … 3
        - 1 2 …
    "#
    ))
    .expect("Document should parse");

    let field = document.field("field").unwrap();
    let items = field.items().unwrap();

    assert_eq!(items[0].required_value::<String>().unwrap(), "…");

    assert_eq!(items[1].required_value::<String>().unwrap(), "… 2 3");

    assert_eq!(items[2].required_value::<String>().unwrap(), "1 … 3");

    assert_eq!(items[3].required_value::<String>().unwrap(), "1 2 …");
}

#[test]
fn test_attribute_key_unicode_ellipsis() {
    let document = parse(&formatdoc!(
        r#"
        field:
        … = value
        … 2 3 = value_1
        1 … 3 = value_2
        1 2 … = value_3
    "#
    ))
    .expect("Document should parse");

    let field = document.field("field").unwrap();

    assert_eq!(
        field
            .attribute("…")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value"
    );

    assert_eq!(
        field
            .attribute("… 2 3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_1"
    );

    assert_eq!(
        field
            .attribute("1 … 3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_2"
    );

    assert_eq!(
        field
            .attribute("1 2 …")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_3"
    );
}

#[test]
fn test_attribute_key_escaped_unicode_ellipsis() {
    let document = parse(&formatdoc!(
        r#"
        field:
        `…` = value
        `… 2 3` = value_1
        `1 … 3` = value_2
        `1 2 …` = value_3
    "#
    ))
    .expect("Document should parse");

    let field = document.field("field").unwrap();

    assert_eq!(
        field
            .attribute("…")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value"
    );

    assert_eq!(
        field
            .attribute("… 2 3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_1"
    );

    assert_eq!(
        field
            .attribute("1 … 3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_2"
    );

    assert_eq!(
        field
            .attribute("1 2 …")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "value_3"
    );
}

#[test]
fn test_attribute_value_unicode_ellipsis() {
    let document = parse(&formatdoc!(
        r#"
        field:
        attribute = …
        attribute_1 = … 2 3
        attribute_2 = 1 … 3
        attribute_3 = 1 2 …
    "#
    ))
    .expect("Document should parse");

    let field = document.field("field").unwrap();

    assert_eq!(
        field
            .attribute("attribute")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "…"
    );

    assert_eq!(
        field
            .attribute("attribute_1")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "… 2 3"
    );

    assert_eq!(
        field
            .attribute("attribute_2")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "1 … 3"
    );

    assert_eq!(
        field
            .attribute("attribute_3")
            .unwrap()
            .required_value::<String>()
            .unwrap(),
        "1 2 …"
    );
}
