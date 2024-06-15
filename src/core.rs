//! core: provides the ability to use AES encryption and decryption

// since 'aes::cipher' and 'cbc::cipher' are both re-exported,
// they are actually the same thing, so it doesn't matter where they are imported from.
use aes::cipher::{BlockEncryptMut, KeyIvInit, block_padding::{Pkcs7}};
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
    pub fn encrypt(source: &[u8], code: &[u8]) -> Vec<u8> {
        // use md5 hash to ensure the length is 16
        let code16 = get_md5!(code);

        Aes128CbcEnc::new(&code16.into(), &code16.into())
            .encrypt_padded_vec_mut::<Pkcs7>(source)
    }

    pub fn decrypt(source: &[u8], code: &[u8]) -> Vec<u8> {
        todo!()
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
    }
}