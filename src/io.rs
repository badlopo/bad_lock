use std::fs;
use std::path::{Path, PathBuf};
use crate::runner::BadLockRunner;
use crate::timestamp;

pub struct BadLockIO;

impl BadLockIO {
    pub fn lock(
        input: impl AsRef<Path>,
        secret: Option<impl AsRef<[u8]>>,
        passwords: Vec<impl AsRef<[u8]>>,
        output: Option<impl AsRef<Path>>,
    ) -> Result<PathBuf, String> {
        let input: &Path = input.as_ref();

        match fs::read(input) {
            Ok(bytes) => {
                let filename: &[u8] = input.file_name().unwrap().to_str().unwrap().as_bytes();
                let bytes = if let Some(s) = secret {
                    BadLockRunner::lock(filename, s, passwords, &bytes)
                } else {
                    BadLockRunner::lock(filename, format!("{}", timestamp!()), passwords, &bytes)
                };

                let output = match output {
                    Some(o) => o.as_ref().to_path_buf(),
                    None => {
                        let mut o = input.to_path_buf();
                        o.set_extension("badlock");
                        o
                    }
                };

                match fs::write(&output, bytes) {
                    Ok(_) => Ok(output.canonicalize().unwrap()),
                    Err(err) => Err(format!("Write error: {}", err)),
                }
            }
            Err(err) => Err(format!("Read error: {}", err)),
        }
    }

    pub fn unlock(
        input: impl AsRef<Path>,
        password: impl AsRef<[u8]>,
        output: Option<impl AsRef<Path>>,
    ) -> Result<PathBuf, String> {
        let input: &Path = input.as_ref();

        match fs::read(input) {
            Ok(bytes) => {
                let (meta, bytes) = BadLockRunner::unlock(&bytes, password)?;

                let output = match output {
                    Some(o) => o.as_ref().to_path_buf(),
                    None => {
                        let mut o = input.to_path_buf();
                        o.set_file_name(if meta.filename.is_empty() { "UNKNOWN_FILE" } else { &meta.filename });
                        o
                    }
                };

                match fs::write(&output, bytes) {
                    Ok(_) => Ok(output.canonicalize().unwrap()),
                    Err(err) => Err(format!("Write error: {}", err)),
                }
            }
            Err(err) => Err(format!("Read error: {}", err)),
        }
    }
}