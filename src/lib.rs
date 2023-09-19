use std::{
    error::Error,
    fmt::{self, Display},
    path::PathBuf,
    process::Command,
};

pub enum Edition {
    E2015,
    E2018,
    E2021,
}

impl Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let edition = match self {
            Self::E2015 => "2015",
            Self::E2018 => "2018",
            Self::E2021 => "2021",
        };
        write!(f, "{edition}")
    }
}

pub enum CrateType {
    Bin,
    Lib,
    RLib,
    DyLib,
    CDyLib,
    StaticLib,
    ProcMacro,
}

impl Display for CrateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crate_type = match self {
            Self::Bin => "bin",
            Self::Lib => "lib",
            Self::RLib => "rlib",
            Self::DyLib => "dylib",
            Self::CDyLib => "cdylib",
            Self::StaticLib => "staticlib",
            Self::ProcMacro => "proc-macro",
        };
        write!(f, "{crate_type}")
    }
}

pub struct Rustc {
    edition: Edition,
    crate_type: CrateType,
    crate_name: String,
    out_dir: PathBuf,
    lib_dir: PathBuf,
    cfg: Vec<String>,
    externs: Vec<String>,
}

impl Rustc {
    pub fn run(self, path: &str) -> Result<(), Box<dyn Error>> {
        Command::new("rustc")
            .arg(path)
            .arg("--edition")
            .arg(self.edition.to_string())
            .arg("--crate-type")
            .arg(self.crate_type.to_string())
            .arg("--crate-name")
            .arg(self.crate_name)
            .arg("--out-dir")
            .arg(self.out_dir)
            .arg("-L")
            .arg(self.lib_dir)
            .args(
                self.externs
                    .into_iter()
                    .map(|r#extern| ["--extern".into(), r#extern])
                    .flatten(),
            )
            .args(
                self.cfg
                    .into_iter()
                    .map(|cfg| ["--cfg".into(), cfg])
                    .flatten(),
            )
            .spawn()?
            .wait()?;
        Ok(())
    }

    pub fn builder() -> RustcBuilder {
        RustcBuilder::default()
    }
}

#[derive(Default)]
pub struct RustcBuilder {
    edition: Option<Edition>,
    crate_type: Option<CrateType>,
    crate_name: Option<String>,
    out_dir: Option<PathBuf>,
    lib_dir: Option<PathBuf>,
    cfg: Vec<String>,
    externs: Vec<String>,
}

impl RustcBuilder {
    pub fn edition(mut self, edition: Edition) -> Self {
        self.edition = Some(edition);
        self
    }

    pub fn out_dir(mut self, out_dir: impl Into<PathBuf>) -> Self {
        self.out_dir = Some(out_dir.into());
        self
    }

    pub fn lib_dir(mut self, lib_dir: impl Into<PathBuf>) -> Self {
        self.lib_dir = Some(lib_dir.into());
        self
    }

    pub fn crate_name(mut self, crate_name: impl Into<String>) -> Self {
        self.crate_name = Some(crate_name.into());
        self
    }

    pub fn crate_type(mut self, crate_type: impl Into<CrateType>) -> Self {
        self.crate_type = Some(crate_type.into());
        self
    }

    pub fn cfg(mut self, cfg: impl Into<String>) -> Self {
        self.cfg.push(cfg.into());
        self
    }

    pub fn externs(mut self, r#extern: impl Into<String>) -> Self {
        self.externs.push(r#extern.into());
        self
    }

    pub fn done(self) -> Rustc {
        Rustc {
            edition: self.edition.unwrap_or(Edition::E2015),
            crate_type: self.crate_type.expect("Crate type given"),
            crate_name: self.crate_name.expect("Crate name given"),
            out_dir: self.out_dir.expect("Out dir given"),
            lib_dir: self.lib_dir.expect("Lib dir given"),
            cfg: self.cfg,
            externs: self.externs,
        }
    }
}