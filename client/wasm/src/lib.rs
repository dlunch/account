extern crate alloc;

mod app;

use typescript_wasm_bindgen::typescript_wasm_bindgen;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

typescript_wasm_bindgen!("client/src/proto/AuthServiceClientPb.ts", "proto/AuthServiceClientPb");
typescript_wasm_bindgen!("client/src/proto/auth_pb.d.ts", "proto/auth_pb");

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    let auth_client = AuthClient::new("http://localhost:8000", JsValue::NULL, JsValue::NULL);
    let login_request = LoginRequest::new();
    auth_client.login(JsValue::from(login_request), JsValue::NULL, JsValue::UNDEFINED);

    yew::start_app::<app::App>();
}
