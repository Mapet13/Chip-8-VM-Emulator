use std::{
    fs::{self, File},
    io::{Read, Write},
};

pub fn read_file_as_bytes(filename: &str, max_size: usize) -> Result<Vec<u8>, String> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let metadata_len = metadata.len() as usize;
    if metadata_len > max_size {
        return Err("File is to big".to_string());
    }
    let mut buffer = vec![0; metadata_len];
    match f.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err("Error with reading ROM file".to_string()),
    }
}

pub fn byte_copy(from: &[u8], mut to: &mut [u8]) -> usize {
    to.write(from).unwrap()
}
