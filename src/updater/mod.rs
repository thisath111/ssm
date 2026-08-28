use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const REPO_OWNER: &str = "thisath111";
const REPO_NAME: &str = "ssm";
const BIN_NAME: &str = "ssm.exe";
const UPDATE_INTERVAL_SECS: u64 = 60 * 60 * 24 * 7; // 7 days

#[derive(Debug)]
pub enum UpdateStatus {
    UpToDate,
    Updated(String),
    CheckFailed(String),
}

pub fn current_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn parse_semver(v: &str) -> (u32, u32, u32) {
    let v = v.trim_start_matches('v');
    let mut parts = v.splitn(3, '.').map(|s| s.parse::<u32>().unwrap_or(0));
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

fn is_newer(remote: &str, local: &str) -> bool {
    parse_semver(remote) > parse_semver(local)
}

pub fn check_for_update() -> Result<Option<(String, String)>, String> {
    let url = format!("https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/releases/latest");

    let response = ureq::get(&url)
        .set("User-Agent", &format!("ssm/{}", env!("CARGO_PKG_VERSION")))
        .set("Accept", "application/vnd.github+json")
        .call()
        .map_err(|e| format!("network error: {e}"))?;

    let body: serde_json::Value =
        serde_json::from_reader(response.into_reader()).map_err(|e| format!("parse error: {e}"))?;

    let remote_version = body["tag_name"]
        .as_str()
        .ok_or("missing tag_name")?
        .to_string();

    let local_version = env!("CARGO_PKG_VERSION");

    if !is_newer(&remote_version, local_version) {
        return Ok(None);
    }

    let download_url = body["assets"]
        .as_array()
        .and_then(|assets| {
            assets.iter().find(|a| {
                a["name"]
                    .as_str()
                    .map(|n| n.eq_ignore_ascii_case(BIN_NAME))
                    .unwrap_or(false)
            })
        })
        .and_then(|a| a["browser_download_url"].as_str())
        .ok_or("no matching asset found in release")?
        .to_string();

    Ok(Some((remote_version, download_url)))
}

pub fn download_and_replace(version: &str, url: &str) -> Result<(), String> {
    let exe_path = std::env::current_exe().map_err(|e| format!("exe path: {e}"))?;
    let tmp_path: PathBuf = exe_path.with_extension("tmp");

    let response = ureq::get(url)
        .set("User-Agent", &format!("ssm/{}", env!("CARGO_PKG_VERSION")))
        .call()
        .map_err(|e| format!("download error: {e}"))?;

    let mut bytes: Vec<u8> = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| format!("read error: {e}"))?;

    let mut tmp_file = std::fs::File::create(&tmp_path).map_err(|e| format!("create tmp: {e}"))?;
    tmp_file
        .write_all(&bytes)
        .map_err(|e| format!("write tmp: {e}"))?;
    drop(tmp_file);

    self_replace(&tmp_path, &exe_path)?;

    log::info!("[Updater] Updated to {version} — restart ssm to apply.");
    Ok(())
}

fn self_replace(new_exe: &PathBuf, current_exe: &PathBuf) -> Result<(), String> {
    let backup = current_exe.with_extension("old");
    let _ = std::fs::remove_file(&backup);
    std::fs::rename(current_exe, &backup).map_err(|e| format!("backup rename: {e}"))?;
    std::fs::rename(new_exe, current_exe).map_err(|e| {
        let _ = std::fs::rename(&backup, current_exe);
        format!("replace failed: {e}")
    })?;
    let _ = std::fs::remove_file(&backup);
    Ok(())
}

pub fn run_update_check(config: &mut crate::utils::config::Config, verbose: bool) -> UpdateStatus {
    let now = current_unix();

    match check_for_update() {
        Ok(Some((version, url))) => {
            if verbose {
                println!("Downloading ssm {version}...");
            }
            match download_and_replace(&version, &url) {
                Ok(()) => {
                    config.last_update_check_unix = now;
                    let _ = config.save();
                    UpdateStatus::Updated(version)
                }
                Err(e) => UpdateStatus::CheckFailed(e),
            }
        }
        Ok(None) => {
            config.last_update_check_unix = now;
            let _ = config.save();
            UpdateStatus::UpToDate
        }
        Err(e) => UpdateStatus::CheckFailed(e),
    }
}

pub fn should_check(last_check_unix: u64) -> bool {
    current_unix().saturating_sub(last_check_unix) >= UPDATE_INTERVAL_SECS
}
