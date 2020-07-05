use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;

use crate::pswdmng::{create_ascii_str,Error};


pub(crate) fn encrypt(key: Vec<u8>, plaintext: String, nonce: Vec<u8>) -> Result<Vec<u8>, Error> {
    let cipher = aes_256_gcm(key);
    match cipher.encrypt(
        GenericArray::from_slice(nonce.as_ref()),
        plaintext.as_bytes(),
    ) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::EncryptError(e)),
    }
}

pub(crate) fn decrypt(key: Vec<u8>, cryptogram: Vec<u8>, nonce: Vec<u8>) -> Result<String, Error> {
    let cipher = aes_256_gcm(key);
    match cipher.decrypt(
        GenericArray::from_slice(nonce.as_ref()),
        cryptogram.as_ref(),
    ) {
        Ok(v) => match String::from_utf8(v) {
            Ok(s) => Ok(s),
            Err(_) => Ok(String::default()),
        },
        Err(e) => Err(Error::EncryptError(e)),
    }
}

pub(crate) fn create_nonce()-> Vec<u8>{
    create_ascii_str(12).as_bytes().to_vec()
}

fn aes_256_gcm(key: Vec<u8>) -> Aes256Gcm {
    Aes256Gcm::new(GenericArray::from_slice(key.as_ref()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_module() {
        let nonce = create_nonce();
        let key = String::from("12345678901234567890123456789012");
        let cryptogram = encrypt(
            key.as_bytes().to_vec(),
            String::from("plaintext"),
            nonce.clone(),
        )
        .unwrap();
        assert_eq!(
            decrypt(
                key.as_bytes().to_vec(),
                cryptogram,
                nonce.clone(),
            ).unwrap(),
            String::from("plaintext")
        );
    }
}
