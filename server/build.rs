use std::io;

fn main() -> io::Result<()> {
    tonic_build::configure().compile(&["common.proto", "auth.proto", "card.proto"], &["../proto"])?;

    prost_build::Config::new()
        .compile_protos(&["internal/internal.proto"], &["../proto"])
        .unwrap();

    Ok(())
}
