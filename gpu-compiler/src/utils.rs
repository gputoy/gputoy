#![cfg(test)]

use gpu_common::{File, Files, SupportedExtension};
use std::{io::Read, path::Path};
use walkdir::WalkDir;

/// USED FOR TESTING PURPOSES ONLY
///
/// Converts directory to gpu_common::Files
/// Writing out an entire Files struct in JSON is too cumbersome, especially dealing
/// with the fact the file contents can't be formatted.
/// This allows test Files to be created from a normal directory.
pub fn files_from_dir<P: AsRef<Path>>(path: P) -> Option<Files> {
    let dir = WalkDir::new(&path).into_iter().filter_map(|e| e.ok());
    let mut ret = Files::default();
    for entry in dir {
        let Ok(metadata) = &entry.metadata() else {
            continue
        };
        if metadata.is_dir() {
            continue;
        }
        let fileid = format!(
            "/{}",
            entry.path().strip_prefix(&path).unwrap().to_str().unwrap()
        );
        let mut file = std::fs::File::open(entry.path()).unwrap();
        let mut data = String::new();
        let _ = file.read_to_string(&mut data);

        let dir: Vec<&str> = fileid.split("/").collect();
        let dir = dir
            .len()
            .checked_sub(2)
            .map(|i| dir.get(i))
            .flatten()
            .unwrap()
            .to_string();
        let (file_name, extension) = entry.file_name().to_str().unwrap().split_once(".").unwrap();
        ret.map.insert(
            fileid,
            File {
                data,
                file_name: file_name.into(),
                extension: SupportedExtension::try_from(extension).unwrap(),
                dir,
                fetch: None,
            },
        );
    }
    Some(ret)
}
