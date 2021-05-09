fn main() {
    prost_build::Config::new()
        .compile_protos(&["../proto/internal/internal.proto", "../proto/common.proto"], &["../proto"])
        .unwrap()
}
