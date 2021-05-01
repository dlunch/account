use std::io;

fn main() -> io::Result<()> {
    tonic_build::configure().compile(&["../proto/common.proto", "../proto/auth.proto", "../proto/card.proto"], &["../proto"])?;

    prost_build::Config::new()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["../proto/internal/internal.proto"], &["../proto"])
        .unwrap();

    Ok(())
}
