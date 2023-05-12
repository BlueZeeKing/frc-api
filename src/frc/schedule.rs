use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum TournamentLevel {
    Playoff,
    Qualification,
}

#[derive(Debug, Deserialize)]
pub enum MatchStations {
    Red1,
    Red2,
    Red3,
    Blue1,
    Blue2,
    Blue3,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleTeam {
    pub team_number: u16,
    pub station: MatchStations,
    pub surrogate: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledMatch {
    pub description: String,
    pub start_time: String,
    pub match_number: u16,
    pub field: String,
    pub tournament_level: TournamentLevel,
}
