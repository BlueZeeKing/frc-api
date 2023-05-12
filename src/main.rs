use frc_api::frc::team::Team;
use futures::StreamExt;

include!(concat!(env!("OUT_DIR"), "/districts.rs"));

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let stream = Team::get_many(
        Some("VAALE".to_string()),
        None,
        Some("Virginia".to_string()),
    );

    tokio::pin!(stream);

    while let Some(item) = stream.next().await {
        println!("{:#?}", item)
    }
}
