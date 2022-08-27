use std::fs::File;
use web_audio_api::AudioBuffer;
use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::{AudioNode, AudioScheduledSourceNode};

pub mod music;

pub fn create_audio_context() -> AudioContext {
    println!("Creating audio context...");
    let context = AudioContext::default();
    context
}

pub fn create_audio_buffer(context: &AudioContext, file_path: String) -> AudioBuffer {
    println!("Creating audio buffer...");
    let file = File::open(file_path).unwrap();
    let buffer = context.decode_audio_data_sync(file).unwrap();
    buffer
}

// pub fn get_bpm(buffer: AudioBuffer) -> i32 {}

// pub fn get_key(buffer: AudioBuffer) -> String {}

pub fn test_audio(context: &AudioContext, file_path: String) {
    let buffer = create_audio_buffer(&context, file_path);

    println!("Setting gain level...");
    let volume = context.create_gain();
    volume.connect(&context.destination());
    volume.gain().set_value(0.5);

    println!("Creating buffer source...");
    let buffer_source = context.create_buffer_source();
    buffer_source.connect(&volume);
    buffer_source.set_buffer(buffer);
    buffer_source.start();

    // Listen
    println!("Listening...");
    std::thread::sleep(std::time::Duration::from_secs(10));

    println!("Done. \n");
}

