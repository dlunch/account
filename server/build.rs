use std::io;

fn main() -> io::Result<()> {
    tonic_build::configure().compile(&["../proto/common.proto", "../proto/auth.proto", "../proto/card.proto"], &["../proto"])?;

    prost_build::Config::new()
        .compile_protos(&["../proto/internal/internal.proto"], &["../proto"])
        .unwrap();

    Ok(())
}
