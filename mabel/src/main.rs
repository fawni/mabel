// mabel, a declarative image generation tool
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use mabel::Mabel;

const fn clap_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Yellow.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

/// mabel, a declarative image generation tool
#[derive(Parser)]
#[clap(version, author, styles = clap_style())]
struct Args {
    /// The path to the mabel file
    #[arg()]
    file: String,

    /// The path to the output file
    #[arg(short, long)]
    output: Option<String>,
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mabel = Mabel::from_file(&args.file)?;
    let output = args.output.as_deref().unwrap_or("output.png");

    // println!("{mabel:#?}");
    mabel.save_png(output)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[1;31merror\x1b[0;1m: {e}");
        std::process::exit(1);
    }
}
