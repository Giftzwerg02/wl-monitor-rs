use actix_web::http;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::{bail, Context, anyhow};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub data: Data,
    pub message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub location_stop: LocationStop,
    pub lines: Vec<Line>,
    pub attributes: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationStop {
    #[serde(rename = "type")]
    pub geo_type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geo_type: String,
    pub coordinates: [f64; 2],
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub name: String,
    pub title: String,
    pub municipality: String,
    pub municipality_id: i32,
    #[serde(rename = "type")]
    pub stop_type: String,
    pub coord_name: String,
    pub attributes: Attributes,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub rbl: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub name: String,
    pub towards: String,
    pub direction: String,
    pub platform: String,
    pub richtungs_id: String,
    pub barrier_free: bool,
    pub realtime_supported: bool,
    #[serde(rename = "trafficjam")]
    pub traffic_jam: bool,
    pub departures: Departures,
    #[serde(rename = "type")]
    pub transport_type: String,
    pub line_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Departures {
    pub departure: Vec<Departure>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    pub departure_time: DepartureTime,
    pub vehicle: Vehicle,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartureTime {
    pub time_planned: DateTime<Utc>,
    pub time_real: DateTime<Utc>,
    pub countdown: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub name: String,
    pub towards: String,
    pub direction: String,
    pub platform: String,
    pub richtungs_id: String,
    pub barrier_free: bool,
    pub folding_ramp: bool,
    pub realtime_supported: bool,
    #[serde(rename = "trafficjam")]
    pub traffic_jam: bool,
    #[serde(rename = "type")]
    pub vehicle_type: String,
    pub attributes: serde_json::Value,
    pub linien_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub value: String,
    pub message_code: i32,
    pub server_time: DateTime<Utc>,
}


pub async fn get_by_diva(diva: u64) -> anyhow::Result<ResponseData> {
    surf::get(format!("https://www.wienerlinien.at/ogd_realtime/monitor?diva={diva}"))
        .recv_json::<ResponseData>()
        .await
        .map_err(|err| anyhow!("get_by_diva exploded: {err}"))
}

pub async fn get_by_stop_id(stop_id: u64) -> anyhow::Result<ResponseData> {
    surf::get(format!("https://www.wienerlinien.at/ogd_realtime/monitor?stopId={stop_id}"))
        .recv_json::<ResponseData>()
        .await
        .map_err(|err| anyhow!("get_by_stop_id exploded: {err}"))
}
