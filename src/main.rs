mod types;

use std::fs;
use std::env::{current_dir};
use std::error::Error;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use chrono::prelude::{TimeZone, Utc};
use chrono::SecondsFormat;
use clap::Parser;
use filetime::FileTime;

use crate::types::{Directory,File};

#[derive(Parser)]
struct Args {
    #[clap(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let dir = process_directory(args.path.unwrap_or(current_dir()?))?;
    println!("{}", serde_json::to_string_pretty(&dir)?);
    Ok(())
}

fn process_directory(current_dir: PathBuf) -> Result<Directory, Box<dyn Error>> {
    let mut root = Directory {
        name: current_dir.file_name().unwrap().to_os_string().into_string().unwrap(),
        contents: Vec::new(),
    };
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        if metadata.is_file() {
            let lmt = FileTime::from_last_modification_time(&metadata);
            let mode = (&format!("{:o}", metadata.permissions().mode())[3..]).to_string();
            let file = File {
                name: entry.file_name().into_string().unwrap(),
                size: metadata.size(),
                permissions: mode.parse::<u16>().unwrap(),
                last_modified: Utc.timestamp(lmt.seconds(), lmt.nanoseconds()).to_rfc3339_opts(SecondsFormat::Micros, true),
            };
            root.contents.push(file);
        }
    }
    Ok(root)
}

