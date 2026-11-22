use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let workspace_root = manifest_dir.join("../../..");
    let proto_root = workspace_root.join("proto");
    let schema = proto_root.join("drasus/desktop/v1/bootstrap.proto");
    let descriptor = PathBuf::from(env::var("OUT_DIR")?).join("bootstrap_descriptor.bin");
    let protoc = protoc_bin_vendored::protoc_bin_path()?;

    println!("cargo:rerun-if-changed={}", schema.display());
    // La misma versión del esquema genera cliente y servidor en ambos procesos.
    unsafe { env::set_var("PROTOC", protoc) };
    tonic_prost_build::configure()
        .file_descriptor_set_path(descriptor)
        .compile_protos(&[schema], &[proto_root])?;

    let profile = env::var("PROFILE")?;
    let core_name = format!("drasus{}", env::consts::EXE_SUFFIX);
    let core_binary = workspace_root.join("target").join(profile).join(core_name);
    println!(
        "cargo:rustc-env=DRASUS_CORE_BINARY={}",
        core_binary.display()
    );

    tauri_build::build();
    Ok(())
}
