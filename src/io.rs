use std::fs;
use std::path::Path;
use crate::r#impl::BadLockImpl;

pub struct BadLockIO;

impl BadLockIO {
    /// - `input` path to the file to be locked
    /// - `password` password to lock the file
    /// - `output` path to the output file, if not provided,
    /// the output file will be named as `input` with extension `.badlock`
    pub fn lock<I, P, O>(input: I, password: P, output: Option<O>) -> Result<(), String>
    where
        I: AsRef<Path>,
        P: AsRef<[u8]>,
        O: AsRef<Path>,
    {
        let input: &Path = input.as_ref();

        match fs::read(input) {
            Ok(bytes) => {
                let ext: &[u8] = input.extension().map_or(b"", |ext| ext.as_encoded_bytes());
                let bytes = BadLockImpl::lock(&bytes, password, ext);

                let output = match output {
                    Some(o) => o.as_ref().to_path_buf(),
                    None => {
                        let mut o = input.to_path_buf();
                        o.set_extension("badlock");
                        o
                    }
                };

                match fs::write(output, bytes) {
                    Ok(_) => Ok(()),
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
    pub fn unlock<I, P, O>(input: I, password: P, output: Option<O>) -> Result<(), String>
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

                match fs::write(output, result.content) {
                    Ok(_) => Ok(()),
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