use std::{
    fs::File,
    io::{Read, Write},
};

pub fn read_file_as_bytes(filename: &str) -> Result<Vec<u8>, String> {
    let mut f = File::open(&filename).expect("no file found");
    let mut buffer = Vec::<u8>::new();
    match f.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err("Error with reading ROM file".to_string()),
    }
}

pub fn byte_copy(from: &[u8], mut to: &mut [u8]) -> usize {
    to.write(from).unwrap()
}
