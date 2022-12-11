#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use aubio::{Smpl, Tempo, OnsetMode, Error};
use hound::{WavReader, WavSamples};

const WAV_FILE_EXT: &str = ".wav";
const MP3_FILE_EXT: &str = ".mp3";

#[derive(Debug)]
pub struct Record {
    title: String,
    artist: String,
    path: PathBuf
}

impl Record {
    fn new(
        title: String,
        artist: String,
        path: PathBuf
    ) -> Self {
        Record { title, artist, path }
    }
}

#[derive(Debug)]
enum SampleFormats {
    WAV,
    MP3
}

#[derive(Debug)]
pub struct Sample {
    name: String,
    format: SampleFormats,
    path: PathBuf
}

impl Sample {
    fn new(
        name: String,
        format: SampleFormats,
        path: PathBuf
    ) -> Self {
        Sample { name, format, path }
    }
}

fn is_ignored(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[allow(unused)]
pub fn index_entities<T>(root_dir: &Path) -> io::Result<Vec<T>> {
    let mut entities: Vec<T> = Vec::new();
    let walker = WalkDir::new(root_dir).into_iter();

    for entry in walker.filter_entry(|e| !is_ignored(e)) {
        let entry = entry?;
        let entry_file_name = entry.file_name().to_str().unwrap();
        // TODO: Filter for files with extension specified under type T
        // TODO: Build T struct from matched file data
    }

    println!("Entities: {}", entities.len());
    Ok(entities)
}

const BUF_SIZE: usize = 512;
const HOP_SIZE: usize = 256;
const I16_TO_SMPL: Smpl = 1.0 / (1 << 16) as Smpl;

pub fn get_bpm(file_name: &Path) -> Result<f32, Error> {
    // TODO[?]: Convert input WAV file to 16-bit mono for analysis with aubio
    // > ffmpeg -i input.wav -acodec pcm_s16le -ac 1 output.wav

    println!("[get_bpm] Initializing with file {:?}...", file_name.as_os_str());
    let mut file_reader = WavReader::open(file_name).unwrap();
    let file_format = file_reader.spec();
    let mut file_samples: WavSamples<BufReader<File>, i16> = file_reader.samples();
    let mut file_tempo = Tempo::new(
        OnsetMode::Complex,
        BUF_SIZE,
        HOP_SIZE,
        file_format.sample_rate
    ).unwrap();

    println!("[get_bpm] Computing...");
    loop {
        let block = file_samples
            .by_ref()
            .map(|sample| sample.map(|sample: i16| sample as Smpl * I16_TO_SMPL))
            .take(HOP_SIZE)
            .collect::<Result<Vec<Smpl>, _>>()
            .unwrap();
        if block.len() == HOP_SIZE {
            file_tempo.do_result(block.as_slice().as_ref())?;
        }
        if block.len() < HOP_SIZE {
            break;
        }
    }

    let bpm = file_tempo.get_bpm();
    println!("[get_bpm] BPM: {:}", bpm);
    Ok(bpm)
}

pub fn get_key(_file_path: &Path) -> &str {
    // TODO: Compute key of music file
    let key = "C";
    println!("[get_key] Key: {key}");
    key
}