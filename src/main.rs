use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::io::{self, Read};
use urlencoding::{decode, encode};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// String to process (if omitted, reads from stdin)
    input: Option<String>,

    /// Mode of operation (encode or decode). Defaults to encode.
    #[arg(short, long, default_value = "encode")]
    mode: Mode,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Mode {
    Encode,
    Decode,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut input = String::new();
    if let Some(i) = args.input {
        input = i;
    } else {
        io::stdin()
            .read_to_string(&mut input)
            .context("Failed to read from stdin")?;
    }

    match args.mode {
        Mode::Encode => {
            println!("{}", encode(&input));
        }
        Mode::Decode => {
            let decoded = decode(&input).context("Failed to URL decode string")?;
            println!("{}", decoded);
        }
    }

    Ok(())
}
