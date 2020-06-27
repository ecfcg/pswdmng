use crypto::digest::Digest;
use crypto::sha2::Sha512;
use crypto::sha3::Sha3;
use rand::seq::SliceRandom;
use rand::thread_rng;

const ASCII_STR: &'static str = r#"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%~+=_-`\|^&*()[]{}:'"<>?,./; "#;

pub(crate) fn create_ascii_str(len: usize) -> String {
    let mut rng = thread_rng();
    String::from_utf8(
        ASCII_STR
            .as_bytes()
            .choose_multiple(&mut rng, len)
            .cloned()
            .collect(),
    )
    .unwrap()
}

pub(crate) fn sha512_hashcode(raw_str: String, salt: String) -> String {
    hashcode(Sha512::new(), raw_str, salt)
}

pub(crate) fn sha3_512_hashcode(raw_str: String, salt: String) -> String {
    hashcode(Sha3::sha3_512(), raw_str, salt)
}

fn hashcode<T>(mut hasher: T, raw_str: String, salt: String) -> String
where
    T: Digest,
{
    hasher.input_str(&format!("{}{}", raw_str, salt));
    hasher.result_str()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_ascii_str() {
        let len = 16;
        let result = create_ascii_str(len);
        assert_eq!(result.len(), len);
        assert_ne!(result, create_ascii_str(len));
        assert_ne!(result, create_ascii_str(len));
    }

    #[test]
    fn test_hash_code() {
        let raw = "raw";
        let salt = "salt";
        assert_ne!(
            sha512_hashcode(String::from(raw), String::from(salt)),
            sha3_512_hashcode(String::from(raw), String::from(salt))
        );
    }
}
