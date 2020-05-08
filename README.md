# A Quick and Dirty Rust wrapper for [cv19api](https://lbandc.github.io/2020/04/14/project-cv19api.html) and [coviddata](https://coviddata.github.io/coviddata/)

## Why?

Why not? It's fun to learn new things.

## [cv19api](https://lbandc.github.io/2020/04/14/project-cv19api.html)

Handles the following endpoints and returns an Ok(Response) for each. Each Endpoint has its own response type rather than making the trust and region fields optional.

```bash
    /deaths

    {
        "data": [
            {
                "date": "2020-04-14",
                "deaths": 113
            },
        ...
        ]
        ,
        "metaData": {
            "recordedOnTo": "2020-04-15",
            "from": "2020-03-16",
            "to": "2020-04-15",
            "recordedOnFrom": "2020-01-01"
        }
    }
```

```bash
    /deaths/trusts

        {
        "data": [
            {
                "date": "2020-04-14",
                "trust": "AIREDALE NHS FOUNDATION TRUST",
                "deaths": 0
            },
        ...
        ]
        ,
        "metaData": {
            "recordedOnTo": "2020-04-15",
            "from": "2020-03-16",
            "to": "2020-04-15",
            "recordedOnFrom": "2020-01-01"
        }
    }
```

```bash
    /deaths/regions

        {
        "data": [
            {
                "date": "2020-04-14",
                "region": "EAST_OF_ENGLAND",
                "deaths": 41
            },
        ...
        ]
        ,
        "metaData": {
            "recordedOnTo": "2020-04-15",
            "from": "2020-03-16",
            "to": "2020-04-15",
            "recordedOnFrom": "2020-01-01"
        }
    }
```

## [coviddata](https://coviddata.github.io/coviddata/)

```bash
    v1/countries/stats.json

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
```

```bash
    v1/regions/stats.json

    [
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
    ]
```

```bash
    v1/places/stats.json
    [
        {
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
        }
    ]
```

## Examples
Quick and dirty usage

    cargo run --example cv19api_deaths_by_region
    cargo run --example cv19api_deaths_by_trust
    cargo run --example cv19api_deaths
    cargo run --example coviddata_countries
