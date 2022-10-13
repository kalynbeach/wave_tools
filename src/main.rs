use std::path::Path;
use clap::Parser;
use wave_tools::{audio, ableton, Cli, Commands};

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
            println!("\n[* Test *]\n");
            if let Some(path) = file_path.as_deref() {
                println!("Test file file_path is: {}", path.display());
                let context = audio::create_audio_context();
                audio::test_web_audio(&context, path);
            } else {
                audio::test_rodio();
            }
        }
        Some(Commands::Templates) => {
            println!("\n[* Templates *]\n");
            // TODO: Create Path using &str from config
            let templates_dir_path = Path::new("/Users/kalynbeach/Music/Ableton/User Library/Templates");
            ableton::index_templates(templates_dir_path).unwrap();
        }
        Some(Commands::Projects) => {
            println!("\n[* Projects *]\n");
            let projects_root_path = Path::new("/Users/kalynbeach/Music/Ableton/Projects");
            ableton::index_projects(projects_root_path).unwrap();
        }
        None => {}
    }
}