extern crate crypto;
extern crate rand;
extern crate base64;


use std::str;

use rand::{thread_rng, Rng};
use base64::{encode, decode};
use crypto::{symmetriccipher, buffer, aes, blockmodes};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};


fn aes_256_ecb_encrypt(
    data: &[u8],
    key: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::ecb_encryptor(aes::KeySize::KeySize256, key, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}


fn aes_256_ecb_decrypt(
    encrypted_data: &[u8],
    key: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize256, key, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}


pub fn encrypt(buf: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    match aes_256_ecb_encrypt(buf, key) {
        Ok(encrypted_data) => Ok(encode(encrypted_data.as_slice()).into_bytes()),
        Err(e) => Err(format!("{:?}", e)),
    }
}


pub fn decrypt(buf: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    match decode(buf) {
        Ok(encrypted_data) => {
            match aes_256_ecb_decrypt(encrypted_data.as_slice(), key) {
                Ok(decrypted_data) => Ok(decrypted_data),
                Err(e) => Err(format!("{:?}", e)),
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}


pub fn genattr() -> String {
    format!(
        "\
* filter=git-mix
.gitattributes !filter
"
    )
}


pub fn gen(specifed: Option<&str>) -> String {
    let key;

    match specifed {
        Some(s) => key = s.to_string(),
        None => key = genkey(32),
    }

    format!(
        "\
[filter \"git-mix\"]
    clean = git-mix encrypt --key {key}
    smudge = git-mix decrypt --key {key}
",
        key = key
    )
}


pub fn genkey(len: usize) -> String {
    thread_rng().gen_ascii_chars().take(len).collect::<String>()
}
