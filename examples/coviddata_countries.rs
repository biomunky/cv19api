use covid19::coviddata;

fn main() {
    let country_data = coviddata::api::countries_data();

    if let Ok(resp) = country_data {
        println!("{:?}", resp.first().unwrap());
        for entry in resp.iter() {
            println!("{:?}", entry.country.name)
        }
    }
}
