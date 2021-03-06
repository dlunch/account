#![recursion_limit = "256"]

extern crate alloc;

mod app;
mod components;
mod context;
mod grpc;
mod router;

use wasm_bindgen::prelude::wasm_bindgen;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn start(grpc_host: &str) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    log::debug!("GRPC_HOST: {}", grpc_host);

    context::Context::init(grpc_host);

    yew::start_app::<app::App>();
}

#[wasm_bindgen(start)]
pub fn main() {}
