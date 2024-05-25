use indoc::formatdoc;

use crate::{parse, TextPrinter};

#[test]
fn test_missing_linebreaks() {
    let input = formatdoc!(
        r#"
        base_url: https://myawesomemusic.site/
        favicon: my_favicon.png
        feed_image: exported_logo_v3.jpg
        title: My awesome music

        home_image:
        description = Me in my studio
        file = studio_3.png

        -- text
        Lorem ipsum dolor sit amet ...
        -- text
    "#
    );

    let document = parse(input.trim_end()).expect("Document should parse");

    let snippet = document.snippet_with_options(&TextPrinter, false);

    assert_eq!(snippet, input.trim_end());
}

#[test]
fn test_missing_linebreaks_with_line_numbers() {
    let input = formatdoc!(
        r#"
        base_url: https://myawesomemusic.site/
        favicon: my_favicon.png
        feed_image: exported_logo_v3.jpg
        title: My awesome music

        home_image:
        description = Me in my studio
        file = studio_3.png

        -- text
        Lorem ipsum dolor sit amet ...
        -- text
    "#
    );

    let mut expected = String::new();
    expected.push_str("   1 | base_url: https://myawesomemusic.site/\n");
    expected.push_str("   2 | favicon: my_favicon.png\n");
    expected.push_str("   3 | feed_image: exported_logo_v3.jpg\n");
    expected.push_str("   4 | title: My awesome music\n");
    expected.push_str("   5 | \n");
    expected.push_str("   6 | home_image:\n");
    expected.push_str("   7 | description = Me in my studio\n");
    expected.push_str("   8 | file = studio_3.png\n");
    expected.push_str("   9 | \n");
    expected.push_str("  10 | -- text\n");
    expected.push_str("  11 | Lorem ipsum dolor sit amet ...\n");
    expected.push_str("  12 | -- text");

    let document = parse(input.trim_end()).expect("Document should parse");

    let snippet = document.snippet_with_options(&TextPrinter, true);

    assert_eq!(snippet, expected);
}
