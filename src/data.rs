use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeathsResponse {
    pub data: Vec<Deaths>,
    #[serde(rename = "metaData")]
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionsResponse {
    pub data: Vec<RegionDeaths>,
    #[serde(rename = "metaData")]
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustsResponse {
    pub data: Vec<TrustDeaths>,
    #[serde(rename = "metaData")]
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deaths {
    pub deaths: u32,
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionDeaths {
    pub deaths: u32,
    pub date: NaiveDate,
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustDeaths {
    pub deaths: u32,
    pub date: NaiveDate,
    pub trust: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "recordedOnTo")]
    pub recorded_on_to: NaiveDate,
    pub from: NaiveDate,
    pub to: NaiveDate,
    #[serde(rename = "recordedOnFrom")]
    pub recorded_on_from: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::DeserializeOwned;
    use std::fs;
    use std::path::PathBuf;

    fn load_file(filename: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(filename);
        fs::read_to_string(d.as_path()).unwrap_or_else(|_| format!("Unable to read {}", filename))
    }

    fn read_json<T>(filename: &str) -> T
    where
        T: DeserializeOwned,
    {
        let contents = load_file(filename);
        serde_json::from_str(&contents).unwrap()
    }

    #[test]
    fn deserialize_deaths_response() {
        let resp: DeathsResponse = read_json("resources/deaths.json");
        let meta = resp.metadata;
        let data = resp.data;
        let first_point = &data[0];

        assert_eq!(meta.from, NaiveDate::from_ymd(2020, 3, 16));
        assert_eq!(meta.to, NaiveDate::from_ymd(2020, 4, 15));
        assert_eq!(data.len(), 30);
        assert_eq!(first_point.deaths, 113);
        assert_eq!(first_point.date, NaiveDate::from_ymd(2020, 4, 14));
    }

    #[test]
    fn deserialize_region_response() {
        let resp: RegionsResponse = read_json("resources/deaths-regions.json");
        let data = resp.data;
        let first_point = &data[0];

        assert_eq!(first_point.deaths, 41);
        assert_eq!(first_point.region, "EAST_OF_ENGLAND");
        assert_eq!(first_point.date, NaiveDate::from_ymd(2020, 4, 14));
    }

    #[test]
    fn deserialize_trust_response() {
        let resp: TrustsResponse = read_json("resources/deaths-trusts.json");
        let data = resp.data;
        let first_point = &data[0];

        assert_eq!(first_point.deaths, 0);
        assert_eq!(first_point.trust, "AIREDALE NHS FOUNDATION TRUST");
        assert_eq!(first_point.date, NaiveDate::from_ymd(2020, 4, 14));
    }
}
