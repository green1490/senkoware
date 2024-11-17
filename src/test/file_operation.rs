use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm //
};
use core::str;
use std::{error::Error, fs::{File, OpenOptions}, io::{self, BufRead, BufReader, BufWriter, Read, Seek, Write}, path::Path, vec};

#[test]
fn encryption() {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref()).unwrap();
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    assert_eq!(&plaintext, b"plaintext message");
}

#[test]
fn file_reading_as_bytes() -> Result<(),std::io::Error> {
    let mut file = File::open("src/test/test_file.txt")?;
    let mut buf:Vec<u8> = vec![];
    let reader = io::BufReader::new(file);

    for byte in reader.bytes() {
        buf.push(byte?);
    }
    let s = String::from_utf8(buf).unwrap();
    assert_eq!(s, "im ready being encrypted!");
    Ok(())
}

#[test]
fn file_encryption() -> Result<(), std::io::Error> {
    // init for encryption
    const BUFFER_SIZE: usize = 512;
    let key = Aes256Gcm::generate_key(OsRng);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let test_file_path = Path::new("src/test/test_file.txt");


    let mut file =  OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(test_file_path)?;

    let mut reader = BufReader::with_capacity(BUFFER_SIZE,  file.try_clone()?);
    
    // reading the file and ecrypting its content
    loop {
        let starting_pos = reader.stream_position()?;
        let buffer = reader.fill_buf()?;
        let buffer_length = buffer.len();

        if buffer_length == 0 {
            break;
        }

        let encrypted_buffer= cipher.encrypt(&nonce, buffer).unwrap_or(vec![]);

        //swappint the file content
        file.seek(io::SeekFrom::Start(starting_pos))?;
        file.write_all(&encrypted_buffer.as_slice())?;
        file.sync_data()?;
        // All bytes consumed from the buffer
        // should not be read again.
        file.sync_all()?;
        reader.consume(buffer_length);
    }
    println!("{:?}", key);
    Ok(())
}