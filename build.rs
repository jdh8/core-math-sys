use anyhow::Context as _;

fn get_source_files<'a>(
    dirs: glob::Paths,
    suffix: impl AsRef<std::ffi::OsStr> + 'a,
) -> impl Iterator<Item = std::path::PathBuf> + 'a {
    dirs.flatten().filter_map(move |mut path| {
        let stem = path.file_stem()?;
        let mut name = stem.to_owned();
        name.push(suffix.as_ref());
        path.push(name);
        path.is_file().then_some(path)
    })
}

fn main() -> anyhow::Result<()> {
    let out_dir = std::env::var_os("OUT_DIR").context("OUT_DIR not set")?;

    bindgen::Builder::default()
        .header("include/math.h")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?
        .write_to_file(std::path::PathBuf::from(out_dir).join("bindings.rs"))?;

    let f32_sources = get_source_files(glob::glob("vendor/src/binary32/*/")?, "f.c");
    let f64_sources = get_source_files(glob::glob("vendor/src/binary64/*/")?, ".c");

    cc::Build::new()
        .files(f32_sources.chain(f64_sources))
        .include("include")
        .try_compile("m")?;

    Ok(())
}
