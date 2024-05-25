use indoc::formatdoc;
use mabel_eno::{parse, Error};

fn main() {
    let input = formatdoc!(
        r#"
        # section
        field:
        attribute = value
    "#
    );

    let result = my_get_attribute_value(&input).unwrap();

    assert_eq!(result, "value");
}

fn my_get_attribute_value(input: &str) -> Result<String, Error> {
    parse(input)?
        .section("section")?
        .field("field")?
        .attribute("attribute")?
        .required_value()
}
