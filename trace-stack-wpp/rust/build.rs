fn main() -> Result<(), wdk_build::ConfigError> {
    let target = std::env::var("TARGET").expect("Cargo must set TARGET");
    assert_eq!(
        target, "x86_64-pc-windows-msvc",
        "The WPP assembly sample supports Windows x64 MSVC only"
    );
    let config = wdk_build::Config::from_env_auto()?;
    config.configure_library_build()?;
    for path in config.library_paths()? {
        println!("cargo::rustc-link-search=native={}", path.display());
    }
    println!("cargo::rustc-link-lib=static=ntoskrnl");
    println!("cargo::rustc-link-lib=static=wpprecorder");
    // Keep native initialization reachable so LTO cannot fold both sink gates off.
    println!("cargo::rustc-link-arg=/EXPORT:trace_entry");
    Ok(())
}
