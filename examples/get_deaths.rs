use covid19;

fn main() {
    let deaths = covid19::api::deaths();

    if let Ok(resp) = deaths {
        println!("{:?}", resp.data[0]);
        println!("{:?}", resp.metadata)
    }
}
