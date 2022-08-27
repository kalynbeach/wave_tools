pub mod audio;
pub mod ableton;
pub mod elektron;

fn main() {
    println!("\n w a v e _ t o o l s \n");

    let context = audio::create_audio_context();
    let file_path = String::from("samples/test_beat.wav");

    audio::test_audio(&context, file_path);
}