use crate::target_os::{TargetOs, TargetOsSpecific};
use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
    process::Stdio,
};

pub fn srcds_dir(steam_dir: &Path, app_id: u32) -> Result<PathBuf, Box<dyn Error>> {
    let steamapps_dir = steam_dir.join("steamapps");

    let manifest = steamy_vdf::load(steamapps_dir.join(format!("appmanifest_{}.acf", app_id)))?;

    let last_updated = manifest
        .lookup("AppState.LastUpdated")
        .and_then(steamy_vdf::Entry::as_str)
        .unwrap_or("0");

    if last_updated == "0" {
        Err("game server was never installed")?;
    }

    let install_dir = manifest
        .lookup("AppState.installdir")
        .and_then(steamy_vdf::Entry::as_str)
        .ok_or("manifest is missing game server directory")?;

    Ok(steamapps_dir.join("common").join(install_dir))
}

pub fn exit_steam(steam_root: &Path) -> io::Result<()> {
    TargetOs::steam_cmd(&steam_root)
        .arg("-shutdown")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}

pub fn install_srcds(steam_root: &Path, app_id: u32) -> io::Result<()> {
    TargetOs::steam_cmd(&steam_root)
        .args(&["-login", "anonymous"])
        .arg(format!("steam://install/{}", app_id))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    TargetOs::steam_cmd(&steam_root)
        .arg("steam://nav/downloads")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}
