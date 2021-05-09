fn main() {
    prost_build::Config::new()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["internal/internal.proto", "common.proto"], &["../proto"])
        .unwrap()
}
