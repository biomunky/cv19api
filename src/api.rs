use super::data::{DeathsResponse, RegionResponse, TrustResponse};
use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;

const BASE_URL: &str = "https://api.cv19api.com/api/v1/";

pub fn fetch_resource(url: &str) -> Result<String> {
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

pub fn deaths() -> Result<DeathsResponse> {
    let url = format!("{}{}", BASE_URL, "deaths");
    fetch::<DeathsResponse>(&url)
}

pub fn deaths_by_region() -> Result<RegionResponse> {
    let url = format!("{}{}", BASE_URL, "deaths/regions");
    fetch::<RegionResponse>(&url)
}

pub fn deaths_by_trust() -> Result<TrustResponse> {
    let url = format!("{}{}", BASE_URL, "deaths/trusts");
    fetch::<TrustResponse>(&url)
}
