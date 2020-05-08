use chrono::NaiveDate;
use std::collections::HashMap;

pub type Places = Vec<PlaceData>;
pub type Regions = Vec<RegionData>;
pub type Countries = Vec<CountryData>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryData {
    pub country: Country,
    pub dates: HashMap<NaiveDate, DataPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionData {
    pub region: Region,
    pub dates: HashMap<NaiveDate, DataPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceData {
    pub place: Place,
    pub dates: HashMap<NaiveDate, DataPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    pub key: String,
    pub name: String,
    pub full_name: String,
    pub country: Country,
    pub region: Region,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub location_type: Option<String>,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub location_type: Option<String>,
    pub key: String,
    pub name: String,
    pub full_name: String,
    pub country: Country,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPoint {
    pub new: Stats,
    pub cumulative: Stats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub cases: u32,
    pub deaths: u32,
    pub recoveries: u32,
}

#[cfg(test)]
mod tests {
    use super::{Countries, DataPoint, Place, Places, Regions};
    use chrono::NaiveDate;

    #[test]
    fn test_countries_stats() {
        let json = r#"
        [
            {
                "country": {
                    "key": "united-states",
                    "name": "United States"
                },
                "dates": {
                    "2020-01-22": {
                        "new": {
                            "cases": 1,
                            "deaths": 0,
                            "recoveries": 0
                        },
                        "cumulative": {
                            "cases": 1,
                            "deaths": 0,
                            "recoveries": 0
                        }
                    },
                    "2020-01-23": {
                        "new": {
                            "cases": 0,
                            "deaths": 0,
                            "recoveries": 0
                        },
                        "cumulative": {
                            "cases": 1,
                            "deaths": 0,
                            "recoveries": 0
                        }
                    }
                }
            }
        ]
        "#;

        let c: Result<Countries, serde_json::Error> = serde_json::from_str(json);
        assert!(c.is_ok());
    }

    #[test]
    fn test_region_stats() {
        let json = r#"[
            {
                "region": {
                    "key": "new-york-united-states",
                    "name": "New York",
                    "full_name": "New York, United States",
                    "country": {
                        "location_type": "country",
                        "key": "united-states",
                        "name": "United States"
                    }
                },
                "dates": {
                    "2020-03-01": {
                        "new": {
                            "cases": 1,
                            "deaths": 0,
                            "recoveries": 0
                        },
                        "cumulative": {
                            "cases": 1,
                            "deaths": 0,
                            "recoveries": 0
                        }
                    },
                    "2020-03-02": {
                        "new": {
                            "cases": 0,
                            "deaths": 0,
                            "recoveries": 0
                        },
                        "cumulative": {
                            "cases": 1,
                            "deaths": 0,
                            "recoveries": 0
                        }
                    }
                }
            }
        ]"#;

        let r: Result<Regions, serde_json::Error> = serde_json::from_str(json);
        assert!(r.is_ok())
    }
    #[test]
    fn test_places() {
        let json = r#"
        [{
            "place": {
                "key": "new-york-city-new-york-united-states",
                "name": "New York City",
                "full_name": "New York City, New York, United States",
                "country": {
                    "location_type": "country",
                    "key": "united-states",
                    "name": "United States"
                },
                "region": {
                    "location_type": "region",
                    "key": "new-york-united-states",
                    "name": "New York",
                    "full_name": "New York, United States",
                    "country": {
                        "location_type": "country",
                        "key": "united-states",
                        "name": "United States"
                    }
                }
            },
            "dates": {
                "2020-03-01": {
                    "new": {
                        "cases": 1,
                        "deaths": 0,
                        "recoveries": 0
                    },
                    "cumulative": {
                        "cases": 1,
                        "deaths": 0,
                        "recoveries": 0
                    }
                },
                "2020-03-02": {
                    "new": {
                        "cases": 0,
                        "deaths": 0,
                        "recoveries": 0
                    },
                    "cumulative": {
                        "cases": 1,
                        "deaths": 0,
                        "recoveries": 0
                    }
                }
            }
        }]"#;
        let places: Places = serde_json::from_str(json).unwrap();
        let first_place = places.first().unwrap();
        let dt = first_place
            .dates
            .get(&NaiveDate::from_ymd(2020, 3, 2))
            .unwrap();
        assert_eq!(dt.new.deaths, 0)
    }

    #[test]
    fn deserialize_date() {
        let sample_date = r#"
        {
            "new": {
                "cases": 1,
                "deaths": 37,
                "recoveries": 10
            },
            "cumulative": {
                "cases": 30,
                "deaths":65,
                "recoveries": 0
            }
        }
        "#;

        let d: DataPoint = serde_json::from_str(sample_date).unwrap();
        assert_eq!(d.new.cases, 1);
        assert_eq!(d.new.deaths, 37);
        assert_eq!(d.new.recoveries, 10);
        assert_eq!(d.cumulative.cases, 30);
        assert_eq!(d.cumulative.deaths, 65);
        assert_eq!(d.cumulative.recoveries, 0);
    }

    #[test]
    fn deserialize_place() {
        let sample_data = r#"
            {
                "key": "new-york-city-new-york-united-states",
                "name": "New York City",
                "full_name": "New York City, New York, United States",
                "country": {
                    "location_type": "country",
                    "key": "united-states",
                    "name": "United States"
                },
                "region": {
                    "location_type": "region",
                    "key": "new-york-united-states",
                    "name": "New York",
                    "full_name": "New York, United States",
                    "country": {
                        "location_type": "country",
                        "key": "united-states",
                        "name": "United States"
                    }
                }
            }
        "#;

        let place: Place = serde_json::from_str(sample_data).unwrap();

        assert_eq!(place.key, "new-york-city-new-york-united-states");
        assert_eq!(place.region.country.key, "united-states");
        assert_eq!(place.country.location_type, Some("country".to_string()));
    }
}
