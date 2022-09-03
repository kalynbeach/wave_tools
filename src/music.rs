#![allow(dead_code)]
use std::path::PathBuf;

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

// pub fn get_bpm(buffer: AudioBuffer) -> i32 {}

// pub fn get_key(buffer: AudioBuffer) -> String {}