use std::io;
use std::path::{Path, PathBuf};
use std::fs;
// use chrono::prelude::*;

// See: https://help.ableton.com/hc/en-us/articles/209769625-Live-specific-file-types
// TODO: Add more as needed
// const LIVE_SET_FILE_EXTENSION: &str = ".als";
// const LIVE_CLIP_FILE_EXTENSION: &str = ".alc";

// enum LiveFileType {
//     Set,
//     Clip
// }

#[allow(dead_code)]
#[derive(Debug)]
pub struct Template {
    name: String,
    version: u8,
    path: PathBuf,
}

impl Template {
    #[allow(dead_code)]
    fn new(name: String, version: u8, path: PathBuf) -> Self {
        Template {
            name,
            version,
            path
        }
    }
}

pub fn list_templates() -> io::Result<()> {
    // TODO: Create Path using &str from config
    let templates_dir = Path::new("/Users/kalynbeach/Music/Ableton/User Library/Templates");
    println!("* Templates dir: {:?}", templates_dir);

    for entry in fs::read_dir(templates_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            // TODO: Validate file extension (must be '.als')
            println!("** Template: {:#?}", entry.file_name());
        }
    }

    Ok(())
}

// pub fn index_templates(templates_dir: PathBuf) {
//     // Open dir at templates_dir path
//     // For each .als file in templates_dir:
//     // - Create a new Template
//     // - Add new Template to Templates Vector
// }

// pub struct Project {
//     title: String,
//     artist: String,
// }

// pub struct Set {
//     title: String,
//     created: DateTime<Local>,
//     modified: DateTime<Local>,
// }

// pub struct Clip {
//     name: String,
// }