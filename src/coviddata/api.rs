use super::data::{Countries, Places, Regions};
use anyhow::Result;

use crate::utils::fetch;

const BASE_URL: &str = "https://coviddata.github.io/coviddata/";

pub fn countries_data() -> Result<Countries> {
    fetch(&format!("{}{}", BASE_URL, "v1/countries/stats.json"))
}

pub fn regions_data() -> Result<Regions> {
    fetch(&format!("{}{}", BASE_URL, "v1/regions/stats.json"))
}

pub fn places_data() -> Result<Places> {
    fetch(&format!("{}{}", BASE_URL, "v1/places/stats.json"))
}
