use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::time::Duration;
use web_audio_api::AudioBuffer;
use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::{AudioNode, AudioScheduledSourceNode};
use rodio::{Decoder, OutputStream, Sink};

pub fn test_rodio() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open("samples/test_beat.wav").unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

pub fn create_audio_context() -> AudioContext {
    println!("Creating audio context...");
    let context = AudioContext::default();
    context
}

pub fn create_audio_buffer(context: &AudioContext, file_path: &Path) -> AudioBuffer {
    println!("Creating audio buffer...");
    let file = File::open(file_path).unwrap();
    let buffer = context.decode_audio_data_sync(file).unwrap();
    buffer
}

pub fn test_web_audio(context: &AudioContext, file_path: &Path) {
    println!("\n[test_audio]");
    let buffer = create_audio_buffer(&context, file_path);
    let buffer_duration = buffer.duration() as u64;

    println!("Setting gain level...");
    let volume = context.create_gain();
    volume.connect(&context.destination());
    volume.gain().set_value(0.5);

    println!("Creating buffer source...");
    let buffer_source = context.create_buffer_source();
    buffer_source.connect(&volume);
    buffer_source.set_buffer(buffer);
    buffer_source.start();

    println!("Listening...");
    std::thread::sleep(Duration::from_secs(buffer_duration));

    println!("Done. \n");
}

