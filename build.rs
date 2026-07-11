fn get_source_files<'a>(
    dirs: impl Iterator<Item = std::path::PathBuf> + 'a,
    suffix: &'a str,
) -> impl Iterator<Item = std::path::PathBuf> + 'a {
    dirs.filter_map(move |mut path| {
        let stem = path.file_stem()?;
        let mut name = stem.to_owned();
        name.push(suffix);
        path.push(name);
        path.is_file().then_some(path)
    })
}

fn subdirs(parent: &str) -> std::io::Result<impl Iterator<Item = std::path::PathBuf>> {
    Ok(std::fs::read_dir(parent)?
        .flatten()
        .map(|entry| entry.path()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var_os("OUT_DIR").ok_or("OUT_DIR not set")?;

    bindgen::Builder::default()
        .header("include/core-math.h")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?
        .write_to_file(std::path::PathBuf::from(out_dir).join("bindings.rs"))?;

    // rustdoc never links; docs.rs's C compiler may not support f16/f128
    if std::env::var_os("DOCS_RS").is_some() {
        return Ok(());
    }

    let feature = |name: &str| std::env::var_os(format!("CARGO_FEATURE_{name}")).is_some();

    let mut sources = vec![std::path::PathBuf::from("lib/signgam.c")];
    sources.extend(get_source_files(subdirs("vendor/src/binary32")?, "f.c"));
    sources.extend(get_source_files(subdirs("vendor/src/binary64")?, ".c"));
    if feature("F16") {
        sources.extend(get_source_files(subdirs("vendor/src/binary16")?, "f16.c"));
    }
    if feature("F128") {
        sources.extend(get_source_files(subdirs("vendor/src/binary128")?, "q.c"));
    }

    let mut builder = cc::Build::new();
    builder
        .files(sources)
        .flag_if_supported({
            let mut flag: std::ffi::OsString = "-march=".into();
            flag.push(std::env::var_os("TARGET_CPU").unwrap_or_else(|| "native".into()));
            flag
        })
        .cargo_warnings(false);

    // Builtin compiler is too old to handle __builtin_roundeven
    #[cfg(target_os = "macos")]
    builder.compiler("clang");

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
