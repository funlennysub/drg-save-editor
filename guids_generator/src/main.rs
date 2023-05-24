mod commands;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use commands::{assets, cores::CoresCommand, json2rust, sav2json};

#[derive(Subcommand, Debug)]
enum Command {
    /// Dump assets from pak file.
    Assets {
        /// Pak file path.
        #[arg(short, long = "pak")]
        pak_file: PathBuf,
        /// Output directory.
        #[arg(short, long = "out")]
        out: PathBuf,
    },
    /// Dump vanity/resource/victory pose/skin/cosmetic matrix cores.
    Cores {
        /// Direcotry with all game assets.
        #[arg(short = 'd', long = "dir")]
        asset_dir: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
    #[command(name = "json2rust")]
    Json2Rust {
        /// JSON file.
        #[arg(short, long)]
        file: PathBuf,
        /// Output file.
        #[arg(short, long)]
        out: PathBuf,
    },
    #[command(name = "sav2json")]
    Sav2Json {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Assets { pak_file, out } => assets::run(&pak_file, &out),
        Command::Cores { asset_dir, out } => CoresCommand::new(asset_dir, out).run(),
        Command::Json2Rust { file, out } => json2rust::run(file, out),
        Command::Sav2Json { file, out } => sav2json::run(file, out),
        
    }
}
