extern crate wasm_bindgen;

mod core;
mod r#impl;
mod macros;

use wasm_bindgen::prelude::wasm_bindgen;
use crate::r#impl::{BadLockImpl, UnlockResult};


// OPTIMIZE: make members public rather than using getters to reduce the cost of cloning
#[wasm_bindgen]
pub struct UnlockResultABI(UnlockResult);

#[wasm_bindgen]
impl UnlockResultABI {
    #[wasm_bindgen(getter)]
    pub fn locker(&self) -> String {
        self.0.locker.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn extension(&self) -> String {
        self.0.extension.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Vec<u8> {
        self.0.content.clone()
    }
}

#[wasm_bindgen]
pub struct BadLockWasm;

#[wasm_bindgen]
impl BadLockWasm {
    pub fn lock(bytes: Vec<u8>, password: &str, extension: Option<String>) -> Vec<u8> {
        BadLockImpl::lock(&bytes, password, extension.unwrap_or("".to_string()))
    }

    pub fn unlock(bytes: Vec<u8>, password: &str) -> Result<UnlockResultABI, String> {
        BadLockImpl::unlock(&bytes, password).map(|v| UnlockResultABI(v))
    }
}