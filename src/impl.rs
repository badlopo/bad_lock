//! io: provides I/O operations for BadLock

use std::fs;
use std::io::Write;
use std::path::Path;
use crate::core::BadLockCore;
use crate::timestamp;

const HEADER: &[u8] = b"BadLock(v0.0.1)\n";

pub struct BadLockImpl;

impl BadLockImpl {
    fn lock(path: impl AsRef<Path>, password: impl AsRef<[u8]>) -> Result<(), String> {
        match fs::read(&path) {
            Ok(bytes) => {
                let path: &Path = path.as_ref();
                let filename = match path.file_stem() {
                    None => format!("{}.badlock", timestamp!()),
                    Some(stem) => format!("{}.badlock", stem.to_string_lossy())
                };

                match fs::File::create(filename) {
                    Ok(mut file) => {
                        let encrypted = BadLockCore::encrypt(&bytes, password);
                        file.write(HEADER).expect("Error: Failed to write the header.");
                        file.write_all(&encrypted).expect("Error: Failed to write the encrypted data.");
                        Ok(())
                    }
                    Err(err) => Err(format!("{}", err))
                }
            }
            Err(err) => Err(format!("{}", err))
        }
    }

    fn unlock(path: impl AsRef<Path>, password: impl AsRef<[u8]>) -> Result<(), String> {
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
        super::BadLockImpl::lock("./test.txt", "password").unwrap();
        // println!("{:?}", timestamp!());
    }
}