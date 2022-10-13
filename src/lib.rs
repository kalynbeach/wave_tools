use std::path::PathBuf;
use clap::{Parser, Subcommand};

pub mod audio;
pub mod ableton;
pub mod elektron;
pub mod music;

// See: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
#[derive(Parser)]
#[clap(name = "wave_tools")]
#[clap(author = "Kalyn Beach")]
#[clap(version = "0.1.0")]
#[clap(about = "Audio and music production tools", long_about = None)]
pub struct Cli {
    /// Set a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging info on
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Calls test_rodio
    Test { file_path: Option<PathBuf> },
    Templates
}