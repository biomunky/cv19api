use covid19;

fn main() {
    let deaths_by_trust = covid19::api::deaths_by_trust();

    if let Ok(resp) = deaths_by_trust {
        println!("{:?}", resp.data[0]);
        println!("{:?}", resp.metadata)
    }
}
