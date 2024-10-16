use std::io::Write;

fn main() {
    // {
    //     let body = self
    //         .client
    //         .get(BASE_URL.to_string() + HOME_URL)
    //         .send()?
    //         .text()?;
    //     let current_dir = std::env::current_dir().unwrap();
    //     eprintln!("current_dir = {:?}", current_dir);
    //     let mut file =
    //         std::fs::File::create("tests/external/atcoder_get_home_logged_in.html").unwrap();
    //     file.write_all(body.as_bytes()).unwrap();
    // }

    let client = reqwest::blocking::Client::new();
    let body = client
        .get("https://atcoder.jp/contests/abc376/tasks_print")
        .send()
        .unwrap()
        .text()
        .unwrap();
    let mut file = std::fs::File::create(
        "crates/infrastructure/tests/external/atcoder_get_tasks_not_started.html",
    )
    .unwrap();
    file.write_all(body.as_bytes()).unwrap();
}
