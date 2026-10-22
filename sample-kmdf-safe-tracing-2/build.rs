//! Build script for the Windows Rust Driver crate.

fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdk_binary_build()?;
    configure_map_output();
    Ok(())
}

fn configure_map_output() {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR").expect("Cargo must provide OUT_DIR to build scripts"),
    );
    let profile_dir = out_dir
        .ancestors()
        .find(|dir| dir.file_name().is_some_and(|name| name == "build"))
        .and_then(std::path::Path::parent)
        .expect("OUT_DIR must be inside target/<profile>/build/");
    let crate_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let map_path = profile_dir.join("deps").join(format!("{crate_name}.map"));
    println!("cargo::rustc-cdylib-link-arg=/MAP:{}", map_path.display());
}
