use crate::core::BadLockCore;

const AGENT: &[u8] = b"BadLock(v0.0.1)\n";

/// Builds the header for the BadLock file.
///
/// The header is fixed length of 32 bytes with the following format:
///
/// ```
/// BadLock(v0.0.1)\n -- (16 bytes)
/// <extension> -- (padding with spaces to 16 bytes)
/// ```
fn build_header<EXT>(extension: EXT) -> Vec<u8>
where
    EXT: AsRef<[u8]>,
{
    // of course, we can define a fixed length array (`[u8; 32]`),
    // but a `Vec` is more flexible for later use.
    let mut header = vec![32u8; 32];
    let extension: &[u8] = extension.as_ref();

    header[..AGENT.len()].copy_from_slice(AGENT);
    header[AGENT.len()..AGENT.len() + extension.len()].copy_from_slice(extension);

    header
}

/// Parses the header of the BadLock file.
///
/// The header is fixed length of 32 bytes with the following format:
///
/// ```
/// BadLock(v0.0.1)\n -- (16 bytes)
/// <extension> -- (padding with spaces to 16 bytes)
/// ```
fn parse_header(header: &[u8]) -> Option<UnlockResult> {
    let (executor, extension) = header.split_at(16);
    if executor.starts_with(b"BadLock") {
        Some(UnlockResult {
            locker: String::from_utf8_lossy(executor).trim().to_string(),
            extension: String::from_utf8_lossy(extension).trim().to_string(),
            content: vec![],
        })
    } else {
        None
    }
}

#[derive(Debug)]
pub struct UnlockResult {
    pub locker: String,
    pub extension: String,
    pub content: Vec<u8>,
}

pub struct BadLockImpl;

impl BadLockImpl {
    /// - `bytes` raw data to be locked
    /// - `password` password to lock the data
    /// - `extension` extension of original file
    pub fn lock<P, EXT>(bytes: &[u8], password: P, extension: EXT) -> Vec<u8>
    where
        P: AsRef<[u8]>,
        EXT: AsRef<[u8]>,
    {
        let mut header = build_header(extension);
        header.extend(BadLockCore::encrypt(bytes, password));
        header
    }

    /// - `bytes` raw data to be unlocked
    /// - `password` password to unlock the data
    pub fn unlock<P>(bytes: &[u8], password: P) -> Result<UnlockResult, String>
    where
        P: AsRef<[u8]>,
    {
        if bytes.len() < 32 {
            return Err("Invalid BadLock file".to_string());
        }

        let (header, encrypted) = bytes.split_at(32);

        match parse_header(header) {
            Some(mut result) => match BadLockCore::decrypt(encrypted, password) {
                Some(decrypted) => {
                    result.content = decrypted;
                    Ok(result)
                }
                None => Err("Invalid password".to_string()),
            }
            None => Err("Invalid BadLock file".to_string()),
        }
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn lock() {
        let encrypted = super::BadLockImpl::lock(b"hello world\nthis is line2", "password", "txt");
        println!("{:?}", encrypted);
    }

    #[test]
    fn unlock() {
        let bytes = [66, 97, 100, 76, 111, 99, 107, 40, 118, 48, 46, 48, 46, 49, 41, 10, 116, 120, 116, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 223, 56, 252, 85, 169, 5, 1, 161, 64, 150, 20, 114, 64, 51, 1, 179, 216, 157, 245, 248, 114, 58, 41, 110, 125, 77, 219, 126, 201, 25, 113, 96];
        let decrypted = super::BadLockImpl::unlock(&bytes, "password");
        println!("{:?}", decrypted);
    }
}