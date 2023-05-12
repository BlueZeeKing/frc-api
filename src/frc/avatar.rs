use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub team_number: u32,
    pub encoded_avatar: String,
}
