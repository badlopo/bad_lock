//! io: provides I/O operations for BadLock

use std::fs;
use std::io::Write;
use std::path::Path;
use crate::core::BadLockCore;
use crate::timestamp;

const HEADER: &[u8] = b"BadLock(v0.0.1)\n";

/// Builds the header for the BadLock file.
///
/// The header is fixed length of 32 bytes with the following format:
///
/// ```
/// BadLock(v0.0.1)\n -- (16 bytes)
/// <extension> -- (padding with spaces to 16 bytes)
/// ```
fn build_header(extension: impl AsRef<[u8]>) -> Vec<u8> {
    // of course, we can define a fixed length array (`[u8; 32]`),
    // but a `Vec` is more flexible for later use.
    let mut header = vec![32u8; 32];
    let extension: &[u8] = extension.as_ref();

    header[..HEADER.len()].copy_from_slice(HEADER);
    header[HEADER.len()..HEADER.len() + extension.len()].copy_from_slice(extension);

    header
}

pub struct BadLockImpl;

impl BadLockImpl {
    /// - `bytes` raw data to be locked
    /// - `password` password to lock the data
    /// - `extension` extension of original file
    fn lock(bytes: &[u8], password: impl AsRef<[u8]>, extension: impl AsRef<[u8]>) -> Vec<u8> {
        let mut header = build_header(extension);
        header.extend(BadLockCore::encrypt(bytes, password));
        header
    }

    fn unlock(bytes: &[u8], password: impl AsRef<[u8]>) -> Result<(), String> {
        todo!()
        // match fs::read(path) {
        //     Ok(bytes) => {
        //         if bytes.starts_with(HEADER) {
        //             return Err("Error: The file is already locked.".to_string());
        //         }
        //
        //         Ok(())
        //     }
        //     Err(err) => Err(format!("{}", err))
        // }
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
    fn t() {
        let mut v = vec![3, 4, 5];
        v.splice(0..0, vec![1, 2, 3]);
        v.extend_from_slice(&[6, 7, 8]);
        println!("{:?}", v)
    }
}