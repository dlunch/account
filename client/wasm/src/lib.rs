extern crate alloc;

mod app;

use typescript_wasm_bindgen::typescript_wasm_bindgen;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

typescript_wasm_bindgen!("client/src/proto/AuthServiceClientPb.ts", "proto/AuthServiceClientPb");

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    let _ = AuthClient::new("test", JsValue::NULL, JsValue::NULL);

    yew::start_app::<app::App>();
}
