//! Build script for the Windows Rust Driver crate.
//!
//! Based on the [`wdk_build::Config`] parsed from the build tree, this build
//! script will provide `Cargo` with the necessary information to build the
//! driver binary (ex. linker flags)

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
        .nth(4)
        .expect("OUT_DIR must be inside target/<profile>/build/<package>/out");
    let crate_name = env!("CARGO_PKG_NAME").replace('-', "_");
    let map_path = profile_dir.join("deps").join(format!("{crate_name}.map"));
    println!("cargo::rustc-cdylib-link-arg=/MAP:{}", map_path.display());
}
