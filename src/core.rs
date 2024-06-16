//! core: provides the ability to use AES encryption and decryption

// since 'aes::cipher' and 'cbc::cipher' are both re-exported,
// they are actually the same thing, so it doesn't matter where they are imported from.
use aes::cipher::{
    BlockEncryptMut,
    BlockDecryptMut,
    KeyIvInit,
    block_padding::{Pkcs7},
};
use crate::get_md5;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128Enc>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128Dec>;

/// AES128, CBC mode, PKCS7 padding
/// - key & iv: md5(code)
pub struct ImageLockCore;

impl ImageLockCore {
    /// encrypt source with code
    ///
    /// - len(result) = len(source) + 16 - len(source) % 16
    pub fn encrypt(source: &[u8], password: &[u8]) -> Vec<u8> {
        // use md5 hash to ensure the length is 16
        let code = get_md5!(password);

        Aes128CbcEnc::new(&code.into(), &code.into())
            .encrypt_padded_vec_mut::<Pkcs7>(source)
    }

    /// decrypt source with code
    pub fn decrypt(source: &[u8], password: &[u8]) -> Vec<u8> {
        // use md5 hash to ensure the length is 16
        let code = get_md5!(password);

        Aes128CbcDec::new(&code.into(), &code.into())
            .decrypt_padded_vec_mut::<Pkcs7>(source)
            .expect("Unpad failed")
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn encrypt() {
        let source = "hello world".as_bytes();
        let code = "1234567890123456".as_bytes();
        let encrypted = super::ImageLockCore::encrypt(source, code);

        println!("{:02x?}", encrypted);
        // [0e, e6, a2, 36, 0d, 77, d8, 46, 18, 15, a3, bf, 07, 12, 74, b2]
    }

    #[test]
    fn decrypt() {
        let source = "hello world".as_bytes();
        let code = "1234567890123456".as_bytes();
        let encrypted = super::ImageLockCore::encrypt(source, code);
        let decrypted = super::ImageLockCore::decrypt(&encrypted, code);

        println!("{:02x?}", decrypted)
        // [68, 65, 6c, 6c, 6f, 20, 77, 6f, 72, 6c, 64]
    }
}