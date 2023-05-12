pub mod commands;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use commands::{assets, cores, json2rust, overclocks};

#[derive(Subcommand, Debug)]
enum Command {
    /// Dump assets from pak file.
    Assets {
        /// Pak file path.
        #[arg(short, long = "pak")]
        pak_file: PathBuf,
        /// Output directory.
        #[arg(short, long = "out")]
        out_dir: PathBuf,
    },
    /// Dump overclocks from assets.
    Overclocks {
        /// Direcotry with all game assets.
        #[arg(short = 'd', long = "dir")]
        asset_dir: PathBuf,
        /// Write to json.
        #[arg(short = 'o', long = "out")]
        out_file: PathBuf,
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
        /// Directory with jsons.
        #[arg(short, long)]
        input: PathBuf,
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
        Command::Assets { pak_file, out_dir } => assets::run(&pak_file, &out_dir),
        Command::Overclocks {
            asset_dir,
            out_file,
        } => overclocks::run(&asset_dir, out_file),
        Command::Cores { asset_dir, out } => cores::run(&asset_dir, out),
        Command::Json2Rust { input } => json2rust::run(&input),
    }
}
