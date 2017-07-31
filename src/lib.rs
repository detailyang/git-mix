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


pub fn encrypt(buf: &[u8], key: &[u8]) -> Result<String, String> {
    match aes_256_ecb_encrypt(buf, key) {
        Ok(encrypted_data) => Ok(encode(encrypted_data.as_slice())),
        Err(e) => Err(format!("{:?}", e)),
    }
}


pub fn decrypt(buf: &[u8], key: &[u8]) -> Result<String, String> {
    match decode(buf) {
        Ok(encrypted_data) => {
            match aes_256_ecb_decrypt(encrypted_data.as_slice(), key) {
                Ok(decrypted_data) => {
                    let plain = str::from_utf8(decrypted_data.as_slice()).unwrap();
                    Ok(plain.to_string())
                }
                Err(e) => Err(format!("{:?}", e)),
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}


pub fn gen(specifed: Option<&str>) -> String {
    let key;

    match specifed {
        Some(s) => key = s.to_string(),
        None => key = genkey(32),
    }

    format!("\
[filter = \"git-mix\"]
    clean = git-mix encrypt --key {key}
    smudge = git-mix decrypt --key {key}
", key=key)
}


pub fn genkey(len: usize) -> String {
    thread_rng().gen_ascii_chars().take(len).collect::<String>()
}


#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_genkey() {
        let key = genkey(32);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let plain = "abcdefghijklmopqrs".as_bytes();
        let key = "EXHcE7JQDy8vSBDVTTsgg4NkCQUfgqDx".as_bytes();
        let cipher = encrypt(plain, key).unwrap();
        let plainplain = decrypt(cipher.as_bytes(), key).unwrap();
        assert_eq!(plain, plainplain.as_bytes());
    }

    #[test]
    fn test_mixed_decrypt() {
        let key = "EXHcE7JQDy8vSBDVTTsgg4NkCQUfgqDx".as_bytes();
        let plain = decrypt("abcd".as_bytes(), key);
        assert_eq!(plain, Err("InvalidLength".to_string()));
    }
}
