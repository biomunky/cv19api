use super::data::{DeathsResponse, RegionsResponse, TrustsResponse};
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use serde::de::DeserializeOwned;

fn fetch_resource(url: &str) -> Result<String> {
    static CLIENT_USER_AGENT: &str = concat!(
        "rust-",
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent(CLIENT_USER_AGENT)
        .build()?;

    let resp = client.get(url).send()?;

    if resp.status().is_success() {
        Ok(resp.text()?)
    } else {
        Err(anyhow!("{} {}", resp.status().as_u16(), resp.text()?))
    }
}

fn fetch<T>(url: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let r = fetch_resource(&url)?;
    Ok(serde_json::from_str::<T>(&r)?)
}

// TODO: unit test me
fn url_for(
    base_url: &str,
    endpoint: &str,
    use_ground_truth: Option<bool>,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> String {
    let (from_param, to_param) = if use_ground_truth.unwrap_or(true) {
        ("from", "to")
    } else {
        ("recordedOnFrom", "recordedOnTo")
    };
    format!(
        "{}{}?{}={}&{}={}",
        base_url, endpoint, from_param, from_date, to_param, to_date
    )
}

const BASE_URL: &str = "https://api.cv19api.com/api/v1/";
const DEATHS_ENDPOINT: &str = "deaths";
const REGIONS_ENDPOINT: &str = "deaths/regions";
const TRUSTS_ENDPOINT: &str = "deaths/trusts";

pub fn deaths() -> Result<DeathsResponse> {
    get_data(DEATHS_ENDPOINT, None, None, None)
}

pub fn parameterised_deaths(
    from: NaiveDate,
    to: NaiveDate,
    use_ground_truth: bool,
) -> Result<DeathsResponse> {
    get_data(
        DEATHS_ENDPOINT,
        Some(from),
        Some(to),
        Some(use_ground_truth),
    )
}

pub fn deaths_by_region() -> Result<RegionsResponse> {
    get_data(REGIONS_ENDPOINT, None, None, None)
}

pub fn parameterised_deaths_by_region(
    from: NaiveDate,
    to: NaiveDate,
    use_ground_truth: bool,
) -> Result<RegionsResponse> {
    get_data(
        REGIONS_ENDPOINT,
        Some(from),
        Some(to),
        Some(use_ground_truth),
    )
}

pub fn deaths_by_trust() -> Result<TrustsResponse> {
    get_data(TRUSTS_ENDPOINT, None, None, None)
}

pub fn parameterised_deaths_by_trust(
    from: NaiveDate,
    to: NaiveDate,
    use_ground_truth: bool,
) -> Result<TrustsResponse> {
    get_data(
        TRUSTS_ENDPOINT,
        Some(from),
        Some(to),
        Some(use_ground_truth),
    )
}

fn get_data<T>(
    endpoint: &str,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    use_ground_truth: Option<bool>,
) -> Result<T>
where
    T: DeserializeOwned,
{
    
    let url = match (from, to) {
        (Some(from_date), Some(to_date)) => url_for(BASE_URL, endpoint, use_ground_truth, from_date, to_date),
        _ => format!("{}{}", BASE_URL, endpoint),
    };
    fetch(&url)
}
