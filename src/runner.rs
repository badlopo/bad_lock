use crate::core::BadLockCore;

pub struct UnlockResult {}

pub struct BadLockRunner;

impl BadLockRunner {
    pub fn lock(
        filename: impl AsRef<[u8]>,
        secret: impl AsRef<[u8]>,
        passwords: Vec<impl AsRef<[u8]>>,
        content: &[u8],
    ) -> Vec<u8> {
        let secret = secret.as_ref();

        // container
        let mut buffer = vec![];

        // line 1: magic number with number of passwords
        buffer.extend(format!("BadLock\0{:04b}\n", passwords.len()).into_bytes());

        // line 2: original filename
        buffer.extend(filename.as_ref());

        // next few lines: encrypt secret with each password
        for password in passwords {
            let sp = BadLockCore::encrypt(secret, password);
            buffer.extend(sp);
            buffer.push(b'\n');
        }

        // final line: encrypted content
        buffer.extend(BadLockCore::encrypt(content, secret));

        buffer
    }

    pub fn unlock(bytes: &[u8], password: impl AsRef<[u8]>) -> Result<UnlockResult, String> {
        todo!()
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn t() {
        let bytes = super::BadLockRunner::lock("a.txt", "1234", vec!["1", "2", "3", "4"], b"hello world");
        println!("{:?}", bytes);
    }
}