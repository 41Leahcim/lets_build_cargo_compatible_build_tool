use super::{rustc::Edition, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct RustDoc {
    edition: Edition,
    crate_name: String,
    lib_path: PathBuf,
}

impl RustDoc {
    pub fn new(
        edition: Edition,
        crate_name: impl Into<String>,
        lib_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            edition,
            crate_name: crate_name.into(),
            lib_path: lib_path.into(),
        }
    }

    pub fn test(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        Command::new("rustdoc")
            .arg("--test")
            .arg(path)
            .arg("--crate-name")
            .arg(&self.crate_name)
            .arg("--edition")
            .arg(self.edition.to_string())
            .arg("-L")
            .arg(&self.lib_path)
            .spawn()?
            .wait()?;
        Ok(())
    }
}
