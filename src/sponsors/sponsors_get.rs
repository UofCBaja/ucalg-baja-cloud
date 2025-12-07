use actix_web::{Responder, get, web};
use darkicewolf50_actix_setup::log_incoming;
use serde::{Deserialize, Serialize};
use serde_yaml_bw;
use std::{collections::HashMap, env, fs};

#[derive(Debug, Serialize, Deserialize)]
struct Sponsor {
    #[serde(rename = "SponsorName")]
    sponsor_name: String,
    #[serde(rename = "LogoUrl")]
    logo_url: Option<String>,
    #[serde(rename = "Url")]
    sponsor_site_url: Option<String>,
    #[serde(rename = "DescriptionAboutSponsor")]
    description_about_sponsor: Option<String>,
}

#[get("/sponsors")]
pub async fn get_sponsors() -> impl Responder {
    log_incoming("GET", "/sponsors");

    let sponsor_database_path = match env::var("SPONSOR_DATABASE") {
        Ok(path_value) => path_value,
        Err(_) => "./Database/sponsorship.yaml".to_string(),
    };

    let yaml = fs::read_to_string(sponsor_database_path).unwrap_or_else(|_| "".to_string());

    let yaml: HashMap<String, Vec<Sponsor>> = serde_yaml_bw::from_str(&yaml)
        .unwrap_or_else(|_| HashMap::from([("".to_string(), vec![])]));

    web::Json(yaml)
}
