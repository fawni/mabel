use indoc::formatdoc;
use mabel_eno::parse_with_printer;

fn main() {
    let input = formatdoc!(
        r#"
        # section
        field:
        attribute = <test></test>

        flag

        -- embed
        it
        is
        ok
        -- embed
    "#
    );

    let document = parse_with_printer(&input, Box::new(mabel_eno::TextPrinter)).unwrap();

    println!("************ document");

    println!(
        "{}",
        document.snippet_with_options(&mabel_eno::TerminalPrinter, true)
    );

    println!("************ section");

    println!(
        "{}",
        document.section("section").unwrap().snippet().unwrap()
    );

    println!("************ field");

    println!(
        "{}",
        document
            .section("section")
            .unwrap()
            .field("field")
            .unwrap()
            .snippet()
            .unwrap()
    );

    println!("************ attribute");

    println!(
        "{}",
        document
            .section("section")
            .unwrap()
            .field("field")
            .unwrap()
            .attribute("attribute")
            .unwrap()
            .snippet()
            .unwrap()
    );

    println!("************ flag");

    println!(
        "{}",
        document
            .section("section")
            .unwrap()
            .flag("flag")
            .unwrap()
            .snippet()
            .unwrap()
    );

    println!("************ embed");

    println!(
        "{}",
        document
            .section("section")
            .unwrap()
            .embed("embed")
            .unwrap()
            .snippet()
            .unwrap()
    );
}
