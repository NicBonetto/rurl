#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use Url;

#[wasm_bindgen]
pub struct Parser<'a> {
  pub serial: String,
  pub base: Option<&'a Url>
}