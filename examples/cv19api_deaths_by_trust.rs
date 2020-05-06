use chrono::NaiveDate;
use covid19::cv19api;

fn main() {
    let deaths_by_trust = cv19api::api::deaths_by_trust();

    if let Ok(resp) = deaths_by_trust {
        println!("{:?}", resp.data[0]);
        println!("{:?}", resp.metadata)
    }

    let from = NaiveDate::from_ymd(2020, 4, 15);
    let to = NaiveDate::from_ymd(2020, 4, 25);
    let use_ground_truth = true;
    let parameterised = cv19api::api::parameterised_deaths_by_trust(from, to, use_ground_truth);

    println!("{:?}", parameterised)
}
