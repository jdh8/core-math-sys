use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let out_dir = std::env::var_os("OUT_DIR").context("OUT_DIR not set")?;

    bindgen::Builder::default()
        .header("include/math.h")
        .use_core()
        .allowlist_file("include/*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?
        .write_to_file(std::path::PathBuf::from(out_dir).join("bindings.rs"))?;

    cc::Build::new()
        .files(glob::glob("vendor/src/binary{32,64}/**/*.c")?.flatten())
        .std("c11")
        .try_compile("m")?;

    Ok(())
}
