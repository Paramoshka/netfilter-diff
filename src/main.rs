mod models;
use models::DetectBackend;
use std::process::Command;

use crate::models::mark_nft;

fn detect_netfilter_backend() -> Result<DetectBackend, String> {
    let out = Command::new("iptables")
        .arg("-V")
        .output()
        .map_err(|e| format!("iptables -V failed: {e}"))?;

    if !out.status.success() {
        return Err(format!("iptables -V exit={}", out.status));
    }

    let s = String::from_utf8_lossy(&out.stdout);

    if s.contains(mark_nft) {
        Ok(DetectBackend::IptablesNfTables)
    } else {
        Ok(DetectBackend::IptablesLegacy)
    }
}
fn main() {
    match detect_netfilter_backend() {
        Ok(DetectBackend::IptablesNfTables) => {}

        Ok(DetectBackend::IptablesLegacy) => {}

        Ok(DetectBackend::NftOnly) => {}

        Err(e) => {
            eprintln!("Failed get nft backend: {e}")
        }
    }
}
