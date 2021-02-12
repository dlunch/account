use typescript_wasm_bindgen::typescript_wasm_bindgen;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

typescript_wasm_bindgen!("client/src/proto/AuthServiceClientPb.ts", "proto/AuthServiceClientPb");
typescript_wasm_bindgen!("client/src/proto/auth_pb.d.ts", "proto/auth_pb");
