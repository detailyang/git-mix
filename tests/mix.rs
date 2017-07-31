#[cfg(test)]
mod tests {
    extern crate mix;

    #[test]
    fn test_genkey() {
        let key = mix::genkey(32);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_simple_encrypt_and_decrypt() {
        let plain = "abcdefghijklmopqrs".as_bytes();
        let key = "EXHcE7JQDy8vSBDVTTsgg4NkCQUfgqDx".as_bytes();
        let cipher = mix::encrypt(plain, key).unwrap();
        let plainplain = mix::decrypt(cipher.as_bytes(), key).unwrap();
        assert_eq!(plain, plainplain.as_bytes());
    }

    #[test]
    fn test_more_encrypt_and_decrypt() {
        let mut plain;
        let mut key;
        let mut cipher;
        let mut plainplain;

        for _ in 0..1000 {
            plain = mix::genkey(1024);
            key = mix::genkey(32);
            cipher = mix::encrypt(plain.as_bytes(), key.as_bytes()).unwrap();
            plainplain = mix::decrypt(cipher.as_bytes(), key.as_bytes()).unwrap();
            assert_eq!(plain, plainplain);
        }
    }

    #[test]
    fn test_mixed_decrypt() {
        let key = "EXHcE7JQDy8vSBDVTTsgg4NkCQUfgqDx".as_bytes();
        let plain = mix::decrypt("abcd".as_bytes(), key);
        assert_eq!(plain, Err("InvalidLength".to_string()));
    }

    #[test]
    fn test_generate_template() {
        let key = "abcdef".to_string();
        let expect = format!(
            "\
[filter \"git-mix\"]
    clean = git-mix encrypt --key {key}
    smudge = git-mix decrypt --key {key}
",
            key = key
        );
        let actual = mix::gen(Some(key.as_str()));

        assert_eq!(actual, expect);
    }
}
