use std::io::{BufRead, BufReader};

fn main() {
    let a = ureq::get("https://lichess.org/api/tv/feed")
        .call()
        .unwrap()
        .into_body()
        .into_reader();

    let a = BufReader::new(a);

    for line in a.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
            }
            Err(_) => todo!(),
        }
    }
}
