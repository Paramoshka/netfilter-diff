#[derive(Debug, Clone, Copy)]
pub enum DetectBackend {
    NftOnly,
    IptablesLegacy,
    IptablesNfTables,
}

pub const mark_nft: &str = "nf_tables";
