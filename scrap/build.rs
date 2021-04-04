fn main() {
    prost_build::Config::new()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["../proto/internal/internal.proto"], &["../proto"])
        .unwrap()
}
