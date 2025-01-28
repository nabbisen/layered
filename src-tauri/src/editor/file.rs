use super::content::compose;
use super::types::ParsedMarkdown;
use std::fs::File;
use std::io::prelude::*;

pub fn open(filepath: &str) -> String {
    let mut file = File::open(filepath).expect(format!("Failed to open: {}", filepath).as_str());
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect(format!("Failed to read: {}", filepath).as_str());
    content
}

pub fn save(parsed_markdowns: Vec<ParsedMarkdown>, filepath: &str) -> () {
    // // todo: file select dialog
    // let home_path =
    // // Linux / Unix / Mac
    // std::env::var("HOME")
    //     .ok()
    //     // Windows
    //     .or_else(|| std::env::var("USERPROFILE").ok());
    // let mut filepath = std::path::PathBuf::from(home_path.unwrap());
    // // Linux / Unix / Mac / Windows
    // filepath.push("Desktop");
    // filepath.push("layered-saved.md");

    let content = compose(parsed_markdowns);

    let mut file = File::create(filepath).expect("Failed to create file to save");
    file.write_all(content.as_bytes()).expect("Failed to save");
}
