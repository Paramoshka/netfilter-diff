mod cli;
mod models;
use clap::Parser;
use cli::Cli;
use models::DetectBackend;
use std::process::Command;

use crate::models::MARK_NFT;

fn detect_netfilter_backend() -> Result<DetectBackend, String> {
    let out = Command::new("iptables")
        .arg("-V")
        .output()
        .map_err(|e| format!("iptables -V failed: {e}"))?;

    if !out.status.success() {
        return Err(format!("iptables -V exit={}", out.status));
    }

    let s = String::from_utf8_lossy(&out.stdout);

    if s.contains(MARK_NFT) {
        Ok(DetectBackend::IptablesNfTables)
    } else {
        Ok(DetectBackend::IptablesLegacy)
    }
}
fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        Some(_) => todo!(),
        None => println!("emtry output file"),
    };

    match detect_netfilter_backend() {
        Ok(DetectBackend::IptablesNfTables) => {}

        Ok(DetectBackend::IptablesLegacy) => {}

        Ok(DetectBackend::NftOnly) => {}

        Err(e) => {
            eprintln!("Failed get nft backend: {e}")
        }
    }
}
