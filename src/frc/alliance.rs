use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alliance {
    pub number: u8,
    pub captain: u16,
    pub round1: u16,
    pub round2: u16,
    pub round3: Option<u16>,
    pub backup: Option<u16>,
    pub backup_replaced: Option<u16>,
    pub name: String,
}
