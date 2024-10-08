use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_build::compile_protos("src/proto/windexer.proto")?;
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("windexer_descriptor.bin"))
        .compile(&["src/proto/windexer.proto"], &["src/proto"])?;
    Ok(())
}
