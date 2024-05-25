use indoc::formatdoc;
use mabel_eno::{lookup_line, parse};

fn main() {
    let input = formatdoc!(
        r#"
        # section
        field:
        attribute = value
    "#
    );

    let document = parse(&input).unwrap();
    let element_option = lookup_line(&document, 3).unwrap();
    let element = element_option.unwrap();

    assert!(element.is_attribute());
}
