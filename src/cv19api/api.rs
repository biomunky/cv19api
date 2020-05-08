use super::data::{DeathsResponse, RegionsResponse, TrustsResponse};
use crate::utils::fetch;
use anyhow::Result;
use chrono::NaiveDate;
use serde::de::DeserializeOwned;

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
        (Some(from_date), Some(to_date)) => {
            url_for(BASE_URL, endpoint, use_ground_truth, from_date, to_date)
        }
        _ => format!("{}{}", BASE_URL, endpoint),
    };
    fetch(&url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn having_none_or_true_for_ground_truth_sets_from_to_param_string() {
        let use_ground_truth_none: Option<bool> = None;
        let use_ground_truth_true = Some(true);
        let base_url = "";
        let endpoint = "";
        let from_date = NaiveDate::from_ymd(2020, 05, 4);
        let to_date = NaiveDate::from_ymd(2020, 05, 12);

        let constructed_url_with_none_ground_truth = url_for(
            base_url,
            endpoint,
            use_ground_truth_none,
            from_date,
            to_date,
        );
        assert_eq!(
            "?from=2020-05-04&to=2020-05-12",
            constructed_url_with_none_ground_truth
        );

        let constructed_url_with_true_ground_truth = url_for(
            base_url,
            endpoint,
            use_ground_truth_true,
            from_date,
            to_date,
        );
        assert_eq!(
            "?from=2020-05-04&to=2020-05-12",
            constructed_url_with_true_ground_truth
        );
    }

    #[test]
    fn having_false_for_ground_truth_sets_recordedonfrom_recordedonto_to_param_string() {
        let use_ground_truth_false = Some(false);
        let base_url = "";
        let endpoint = "";
        let from_date = NaiveDate::from_ymd(2020, 05, 4);
        let to_date = NaiveDate::from_ymd(2020, 05, 12);

        let constructed_url_with_false_ground_truth = url_for(
            base_url,
            endpoint,
            use_ground_truth_false,
            from_date,
            to_date,
        );

        assert_eq!(
            "?recordedOnFrom=2020-05-04&recordedOnTo=2020-05-12",
            constructed_url_with_false_ground_truth
        );
    }
}
