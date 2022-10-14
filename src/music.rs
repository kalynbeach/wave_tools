#![allow(dead_code)]
use std::path::{Path, PathBuf};

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

pub fn get_bpm(_file_path: &Path) -> i32 {
    // TODO: Compute BPM of music file
    let bpm = 0;
    println!("[get_bpm] BPM: {bpm}");
    bpm
}

pub fn get_key(_file_path: &Path) -> &str {
    // TODO: Compute key of music file
    let key = "C";
    println!("[get_key] Key: {key}");
    key
}