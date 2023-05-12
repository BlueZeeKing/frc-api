use std::env;

use lazy_static::lazy_static;
use reqwest::{Client, RequestBuilder};

pub mod error;
pub mod frc;

lazy_static! {
    pub static ref CLIENT: Client = Client::default();
}

pub const BASE_URL: &'static str = "https://frc-api.firstinspires.org/v3.0";

fn add_auth(req: RequestBuilder) -> RequestBuilder {
    req.basic_auth(
        env::var("API_USERNAME").expect("Could not find username"),
        Some(env::var("API_PASSWORD").expect("Could not find password")),
    )
}
