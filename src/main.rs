use std::fs::File;
use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::{AudioNode, AudioScheduledSourceNode};

fn main() {
    println!("\n w a v e _ t o o l s \n");

    let file_path = String::from("samples/test_beat.wav");

    // Set up audio context
    let context = AudioContext::default();

    // Open local file and create an audio buffer
    let file = File::open(file_path).unwrap();
    let buffer = context.decode_audio_data_sync(file).unwrap();

    // Play the buffer at given volume
    let volume = context.create_gain();
    volume.connect(&context.destination());
    volume.gain().set_value(0.5);

    let buffer_source = context.create_buffer_source();
    buffer_source.connect(&volume);
    buffer_source.set_buffer(buffer);

    // Start the source
    buffer_source.start();

    // Listen
    std::thread::sleep(std::time::Duration::from_secs(10));
}
