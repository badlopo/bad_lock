use std::fs;
use std::path::{Path, PathBuf};
use crate::r#impl::BadLockImpl;
use crate::runner::BadLockRunner;
use crate::timestamp;

pub struct BadLockIO;

impl BadLockIO {
    /// - `input` path to the file to be locked
    /// - `password` password to lock the file
    /// - `output` path to the output file, if not provided,
    /// the output file will be named as `input` with extension `.badlock`
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

    /// - `input` path to the file to be unlocked
    /// - `password` password to unlock the file
    /// - `output` path to the output file, if not provided,
    /// the output file will be named as `input` with extension in the locked file (if any)
    pub fn unlock<I, P, O>(input: I, password: P, output: Option<O>) -> Result<PathBuf, String>
    where
        I: AsRef<Path>,
        P: AsRef<[u8]>,
        O: AsRef<Path>,
    {
        let input: &Path = input.as_ref();

        match fs::read(input) {
            Ok(bytes) => {
                let result = BadLockImpl::unlock(&bytes, password)?;

                let output = match output {
                    Some(o) => o.as_ref().to_path_buf(),
                    None => {
                        let mut o = input.to_path_buf();
                        o.set_extension(&result.extension);
                        o
                    }
                };

                match fs::write(&output, result.content) {
                    Ok(_) => Ok(output.canonicalize().unwrap()),
                    Err(err) => Err(format!("Write error: {}", err)),
                }
            }
            Err(err) => Err(format!("Read error: {}", err)),
        }
    }
}

#[cfg(test)]
mod unit_test {
    use std::path::Path;

    #[test]
    fn lock() {
        let r = super::BadLockIO::lock("__test__/test.txt", "password", None::<&Path>);
        println!("{:?}", r)
    }

    #[test]
    fn unlock() {
        let r = super::BadLockIO::unlock("__test__/test.badlock", "password", Some("__test__/test_recover.txt"));
        println!("{:?}", r)
    }
}