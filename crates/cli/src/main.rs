use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use coverwin_core::Format;

#[derive(Parser)]
#[command(
    name = "coverwin-cli",
    version,
    about = "Conversor de imagens do coverwin"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Converte um ou mais arquivos. Ex: coverwin-cli convert a.png b.jpg --to webp
    Convert {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// png, jpg, webp, avif, bmp, gif, tiff, ico
        #[arg(long, short)]
        to: Format,
        /// 1–100 (JPG, WebP, AVIF)
        #[arg(long, short, default_value_t = 85, value_parser = clap::value_parser!(u8).range(1..=100))]
        quality: u8,
    },
}

fn main() -> ExitCode {
    let Command::Convert { files, to, quality } = Cli::parse().command;
    let mut failed = false;
    for file in &files {
        match coverwin_core::convert(file, to, quality) {
            Ok(out) => println!("{} -> {}", file.display(), out.display()),
            Err(e) => {
                eprintln!("{}: {e}", file.display());
                failed = true;
            }
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
