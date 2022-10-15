use std::path::Path;
use clap::Parser;
use wave_tools::{audio, music, ableton, Cli, Commands};

fn main() {
    println!("\n w a v e _ t o o l s \n");

    let cli = Cli::parse();

    match &cli.command {
        // Audio commands
        Commands::Test { file_path } => {
            println!("\n[* Audio - Test *]\n");
            if let Some(path) = file_path.as_ref() {
                println!("Test file file_path is: {}", path.display());
                // let context = audio::create_audio_context();
                // audio::test_web_audio(&context, path);
                // audio::test_aubio(path);
                audio::test_aubio();
            } else {
                // audio::test_cpal();
                audio::test_aubio();
            }
        }

        // Music commands
        Commands::Bpm { file_path } => {
            println!("\n[* Music - BPM *]\n");
            if let Some(path) = file_path.as_deref() {
                music::get_bpm(path);
            }
        }
        Commands::Key { file_path } => {
            println!("\n[* Music - Key *]\n");
            if let Some(path) = file_path.as_deref() {
                music::get_key(path);
            }
        }

        // Ableton commands
        Commands::Templates => {
            println!("\n[* Ableton - Templates *]\n");
            // TODO: Create Path using &str from config
            let templates_dir_path = Path::new(
                "/Users/kalynbeach/Music/Ableton/User Library/Templates"
            );
            ableton::index_templates(templates_dir_path).unwrap();
        }
        Commands::Projects => {
            println!("\n[* Ableton - Projects *]\n");
            // TODO: Create Path using &str from config
            let projects_root_path = Path::new(
                "/Users/kalynbeach/Music/Ableton/Projects"
            );
            ableton::index_projects(projects_root_path).unwrap();
        }
    }
}