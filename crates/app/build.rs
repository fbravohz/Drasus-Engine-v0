use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto_root = manifest_dir.join("../../proto");
    let schema = proto_root.join("drasus/desktop/v1/bootstrap.proto");
    let protoc = protoc_bin_vendored::protoc_bin_path()?;

    println!("cargo:rerun-if-changed={}", schema.display());
    // SAFETY: el build script no crea hilos antes de configurar `PROTOC` para la generación.
    unsafe { std::env::set_var("PROTOC", protoc) };

    tonic_prost_build::configure().compile_protos(&[schema], &[proto_root])?;
    Ok(())
}
