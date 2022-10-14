use std::path::PathBuf;
use clap::{Parser, Subcommand};

pub mod audio;
pub mod ableton;
pub mod elektron;
pub mod music;

#[derive(Parser)]
#[clap(name = "wave_tools")]
#[command(author = "Kalyn Beach")]
#[command(about = "Audio and music production tools")]
#[command(version, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Audio commands
    /// Call audio::test function set in main
    Test { file_path: Option<PathBuf> },

    // Music commands
    /// Call music::get_bpm on given file
    Bpm { file_path: Option<PathBuf> },
    /// Call music::get_key on given file
    Key { file_path: Option<PathBuf> },

    // Ableton commands
    /// Call ableton::Template functions
    Templates,
    /// Call ableton::Project functions
    Projects,
}