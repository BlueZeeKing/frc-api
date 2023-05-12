use serde::Deserialize;

include!(concat!(env!("OUT_DIR"), "/districts.rs"));

#[derive(Debug, Deserialize)]
pub enum AllianceCount {
    EightAlliance,
    FourAlliance,
}

#[derive(Debug, Deserialize)]
pub enum EventType {
    OffSeasonWithAzureSync,
    ChampionshipDivision,
    Regional,
    DistrictEvent,
    DistrictChampionship,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub alliance_count: AllianceCount,
    pub week_number: u8,
    pub announcements: Vec<String>,
    pub code: String, // can be improved
    pub division_code: Districts,
    pub name: String,
    #[serde(alias = "type")]
    pub event_type: EventType,
    pub district_code: Option<String>,
    pub venue: String,
    pub city: String,
    pub stateprov: String,
    pub country: String,
    pub date_start: String,
    pub date_end: String,
    pub address: String,
    pub website: String,
    pub webcasts: Vec<String>,
    pub timezone: String,
}
