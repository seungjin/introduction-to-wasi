use waki::Client;

fn main() {
    let resp = Client::new()
        .get("https://ifconfig.io/all.json")
        .headers([("Content-Type", "application/json"), ("Accept", "*/*")])
        .send()
        .unwrap();

    println!(
        "status code: {}\n body:\n{}",
        resp.status_code(),
        String::from_utf8(resp.body().unwrap()).unwrap()
    );
}
