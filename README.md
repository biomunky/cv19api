# A Quick and Dirty Rust wrapper for [cv19api](https://lbandc.github.io/2020/04/14/project-cv19api.html)

Wraps [cv19api](https://lbandc.github.io/2020/04/14/project-cv19api.html), a weekend project on Covid by the Lancaster Beer & Code collective.

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

## Example
Quick and dirty usage

    cargo run --example get_deaths
