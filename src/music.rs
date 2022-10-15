#![allow(dead_code)]
use std::io::{stdin, Stdin};
use std::path::{Path, PathBuf};
use aubio::{Smpl, Tempo, OnsetMode, Error};
use hound::{WavReader, WavSamples};

#[derive(Debug)]
pub struct Record {
    title: String,
    artist: String,
}

#[derive(Debug)]
pub struct Sample {
    name: String,
    path: PathBuf
}

const BUF_SIZE: usize = 512;
const HOP_SIZE: usize = 256;
const I16_TO_SMPL: Smpl = 1.0 / (1 << 16) as Smpl;

pub fn get_bpm() -> Result<f32, Error> {
    // TODO: Use WAV file input from a given path
    // let mut reader = WavReader::open(file_path).unwrap();

    // TODO[?]: Convert input WAV file to 16-bit mono for analysis with aubio

    println!("[get_bpm] Initializing...");
    let input = stdin();
    let mut reader = WavReader::new(input).unwrap();
    let format = reader.spec();
    let mut samples: WavSamples<Stdin, i16> = reader.samples();
    let mut tempo = Tempo::new(
        OnsetMode::Complex,
        BUF_SIZE,
        HOP_SIZE,
        format.sample_rate
    ).unwrap();

    println!("[get_bpm] Computing...");
    loop {
        let block = samples
            .by_ref()
            .map(|sample| sample.map(|sample: i16| sample as Smpl * I16_TO_SMPL))
            .take(HOP_SIZE)
            .collect::<Result<Vec<Smpl>, _>>()
            .unwrap();
        if block.len() == HOP_SIZE {
            tempo.do_result(block.as_slice().as_ref())?;
        }
        if block.len() < HOP_SIZE {
            break;
        }
    }

    let bpm = tempo.get_bpm();
    println!("[get_bpm] BPM: {:}", bpm);
    Ok(bpm)
}

pub fn get_key(_file_path: &Path) -> &str {
    // TODO: Compute key of music file
    let key = "C";
    println!("[get_key] Key: {key}");
    key
}