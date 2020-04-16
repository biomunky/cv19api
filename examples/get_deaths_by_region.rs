use covid19;

fn main() {
    let deaths_by_region = covid19::api::deaths_by_region();

    if let Ok(resp) = deaths_by_region {
        println!("{:?}", resp.data[0]);
        println!("{:?}", resp.metadata)
    }
}
