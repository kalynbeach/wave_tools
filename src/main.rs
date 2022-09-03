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
struct Cli {
    /// Set a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging info on
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Calls test_audio
    Test { file_path: Option<PathBuf> },
    Templates
}

fn main() {
    println!("\n w a v e _ t o o l s \n");

    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Bruh"),
    }

    match &cli.command {
        Some(Commands::Test { file_path }) => {
            println!("\n[* Test *]");
            if let Some(path) = file_path.as_deref() {
                println!("Test file file_path is: {}", path.display());
                let context = audio::create_audio_context();
                audio::test_web_audio(&context, path);
            } else {
                audio::test_rodio();
            }
        }
        Some(Commands::Templates) => {
            println!("\n[* Templates *]");
            ableton::list_templates().unwrap();
        }
        None => {}
    }
}