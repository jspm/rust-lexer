//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]
#![cfg(feature = "wasm")]

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_wasm() {
    let fixture = include_str!("../fixtures/angular.js");
    assert_eq!(1, 1)
}
