use std::fs::File;
use std::path::Path;
// use std::io::{stdin, BufReader};
use std::time::Duration;
// use aubio::{Notes, Smpl};
// use hound::WavReader;
use cpal::Host;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use rodio::{Decoder, OutputStream, Sink};
use web_audio_api::AudioBuffer;
use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::{AudioNode, AudioScheduledSourceNode};
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader};
// use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
// use symphonia::core::probe::Hint;
use symphonia::core::audio::{AudioBufferRef, Signal, SampleBuffer};
use symphonia::default::formats::WavReader;


// symphonia experiments

pub fn test_symphonia(file_path: &Path) {
    let file = Box::new(File::open(&file_path).unwrap());
    let stream = MediaSourceStream::new(file, Default::default());

    // let mut hint = Hint::new();
    // hint.with_extension("wav");

    // let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();
    let dec_opts: DecoderOptions = Default::default();

    // let probed = symphonia::default::get_probe()
    //     .format(&hint, stream, &fmt_opts, &meta_opts)
    //     .unwrap();
    
    // let mut format = probed.format;

    let mut format = WavReader::try_new(stream, &fmt_opts).unwrap();

    let track = format.default_track().unwrap();

    // let track = format
    //     .tracks()
    //     .iter()
    //     .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
    //     .expect("no supported audio tracks");

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .unwrap();
    
    // Track id is used to filter packets
    let track_id = track.id;
    println!("track_id: {}", track_id);

    let mut sample_count = 0;
    let mut sample_buf = None;

    // The decode loop
    loop {
        // Get the next packet from the media format
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                unimplemented!();
            },
            Err(_) => break,
        };

        // let packet = format.next_packet().unwrap();

        // Consume any new metadata that has been read since the last packet
        while !format.metadata().is_latest() {
            format.metadata().pop();
            // TODO: Consume the new metadata at the head of the metadata queue
        }

        // If the packet does not belong to the selected track, skip it
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // Consume the decoded audio samples

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();

                    println!("{:?}", spec);

                    // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                    let duration = audio_buf.capacity() as u64;

                    // Create the f32 sample buffer.
                    sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }

                // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);

                    // The samples may now be access via the `samples()` function.
                    sample_count += buf.samples().len();
                    print!("\rDecoded {} samples", sample_count);
                }

                // match audio_buf {
                //     AudioBufferRef::F32(buf) => {
                //         for &sample in buf.chan(0) {
                //             println!("{}", &sample);
                //         }
                //     }
                //     _ => {
                //         // Repeat for the different sample formats
                //         // unimplemented!();
                //         println!("{}", _);
                //         ()
                //     }
                // }
            }
            Err(Error::IoError(_)) => (),
            Err(Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }
}

// aubio + hound experiments

// const BUF_SIZE: usize = 512;
// const HOP_SIZE: usize = 256;
// const I16_TO_SMPL: Smpl = 1.0 / (1 << 16) as Smpl;

// pub fn test_aubio() {
//     println!("Initializing...");
//     let input = stdin();
//     let mut reader = WavReader::new(input).unwrap();
//     let format = reader.spec();
//     let mut samples = reader.samples();
//     let mut notes = Notes::new(BUF_SIZE, HOP_SIZE, format.sample_rate).unwrap();
//     let period = 1.0 / format.sample_rate as Smpl;
//     let mut time: f32 = 0.0;
//     let mut offset = 0;

//     println!("Computing...");
//     loop {
//         let block = samples
//             .by_ref()
//             .map(|sample| sample.map(|sample: i16| sample as Smpl * I16_TO_SMPL))
//             .take(HOP_SIZE)
//             .collect::<Result<Vec<Smpl>, _>>()
//             .unwrap();
        
//         if block.len() == HOP_SIZE {
//             for note in notes.do_result(block.as_slice().as_ref()).unwrap() {
//                 if note.velocity > 0.0 {
//                     println!("Time: {}s -> Pitch: {}", time, note.pitch);
//                 } else {
//                     println!("Time: {}s", time);
//                 }
//             }
//         }

//         offset += block.len();
//         time = offset as Smpl * period;

//         if block.len() < HOP_SIZE {
//             break;
//         }
//     }
// }

// cpal experiments

pub fn list_devices(host: &Host) {
    let devices = host.devices().unwrap();
    for (device_index, device) in devices.enumerate() {
        println!("  {}. \"{}\"", device_index + 1, device.name().unwrap());
    }
}

pub fn create_stream(host: &Host) {
    let device = host.default_output_device().expect("No output device available");

    let mut supported_configs_range = device.supported_output_configs()
        .expect("Error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("No supported config")
        .with_max_sample_rate();
    let config = supported_config.config();

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // React to stream events and read or write stream data here
            println!("{:?}", data);
        },
        move |err| {
            eprintln!("an error occurred on stream: {}", err);
        },
    ).unwrap();

    stream.play().unwrap();
}

pub fn test_cpal() {
    let host = cpal::default_host();
    list_devices(&host);
}

// rodio experiments

// pub fn test_rodio() {
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let sink = Sink::try_new(&stream_handle).unwrap();

//     let file = BufReader::new(File::open("samples/test_beat.wav").unwrap());
//     let source = Decoder::new(file).unwrap();

//     sink.append(source);
//     sink.sleep_until_end();
// }

// web_audio_api experiments

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

