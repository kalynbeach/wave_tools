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