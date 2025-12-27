mod models;
use models::DetectBackend;
use std::{fmt::format, process::Command};

fn detect_netfilter_backend() -> Result<DetectBackend, String> {
    let out = Command::new("iptables")
        .arg("-V")
        .output()
        .map_err(|e| format!("iptables -V failed: {e}"))?;

    if !out.status.success() {
        return Err(format!("iptables -V exit={}", out.status));
    }else {
        return Ok(DetectBackend::NftOnly);
    }

fn main() {
    // todo
}
