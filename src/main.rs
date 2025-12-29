use std::{fs, path::PathBuf, process};

use clap::Parser;

use crate::error::{MiniPngError, Result};
use crate::mininpng::MiniPNG;

mod error;
mod mininpng;
mod mininpng_tests;
mod pixel_type;

/// CLI for the mini png exercices
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    Display {
        /// paths of the Mini-PNG images
        paths: Vec<PathBuf>,
    },
    Encode {
        /// path of the input text file
        input: PathBuf,

        /// path of the output Mini-PNG file
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn run() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Display { paths } => {
            for path in paths {
                let bytes = fs::read(&path).map_err(|e| MiniPngError::FileRead {
                    path: path.display().to_string(),
                    source: e,
                })?;
                let image: MiniPNG = bytes.try_into()?;
                let output = image.display()?;
                println!("{}", output);
            }
        }
        Commands::Encode { input, output } => {
            let text = fs::read_to_string(&input).map_err(|e| MiniPngError::FileRead {
                path: input.display().to_string(),
                source: e,
            })?;
            let image = MiniPNG::bw_from_string(&text)?;
            image.save(output)?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
