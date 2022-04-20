use std::fs;
use std::env::current_dir;
use std::error::Error;
use std::fmt::Display;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use chrono::prelude::{TimeZone, Utc};
use chrono::SecondsFormat;
use clap::Parser;
use filetime::FileTime;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
struct Directory {
    name: String,
    contents: Vec<File>,
}

fn as_string<S,N: ToString>(n: N, s: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
{
    s.serialize_str(&*n.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    name: String,
    #[serde(serialize_with = "as_string")]
    size: u64,
    #[serde(serialize_with = "as_string")]
    permissions: u16,
    last_modified: String,
}

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

