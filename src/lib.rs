use config::Manifest;
use logger::Logger;
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

mod config;
mod logger;
pub mod rustc;
mod rustdoc;

use rustc::{CrateType, Rustc};
use rustdoc::RustDoc;

pub type Result<T> = std::result::Result<T, BoxError>;
pub type BoxError = Box<dyn Error>;

fn lib_compile(
    logger: &mut Logger,
    manifest: &Manifest,
    lib_path: &Path,
    out_dir: &Path,
) -> Result<()> {
    logger.compiling_crate(&manifest.crate_name)?;
    Rustc::builder()
        .edition(manifest.edition)
        .crate_type(CrateType::Lib)
        .crate_name(&manifest.crate_name)
        .out_dir(out_dir)
        .lib_dir(out_dir)
        .done()
        .run(lib_path.to_str().unwrap())?;
    Ok(())
}

fn bin_compile(
    logger: &mut Logger,
    manifest: &Manifest,
    bin_path: &Path,
    out_dir: &Path,
    externs: &[&str],
) -> Result<()> {
    logger.compiling_bin(&manifest.crate_name)?;
    let mut builder = Rustc::builder()
        .edition(manifest.edition)
        .crate_type(CrateType::Bin)
        .crate_name(&manifest.crate_name)
        .out_dir(out_dir)
        .lib_dir(out_dir);

    for ex in externs {
        builder = builder.externs(*ex);
    }

    builder.done().run(bin_path.to_str().unwrap())?;
    Ok(())
}

pub fn build() -> Result<()> {
    let mut logger = Logger::new();
    let root_dir = root_dir()?;
    let manifest = Manifest::parse_from_file(root_dir.join("Freight.toml"))?;
    let lib_rs = root_dir.join("src").join("lib.rs");
    let main_rs = root_dir.join("src").join("main.rs");
    let target = root_dir.join("target");
    let target_debug = target.join("debug");
    fs::create_dir_all(&target_debug)?;

    match (lib_rs.exists(), main_rs.exists()) {
        (true, true) => {
            lib_compile(&mut logger, &manifest, &lib_rs, &target_debug)?;
            bin_compile(
                &mut logger,
                &manifest,
                &main_rs,
                &target_debug,
                &[&manifest.crate_name],
            )?;
        }
        (true, false) => lib_compile(&mut logger, &manifest, &lib_rs, &target_debug)?,
        (false, true) => bin_compile(
            &mut logger,
            &manifest,
            &main_rs,
            &target_debug,
            &[&manifest.crate_name],
        )?,
        (false, false) => return Err("There is nothing to compile".into()),
    };
    Ok(())
}

fn test_compile(
    manifest: &Manifest,
    bin_path: &Path,
    out_dir: &Path,
    externs: &[&str],
) -> Result<()> {
    let mut builder = Rustc::builder()
        .edition(manifest.edition)
        .crate_type(CrateType::Bin)
        .crate_name(format!(
            "test_{}_{}",
            &manifest.crate_name,
            bin_path.file_stem().unwrap().to_str().unwrap()
        ))
        .out_dir(out_dir)
        .lib_dir(out_dir)
        .test(true);

    for ex in externs {
        builder = builder.externs(*ex);
    }
    builder.done().run(bin_path.to_str().unwrap())?;
    Ok(())
}

pub fn build_tests() -> Result<()> {
    let mut logger = Logger::new();
    let root_dir = root_dir()?;
    let manifest = Manifest::parse_from_file(root_dir.join("Freight.toml"))?;

    let lib_rs = root_dir.join("src").join("lib.rs");
    let main_rs = root_dir.join("src").join("main.rs");
    let target = root_dir.join("target");
    let target_tests = target.join("debug").join("tests");
    fs::create_dir_all(&target_tests)?;

    match (lib_rs.exists(), main_rs.exists()) {
        (true, true) => {
            test_compile(&manifest, &lib_rs, &target_tests, &[])?;
            lib_compile(&mut logger, &manifest, &lib_rs, &target_tests)?;
            test_compile(&manifest, &main_rs, &target_tests, &[&manifest.crate_name])?;
        }
        (true, false) => test_compile(&manifest, &lib_rs, &target_tests, &[])?,
        (false, true) => test_compile(&manifest, &main_rs, &target_tests, &[])?,
        (false, false) => return Err("There is nothing to compile".into()),
    };
    Ok(())
}

pub fn run_tests(test_args: Vec<String>) -> Result<()> {
    let mut logger = Logger::new();
    let root = root_dir()?;
    let manifest = Manifest::parse_from_file(root.join("Freight.toml"))?;
    for item in root.join("target").join("debug").join("tests").read_dir()? {
        let item = item?;
        let path = item.path();
        let is_test = path.extension().is_none();
        if is_test {
            let file_name = path.file_name().unwrap();
            if file_name == "test_freight_main" {
                logger.main_unit_test()?;
            } else if file_name == "test_freight_lib" {
                logger.lib_unit_test()?;
            }
            Command::new(path).args(&test_args).spawn()?.wait()?;
        }
    }

    let lib = root.join("src").join("lib.rs");
    if lib.exists() {
        logger.doc_test(&manifest.crate_name)?;
        RustDoc::new(
            manifest.edition,
            manifest.crate_name,
            root.join("target").join("debug"),
        )
        .test(lib)?;
    }
    Ok(())
}

fn root_dir() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    for ancestor in current_dir.ancestors() {
        if ancestor.join("Freight.toml").exists() {
            return Ok(ancestor.into());
        }
    }
    Err("No root dir".into())
}
