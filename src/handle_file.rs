use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer, Serialize,
};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::os::unix::fs::FileExt;

pub fn extract_file_by_position(mut file: File, end_position: u64, start_position: u64) -> Vec<u8> {
    let mut chunk_size = 4194303;
    let chunk_position = start_position;
    // let mut buffer = [0u8; 4194303];

    let chunk_size = (end_position - start_position) as usize;

    let mut buffer = Vec::with_capacity(chunk_size); // empty bytes

    // Take the file from given position
    file.seek(SeekFrom::Start(start_position)).unwrap();
    file.take(chunk_size as u64)
        .read_to_end(&mut buffer) // fill the empty bytes starting from where we seek.
        .unwrap();
    buffer
}
