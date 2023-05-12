use futures::{stream, Stream, StreamExt};
use serde::Deserialize;

include!(concat!(env!("OUT_DIR"), "/districts.rs"));

use crate::{add_auth, error::Error, BASE_URL, CLIENT};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub team_number: u16,
    pub name_full: String,
    pub name_short: String,
    pub city: String,
    pub state_prov: String,
    pub country: String,
    pub rookie_year: u32,
    pub robot_name: String,
    pub district_code: Option<Districts>,
    pub school_name: String,
    pub website: String,
    #[serde(alias = "homeCMP")]
    pub home_cmp: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Teams {
    teams: Vec<Team>,
    page_total: u32,
}

impl Team {
    pub async fn get(number: u16) -> Result<Self, Error> {
        match add_auth(CLIENT.get(format!("{BASE_URL}/2023/teams?teamNumber={number}")))
            .send()
            .await?
            .json::<Teams>()
            .await?
            .teams
            .into_iter()
            .nth(0)
        {
            Some(val) => Ok(val),
            None => Err(Error::CannotFindTeam(number)),
        }
    }

    pub fn get_many(
        event: Option<String>,
        district: Option<Districts>,
        state: Option<String>,
    ) -> impl Stream<Item = Team> {
        let mut params = Vec::new();

        if let Some(event) = &event {
            params.push(["eventCode".to_owned(), event.to_owned()])
        }

        if let Some(district) = &district {
            params.push(["districtCode".to_owned(), format!("{:?}", district)])
        }

        if let Some(state) = &state {
            params.push(["state".to_owned(), state.to_owned()])
        }

        stream::unfold(1, move |page| {
            let params = params.clone();

            async move {
                let res = add_auth(CLIENT.get(format!("{BASE_URL}/2023/teams")))
                    .query(&params)
                    .query(&[["page", &format!("{page}")]])
                    .send()
                    .await
                    .unwrap();

                let res = res.json::<Teams>().await.unwrap();

                if page <= res.page_total {
                    Some((res.teams, page + 1))
                } else {
                    None
                }
            }
        })
        .flat_map(|teams| tokio_stream::iter(teams.into_iter()))
    }

    pub fn get_all() -> impl Stream<Item = Team> {
        Self::get_many(None, None, None)
    }
}
