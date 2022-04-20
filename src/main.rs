use std::{env, fs};
use std::error::Error;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

use chrono::prelude::{TimeZone, Utc};
use chrono::SecondsFormat;
use filetime::FileTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct File {
    name: String,
    size: u64,
    permissions: String,
    last_modified: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        if metadata.is_file() {
            let lmt = FileTime::from_last_modification_time(&metadata);
            let mode = format!("{}", &format!("{:o}", metadata.permissions().mode())[3..]);
            let file = File {
                name: entry.file_name().into_string().unwrap(),
                size: metadata.size(),
                permissions: mode,
                last_modified: Utc.timestamp(lmt.seconds(), lmt.nanoseconds()).to_rfc3339_opts(SecondsFormat::Micros, true),
            };
            println!("{}", serde_json::to_string_pretty(&file)?);
        }
    }

    Ok(())
}

