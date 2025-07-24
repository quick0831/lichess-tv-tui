use std::io::{BufRead, BufReader};

use serde::Deserialize;

#[allow(dead_code)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Deserialize)]
#[serde(tag = "t", content = "d")]
enum TvData {
    #[serde(rename = "featured")]
    Featured(FeaturedData),
    #[serde(rename = "fen")]
    Fen(FenData),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FeaturedData {
    id: String,
    orientation: Color,
    players: [Player; 2],
    fen: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Player {
    color: Color,
    user: User,
    rating: i32,
    seconds: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct User {
    name: String,
    title: Option<String>,
    flair: Option<String>,
    #[serde(default)]
    patron: bool,
    id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FenData {
    fen: String,
    lm: String,
    wc: i32,
    bc: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
enum Color {
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
}

fn main() {
    let a = ureq::get("https://lichess.org/api/tv/feed")
        .call()
        .unwrap()
        .into_body()
        .into_reader();

    let a = BufReader::new(a);

    for line in a.lines() {
        match line {
            Ok(line) => match serde_json::from_str::<TvData>(&line) {
                Ok(data) => println!("{:?}", data),
                Err(err) => panic!("JSON parse failed: {err}"),
            },
            Err(_) => todo!(),
        }
    }
}
