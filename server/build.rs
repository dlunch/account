use std::io;

fn main() -> io::Result<()> {
    tonic_build::compile_protos("../proto/auth.proto")?;
    tonic_build::compile_protos("../proto/card.proto")?;

    prost_build::Config::new()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["../proto/internal/internal.proto"], &["../proto"])
        .unwrap();

    Ok(())
}
