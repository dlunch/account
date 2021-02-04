fn main() {
    tonic_build::compile_protos("../proto/auth.proto").unwrap();
}
