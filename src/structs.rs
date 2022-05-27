use anyhow::{bail, Result};
use serde::Deserialize;

#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Debug)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum AqiResponse {
    Error { data: String },
    Ok { data: serde_json::Value },
}

impl AqiResponse {
    pub fn result(self) -> Result<Vec<StationAqi>> {
        match self {
            AqiResponse::Error { data } => bail!(data),
            AqiResponse::Ok { data } => Ok(serde_json::from_value(data)?),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct StationAqi {
    pub uid: u16,
    pub aqi: String,
    pub time: Time,
    pub station: Station,
}

#[derive(Deserialize, Debug)]
pub struct Time {
    pub tz: String,
    pub stime: Option<String>,
    pub vtime: u32,
}

#[derive(Deserialize, Debug)]
pub struct Station {
    pub name: String,
    pub url: String,
    pub geo: [f64; 2],
}
