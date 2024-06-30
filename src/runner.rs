use std::io::{BufRead, Cursor, Read, Seek, SeekFrom};
use crate::core::BadLockCore;

const ERROR_INVALID_FILE: &'static str = "Invalid BadLock file";
const ERROR_INVALID_HEADER: &'static str = "Invalid Magic Number";
const ERROR_INVALID_COUNT: &'static str = "Invalid Password Count";
const ERROR_LOST_FILENAME: &'static str = "Original Filename Lost";
const ERROR_PASSWORD_COUNT_MISMATCH: &'static str = "Password Count Mismatch";
const ERROR_PASSWORD_LENGTH_PARSE: &'static str = "Password Length Parse Error";
const ERROR_BROKEN: &'static str = "Broken BadLock file";
const ERROR_PASSWORD: &'static str = "Invalid password";

#[derive(Debug)]
pub struct BadLockMeta {
    /// original filename
    pub filename: String,
    /// number of passwords used to lock the file (1-16)
    pub password_count: u8,
}

pub struct BadLockRunner;

impl BadLockRunner {
    pub fn lock(
        filename: impl AsRef<[u8]>,
        secret: impl AsRef<[u8]>,
        passwords: Vec<impl AsRef<[u8]>>,
        content: &[u8],
    ) -> Vec<u8> {
        // no password, no lock!
        if passwords.len() == 0 {
            return vec![];
        }

        let secret = secret.as_ref();

        // container
        let mut buffer = vec![];

        // line 1: magic number with number of passwords
        buffer.extend(format!("BadLock\0{:04b}\n", passwords.len() - 1).into_bytes());

        // line 2: original filename
        buffer.extend(filename.as_ref());
        buffer.push(b'\n');

        // next few lines: encrypt secret with each password
        let mut password_bytes = vec![];
        for password in passwords {
            let sp = BadLockCore::encrypt(secret, password);
            buffer.extend(format!("{}", sp.len()).into_bytes());
            buffer.push(b'\n');
            password_bytes.extend(sp);
        }
        // append all encrypted secrets to the buffer without separator and newline character
        buffer.extend(password_bytes);

        // final line: encrypted content
        buffer.extend(BadLockCore::encrypt(content, secret));

        buffer
    }

    pub fn unlock(bytes: &[u8], password: impl AsRef<[u8]>) -> Result<(BadLockMeta, Vec<u8>), &'static str> {
        let mut csr = Cursor::new(bytes);
        let mut csr_l = (&mut csr).lines();

        let mut meta = BadLockMeta { filename: String::new(), password_count: 0 };

        // parse line 1 to check the magic number and get the number of passwords
        if let Some(Ok(l1)) = csr_l.next() {
            if l1.len() != 12 {
                return Err(ERROR_INVALID_FILE);
            }

            let (magic, password_count) = l1.split_at(8);
            if magic != "BadLock\0" {
                return Err(ERROR_INVALID_HEADER);
            }

            if let Ok(n) = u8::from_str_radix(password_count, 2) {
                meta.password_count = n + 1;
            } else {
                return Err(ERROR_INVALID_COUNT);
            };
        } else {
            return Err(ERROR_INVALID_FILE);
        }

        // parse line 2 to get the original filename
        if let Some(Ok(l2)) = csr_l.next() {
            meta.filename = l2;
        } else {
            return Err(ERROR_LOST_FILENAME);
        }

        // read next few lines to get lengths of encrypted secrets
        let mut encrypted_lengths = vec![];
        for _ in 0..meta.password_count {
            if let Some(Ok(l)) = csr_l.next() {
                if let Ok(p_len) = usize::from_str_radix(&l, 10) {
                    encrypted_lengths.push(p_len);
                } else {
                    return Err(ERROR_PASSWORD_LENGTH_PARSE);
                }
            } else {
                return Err(ERROR_PASSWORD_COUNT_MISMATCH);
            }
        }

        // read bytes according to the lengths
        let mut secret = None::<Vec<u8>>;
        for encrypted_length in encrypted_lengths {
            if secret.is_some() {
                // if the secret is already found, just skip the bytes without decrypting
                csr.seek(SeekFrom::Current(encrypted_length as i64)).unwrap();
            } else {
                // otherwise, try to decrypt the bytes with the password to get the secret
                let mut fragment = vec![0; encrypted_length];
                csr.read_exact(&mut fragment).unwrap();
                if let Some(s) = BadLockCore::decrypt(&fragment, password.as_ref()) {
                    secret = Some(s);
                }
            }
        }

        // finally, decrypt the content with the secret
        if let Some(s) = secret {
            let p = csr.position() as usize;
            BadLockCore::decrypt(&bytes[p..], s)
                .map(|c| (meta, c))
                .ok_or(ERROR_BROKEN)
        } else {
            return Err(ERROR_PASSWORD);
        }
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn lock() {
        let content = b"hello world";
        let passwords = vec!["1", "2", "3", "4"];
        let bytes = super::BadLockRunner::lock("a.txt", "1234", passwords, content);
        println!("{:?}", bytes);
    }

    #[test]
    fn unlock() {
        let raw = vec![66, 97, 100, 76, 111, 99, 107, 0, 48, 48, 49, 49, 10, 97, 46, 116, 120, 116, 10, 49, 54, 10, 49, 54, 10, 49, 54, 10, 49, 54, 10, 8, 183, 56, 21, 225, 185, 248, 95, 232, 245, 107, 31, 59, 71, 51, 52, 199, 136, 140, 23, 62, 83, 130, 240, 93, 37, 51, 39, 28, 86, 11, 67, 59, 110, 221, 18, 99, 239, 216, 55, 23, 70, 79, 117, 4, 95, 196, 21, 248, 128, 250, 93, 241, 65, 149, 157, 247, 177, 24, 49, 24, 67, 87, 131, 175, 0, 34, 166, 71, 158, 233, 95, 57, 205, 154, 224, 203, 163, 195, 152];

        let content = b"hello world";
        let passwords = vec!["1", "2", "3", "4"];

        for password in passwords {
            let result = super::BadLockRunner::unlock(&raw, password);
            println!("Result of {}: {:?}", password, result);
        }
    }
}