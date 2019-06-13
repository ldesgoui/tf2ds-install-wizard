use std::{
    ffi::OsString,
    io::{self, Write as _},
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(target_os = "linux")]
pub type TargetOs = Linux;

#[cfg(target_os = "windows")]
pub type TargetOs = Windows;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
compile_error!("Unsupported platform");

pub trait TargetOsSpecific {
    fn steam_dir() -> Option<PathBuf>;
    fn steam_cmd(steam_dir: &Path) -> Command;
    fn write_script(srcds_dir: &Path) -> io::Result<()>;
}

pub struct Linux;

impl TargetOsSpecific for Linux {
    fn steam_dir() -> Option<PathBuf> {
        let base = directories::BaseDirs::new()?;

        let dir = base.data_local_dir().join("Steam");

        if dir.is_dir() {
            return Some(dir);
        }

        None
    }

    fn steam_cmd(_: &Path) -> Command {
        Command::new("steam")
    }

    fn write_script(srcds_dir: &Path) -> io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .mode(0o755)
            .open("./run-tf2ds.sh")?;

        write!(
            file,
            "{:?} +sv_pure 2 +map cp_process_final +maxplayers 24",
            srcds_dir.join("srcds_run"),
        )
    }
}

pub struct Windows;

impl TargetOsSpecific for Windows {
    fn steam_dir() -> Option<PathBuf> {
        for base in &["/Program Files (x86)", "/Program Files"] {
            let dir: PathBuf = [base, "Steam"].iter().collect();

            if dir.is_dir() {
                return Some(dir);
            }
        }

        None
    }

    fn steam_cmd(steam_dir: &Path) -> Command {
        Command::new(steam_dir.join("Steam.exe"))
    }

    fn write_script(srcds_dir: &Path) -> io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("./run-tf2ds.bat")?;

        write!(
            file,
            "{}/srcds.exe -game tf -console +sv_pure 2 +map cp_process_final +maxplayers 24",
            srcds_dir.to_string_lossy()
        )
    }
}
