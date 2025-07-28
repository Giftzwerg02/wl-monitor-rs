use serde::{de::{self, Unexpected}, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Line {
    #[serde(rename = "LineID")]
    pub id: u64,

    #[serde(rename = "LineText")]
    pub text: String,

    #[serde(rename = "SortingHelp")]
    pub sorting: u64,

    #[serde(rename = "Realtime", deserialize_with = "bool_from_int")]
    pub realtime: bool, // 0 or 1

    #[serde(rename = "MeansOfTransport")]
    pub means_of_transport: String,
}

#[derive(Debug, Deserialize)]
pub struct Stop {
    #[serde(rename = "StopID")]
    pub id: u64,

    #[serde(rename = "DIVA")]
    pub diva: Option<u64>,

    #[serde(rename = "StopText")]
    pub stop_text: String,

    #[serde(rename = "Municipality")]
    pub municipality: Option<String>,

    #[serde(rename = "MunicipalityID")]
    pub municipality_id: Option<u64>,

    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,

    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    #[serde(rename = "LineID")]
    pub line_id: u64,

    #[serde(rename = "PatternID")]
    pub pattern_id: u64,

    #[serde(rename = "StopSeqCount")]
    pub stop_seq_count: u64,

    #[serde(rename = "StopID")]
    pub stop_id: u64,

    #[serde(rename = "Direction", default="Option::default")]
    pub direction: Option<u64>,
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub fn load_csv_data() -> Result<(Vec<Line>, Vec<Stop>, Vec<Route>), anyhow::Error> {
    let mut lines = vec![];
    let mut stops = vec![];
    let mut routes = vec![];

    let mut line_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_path("res/wienerlinien-ogd-linien.csv")?;

    let mut stops_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_path("res/wienerlinien-ogd-haltepunkte.csv")?;

    let mut routes_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_path("res/wienerlinien-ogd-fahrwegverlaeufe.csv")?;

    for result in line_reader.deserialize::<Line>() {
        lines.push(result?);
    }

    for result in stops_reader.deserialize::<Stop>() {
        stops.push(result?);
    }

    for result in routes_reader.deserialize::<Route>() {
        routes.push(result?);
    }

    Ok((lines, stops, routes))
}
