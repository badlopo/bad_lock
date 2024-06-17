use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use crate::r#impl::BadLockImpl;

pub struct BadLockIO;

impl BadLockIO {
    /// - `input` path to the file to be locked
    /// - `password` password to lock the file
    /// - `output` path to the output file
    pub fn lock(
        input: impl AsRef<Path>,
        password: impl AsRef<[u8]>,
        output: Option<impl AsRef<Path>>,
    ) -> Result<(), String> {
        let input: &Path = input.as_ref();

        match fs::read(input) {
            Ok(bytes) => {
                let ext: &[u8] = input.extension().map_or(b"", |ext| ext.as_encoded_bytes());
                let bytes = BadLockImpl::lock(&bytes, password, ext);

                let output = match output {
                    None => {
                        let mut o = input.to_path_buf();
                        o.set_extension("badlock");
                        o
                    }
                    Some(o) => o.as_ref().to_path_buf(),
                };

                match fs::write(output, bytes) {
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
    #[test]
    fn lock() {
        let r = super::BadLockIO::lock("test.txt", "password", Some("test.badlock"));
        println!("{:?}", r)
    }
}