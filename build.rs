fn sources(
    parent: &str,
    suffix: &str,
) -> std::io::Result<impl Iterator<Item = std::path::PathBuf>> {
    Ok(std::fs::read_dir(parent)?
        .flatten()
        .filter_map(move |entry| {
            let dir = entry.path();
            let file = dir.join(format!("{}{suffix}", dir.file_name()?.to_str()?));
            file.is_file().then_some(file)
        }))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // rustdoc never links; docs.rs's C compiler may not support f16/f128
    if std::env::var_os("DOCS_RS").is_some() {
        return Ok(());
    }

    let feature = |name: &str| std::env::var_os(format!("CARGO_FEATURE_{name}")).is_some();

    let mut files: Vec<_> = sources("vendor/src/binary32", "f.c")?
        .chain(sources("vendor/src/binary64", ".c")?)
        .collect();
    if feature("F16") {
        files.extend(sources("vendor/src/binary16", "f16.c")?);
    }
    if feature("F128") {
        files.extend(sources("vendor/src/binary128", "q.c")?);
    }

    let mut builder = cc::Build::new();
    builder
        .files(files)
        .flag_if_supported({
            let mut flag: std::ffi::OsString = "-march=".into();
            flag.push(std::env::var_os("TARGET_CPU").unwrap_or_else(|| "native".into()));
            flag
        })
        .cargo_warnings(false);

    // glibc declares the *f128 functions only for GCC, but the binary128
    // sources call them in alias wrappers, which Clang 16+ rejects as
    // implicit declarations
    if feature("F128") && builder.get_compiler().is_like_clang() {
        builder.flag("-include").flag("lib/f128-decls.h");
        println!("cargo:rerun-if-changed=lib/f128-decls.h");
    }

    builder.try_compile("core-math")?;
    Ok(())
}
