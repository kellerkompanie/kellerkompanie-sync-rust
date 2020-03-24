extern crate ring;

use std::fs::File;
use std::io::{BufReader, Read};

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use walkdir::DirEntry;

pub fn hash_file(path: &String) -> String {
    let input = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening file {}: {:?}", path, error)
        }
    };
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader);

    HEXUPPER.encode(digest.as_ref())
}

const BUFFER_SIZE: usize = 1024;

fn sha256_digest<R: Read>(mut reader: R) -> Digest {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let count = reader.read(&mut buffer).expect("");
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    context.finish()
}