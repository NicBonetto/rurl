#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[derive(Clone)]
#[wasm_bindgen]
pub struct Url {
  serialized: String,
  scheme_end: u32,
  user_end: u32,
  host_start: u32,
  host_end: u32,
  port: Option<u16>,
  path_start: u32,
  query_start: Option<u32>,
  fragment_start: Option<u32>
}
