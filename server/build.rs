use std::io;

fn main() -> io::Result<()> {
    tonic_build::compile_protos("../proto/auth.proto")?;
    tonic_build::compile_protos("../proto/card.proto")?;

    Ok(())
}
