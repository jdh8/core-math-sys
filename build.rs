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

    let mut builder = cc::Build::new();
    builder
        .files(
            core::iter::once(std::path::PathBuf::from("lib/signgam.c"))
                .chain(get_source_files(glob::glob("vendor/src/binary32/*/")?, "f.c"))
                .chain(get_source_files(glob::glob("vendor/src/binary64/*/")?, ".c")),
        )
        .include("include");

    // Builtin compiler is too old to handle __builtin_roundeven
    #[cfg(target_os = "macos")]
    builder.compiler("clang");

    builder.try_compile("m")?;
    Ok(())
}
