use std::path::PathBuf;
use clap::{Parser, Subcommand};

pub mod audio;
pub mod ableton;
pub mod elektron;
pub mod music;

// See: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

#[derive(Parser)]
#[clap(name = "WaveTools")]
#[clap(author = "Kalyn Beach")]
#[clap(version = "0.1.0")]
#[clap(about = "Audio and music production tools", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(short, long, value_parser, value_name = "FILE")]
    file_path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    // Calls test_audio
    Test { name: Option<String> }
}

fn main() {
    println!("\n w a v e _ t o o l s \n");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Test { name } => {
            println!("Test file name is: {:?}", name);
            let context = audio::create_audio_context();
            let file_path = String::from("samples/test_beat.wav");
            audio::test_audio(&context, file_path);
        }
    }

    // let context = audio::create_audio_context();
    // let file_path = String::from("samples/test_beat.wav");
    // audio::test_audio(&context, file_path);
}