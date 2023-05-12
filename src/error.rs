#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    CannotFindTeam(u16),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
