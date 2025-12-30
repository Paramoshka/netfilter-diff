#[derive(Debug, Clone, Copy)]
pub enum DetectBackend {
    NftOnly,
    IptablesLegacy,
    IptablesNfTables,
}

pub const MARK_NFT: &str = "nf_tables";
