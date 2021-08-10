use std::fs::File;
use std::path::Path;

use hex::ToHex;
use sha2::{Digest, Sha256};

/// hash_vec returns the sha256 hash sum of a `&[u8]` as a string
pub fn hash_vec(v: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(v);
    let result = hasher.finalize();
    result.as_slice().encode_hex::<String>()
}

/// get_png_bytes returns the bytes of a png file
pub fn get_png_bytes(input: &str) -> Vec<u8> {
    let file = File::open(input).unwrap();
    let decoder = png::Decoder::new(file);
    let (_, mut reader) = decoder.read_info().unwrap();
    let len = reader.output_buffer_size();
    let mut buf = vec![0; len];
    reader.next_frame(&mut buf).unwrap();
    buf
}

/// str_to_vec turns a string into a `Vec<u8>`
/// it was used for headers
pub fn str_to_vec(inp: &str) -> Vec<u8> {
    inp.as_bytes().to_vec()
}

/// Returns the filename of an input file
pub fn file_name<'a>(input: &'a str) -> &'a str {
    Path::new(input).file_name().unwrap().to_str().unwrap()
}

/// This function returns the starting point for a file
/// This uses a weird workaround so it's not very `safe`
/// Might fail if some metadata contains `###SIZE`
pub fn get_starting_point(data: &Vec<u8>) -> String {
    let mut headers = Vec::new();
    for ch in (5..data.len() - 6).rev() {
        // Find the first line that contains stuff like ###SIZE
        if data[ch] == 'E' as u8 {
            if data[ch - 1] != 'Z' as u8 {
                continue;
            }
            if data[ch - 2] != 'I' as u8 {
                continue;
            }
            if data[ch - 3] != 'S' as u8 {
                continue;
            }
            if data[ch - 4] != '#' as u8 {
                continue;
            }
            if data[ch - 5] != '#' as u8 {
                continue;
            }
            if data[ch - 6] != '#' as u8 {
                continue;
            }
            headers = data[ch - 6..].to_owned();
            break;
        }
    }
    if headers.len() == 0 {
        panic!("No info available!");
    }
    std::str::from_utf8(&headers).unwrap().to_owned()
}
