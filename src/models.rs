#[derive(Debug, Clone, Copy)]
pub enum DetectBackend {
    NftOnly,
    IptablesLegacy,
    IptablesNfTables,
}
