use std::error::Error;
use std::{env, path::PathBuf};
use tonic_prost_build::configure;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("indicator_descriptor.bin"))
        .compile_protos(&["proto/indicators.proto"], &["proto"])?;

    Ok(())
}
