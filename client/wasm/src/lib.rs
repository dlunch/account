extern crate alloc;

mod app;

use wasm_bindgen::prelude::wasm_bindgen;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "proto/AuthServiceClientPb")]
extern "C" {
    type AuthClient;

    #[wasm_bindgen(constructor)]
    fn new(hostname: &str) -> AuthClient;
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    let _ = AuthClient::new("test");

    yew::start_app::<app::App>();
}
