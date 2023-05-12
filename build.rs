use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{env, fs, path::Path};

#[derive(Deserialize)]
struct District {
    code: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DistrictList {
    districts: Vec<District>,
    // district_count: u8,
}

fn main() {
    dotenv().expect(".env file not found");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("districts.rs");

    let districts = Client::default()
        .get("https://frc-api.firstinspires.org/v3.0/2023/districts")
        .basic_auth(
            env::var("API_USERNAME").expect("Could not find username"),
            Some(env::var("API_PASSWORD").expect("Could not find password")),
        )
        .send()
        .unwrap()
        .json::<DistrictList>()
        .unwrap()
        .districts;

    fs::write(
        dest_path,
        format!(
            "
#[derive(PartialEq, serde::Deserialize, Debug)]
pub enum Districts {{
{}
}}

impl Districts {{
    pub fn get_name(&self) -> &'static str {{
        match self {{
        {}
        }}
    }}
}}",
            districts
                .iter()
                .map(|district| format!("    {},", district.code.to_uppercase()))
                .collect::<Vec::<_>>()
                .join("\n"),
            districts
                .iter()
                .map(|district| format!(
                    "    Districts::{} => \"{}\",",
                    district.code.to_uppercase(),
                    district.name
                ))
                .collect::<Vec::<_>>()
                .join("\n")
        ),
    )
    .unwrap();
}
