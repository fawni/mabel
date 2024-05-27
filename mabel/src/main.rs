// mabel, declarative pixel art
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser, Args
};
#[cfg(feature = "aseprite")]
use clap::Subcommand;
use mabel::Mabel;

const fn clap_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Yellow.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

/// mabel, declarative pixel art
#[derive(Parser)]
#[clap(version, author, styles = clap_style())]
struct MabelArgs {
    #[cfg(feature = "aseprite")]
    #[command(subcommand)]
    command: Option<Command>,

    /// The path to the mabel file
    #[arg()]
    file: Option<String>,

    /// The path to the output file
    #[arg(short, long)]
    output: Option<String>,
}

#[cfg(feature = "aseprite")]
#[derive(Subcommand)]
enum Command {
    /// Convert an aseprite file to eno
    Aseprite(Aseprite),
}

#[derive(Args)]
#[command(aliases = ["ase"])]
struct Aseprite {
    /// The path to the aseprite/ase file
    #[arg()]
    file: String,

    /// The path to the output file
    #[arg(short, long)]
    output: Option<String>,
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = MabelArgs::parse();

    #[cfg(feature = "aseprite")]
    if let Some(Command::Aseprite(args)) = args.command {
        let output = args.output.unwrap_or("output.eno".to_owned());
        mabel::aseprite::save_to_eno(&args.file, &output)?;

        return Ok(());
    }

    let output = args.output.unwrap_or("output.png".to_owned());
    let mabel = if let Some(file) = args.file {
        Mabel::from_file(&file)?
    } else {
        return Err("No eno file provided.".into());
    };

    mabel.save_png(&output)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[1;31merror\x1b[0;1m: {e}");
        std::process::exit(1);
    }
}
