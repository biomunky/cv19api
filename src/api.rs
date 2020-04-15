use super::data::{DeathsResponse, RegionResponse, TrustResponse};
use anyhow::{anyhow, Result};

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

pub fn deaths() -> Result<DeathsResponse> {
    let mut url = BASE_URL.to_owned();
    url.push_str("deaths");

    let r = fetch_resource(&url)?;
    Ok(serde_json::from_str::<DeathsResponse>(&r)?)
}

pub fn deaths_by_region() -> Result<RegionResponse> {
    let mut url = BASE_URL.to_owned();
    url.push_str("deaths/regions");

    let r = fetch_resource(&url)?;
    Ok(serde_json::from_str::<RegionResponse>(&r)?)
}

pub fn deaths_by_trust() -> Result<TrustResponse> {
    let mut url = BASE_URL.to_owned();
    url.push_str("deaths/trusts");

    let r = fetch_resource(&url)?;
    Ok(serde_json::from_str::<TrustResponse>(&r)?)
}
