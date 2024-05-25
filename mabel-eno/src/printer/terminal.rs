use super::Printer;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

const BRIGHT_BLACK_BACKGROUND: &str = "\x1b[40m";

const BRIGHT_BLACK: &str = "\x1b[90m";
const WHITE: &str = "\x1b[37m";
const BRIGHT_WHITE: &str = "\x1b[97m";

#[derive(Debug)]
pub struct TerminalPrinter;

impl Printer for TerminalPrinter {
    fn comment(&self, comment: &str) -> String {
        format!(r#"{BRIGHT_BLACK}{comment}{RESET}"#)
    }

    fn gutter(&self, line_number: u32) -> String {
        format!("{BRIGHT_BLACK_BACKGROUND} {:>3} {RESET} ", line_number)
    }

    fn key(&self, key: &str) -> String {
        format!(r#"{BOLD}{BRIGHT_WHITE}{key}{RESET}"#)
    }

    fn operator(&self, operator: &str) -> String {
        format!(r#"{WHITE}{operator}{RESET}"#)
    }

    fn value(&self, value: &str) -> String {
        format!(r#"{DIM}{WHITE}{value}{RESET}"#)
    }
}
