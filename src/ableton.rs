#![allow(dead_code)]
use std::{fs, io};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use chrono::prelude::*;

// See: https://help.ableton.com/hc/en-us/articles/209769625-Live-specific-file-types
// TODO: Add more as needed
const LIVE_SET_FILE_EXTENSION: &str = ".als";
const LIVE_CLIP_FILE_EXTENSION: &str = ".alc";

// enum LiveFileType {
//     Set,
//     Clip
// }

#[derive(Debug)]
pub struct Project {
    title: String,
    artist: String,
    collection: String,
    path: PathBuf
}

impl Project {
    fn new(title: String, artist: String, collection: String, path: PathBuf) -> Self {
        Project { title, artist, collection, path }
    }
}

#[derive(Debug)]
pub struct Set {
    title: String,
    created: DateTime<Local>,
    modified: DateTime<Local>,
}

#[derive(Debug)]
pub struct Clip {
    name: String,
}

#[derive(Debug)]
pub struct Template {
    name: String,
    path: PathBuf,
    version: f32,
}

impl Template {
    #[allow(dead_code)]
    fn new(name: String, path: PathBuf, version: f32) -> Self {
        Template {
            name,
            path,
            version
        }
    }
}

fn is_ignored(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| { s.starts_with(".") || s == "Ableton Project Info" })
        .unwrap_or(false)
}

pub fn index_projects(projects_root_path: &Path) -> io::Result<Vec<Project>> {
    let mut projects: Vec<Project> = Vec::new();
    let mut current_collection = String::new();
    let walker = WalkDir::new(projects_root_path).into_iter();

    for entry in walker.filter_entry(|e| !is_ignored(e)) {
        let entry = entry?;
        let entry_file_name = entry.file_name().to_str().unwrap();
        // Handle Project 'Collection' directories
        if entry.depth() == 1 {
            current_collection = String::from(entry_file_name);
        }
        // Handle Project directories
        if entry_file_name.contains("Project") {
            let project_name = entry_file_name.replace(" Project", "");
            // TODO: Replace with artist name str from config or metadata
            let project_artist = String::from("KB");
            let project_collection = String::from(&current_collection);
            let project_path = PathBuf::from(entry.path());
            let project = Project::new(
                    project_name,
                    project_artist,
                    project_collection,
                    project_path
            );
            println!("{:?}", project);
            projects.push(project);
        }
    }

    println!("Projects: {}", projects.len());

    Ok(projects)
}

pub fn index_templates(path: &Path) -> io::Result<Vec<Template>> {
    let mut templates: Vec<Template> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let template_name = entry.file_name().into_string().unwrap();
            let template_path = PathBuf::from(entry.path());
            let template_version = 0.1;
            let template = Template::new(template_name, template_path, template_version);
            println!("{:?}", template);
            templates.push(template);
        }
    }

    Ok(templates)
}

pub fn list_templates(path: &Path) -> io::Result<()> {
    println!("* Templates dir: {:?}", path);

    for entry in fs::read_dir(path)? {
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