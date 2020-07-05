use sha3::{Sha3_512, Digest};

pub(crate) fn sha3_512_hashcode(raw_str: &String, salt: &String) -> Vec<u8>{
    hashcode(Sha3_512::new(), raw_str, salt)
}

fn hashcode<T>(mut hasher: T, raw_str: &String, salt: &String) -> Vec<u8>
where
    T: Digest,
{
    hasher.update(&format!("{}{}", raw_str, salt));
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_code() {
        let raw = "raw";
        let salt = "salt";
        assert_eq!(
            sha3_512_hashcode(&String::from(raw), &String::from(salt)),
            sha3_512_hashcode(&String::from(raw), &String::from(salt))
        );
    }
}
