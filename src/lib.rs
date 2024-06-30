extern crate wasm_bindgen;

mod core;
mod macros;
mod runner;

use wasm_bindgen::prelude::wasm_bindgen;
use crate::runner::{BadLockRunner};


#[wasm_bindgen]
pub struct UnlockResult {
    filename: String,
    pub password_count: u8,
    content: Vec<u8>,
}

#[wasm_bindgen]
impl UnlockResult {
    #[wasm_bindgen(getter)]
    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Vec<u8> {
        self.content.clone()
    }
}

#[wasm_bindgen]
pub struct BadLockWasm;

#[wasm_bindgen]
impl BadLockWasm {
    pub fn lock(filename: &str, secret: &str, passwords: Vec<String>, content: Vec<u8>) -> Vec<u8> {
        BadLockRunner::lock(filename, secret, passwords, &content)
    }

    pub fn unlock(password: &str, content: Vec<u8>) -> Result<UnlockResult, String> {
        match BadLockRunner::unlock(&content, password) {
            Ok((meta, bytes)) => Ok(UnlockResult {
                filename: meta.filename,
                password_count: meta.password_count,
                content: bytes,
            }),
            Err(err) => Err(format!("{}", err))
        }
    }
}