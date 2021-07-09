extern crate wasm_bindgen;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Attr, 
    // CanvasRenderingContext2d,
    // Document, 
    // Element,
    // HtmlCanvasElement,
    // HtmlFontElement,
    // HtmlVideoElement, 
    // HtmlMediaElement, 
    // Performance,
    // Navigator, 
    // MediaDevices, 
    // MediaStream, 
    // MediaStreamConstraints, 
    // Node, 
    // Window
};
use js_sys::Promise;
// use wasm_bindgen_futures::JsFuture;


/// Pulls console.log function from javascript
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// This will run when page loads
#[wasm_bindgen]
pub async fn run() {
    log("Hi from Rust");
}
