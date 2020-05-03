use chrono::NaiveDate;

fn main() {
    let deaths = covid19::api::deaths();

    if let Ok(resp) = deaths {
        println!("{:?}", resp.data[0]);
        println!("{:?}", resp.metadata)
    }

    let from = NaiveDate::from_ymd(2020, 4, 15);
    let to = NaiveDate::from_ymd(2020, 4, 25);
    let use_ground_truth = false;
    let parameterised = covid19::api::parameterised_deaths(from, to, use_ground_truth);

    println!("{:?}", parameterised)
}
