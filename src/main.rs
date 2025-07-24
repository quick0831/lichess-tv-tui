use std::{
    io::{BufRead, BufReader},
    str::FromStr,
};

use serde::{Deserialize, Deserializer, de::Visitor};
use shakmaty::{Square, fen::Fen};

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
    fen: Fen,
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
    fen: Fen,
    #[serde(deserialize_with = "parse_lm")]
    lm: [Square; 2],
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

fn parse_lm<'de, D>(deserializer: D) -> Result<[Square; 2], D::Error>
where
    D: Deserializer<'de>,
{
    struct StrVisitor;

    impl Visitor<'_> for StrVisitor {
        type Value = [Square; 2];

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a standard UCI chess move")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok([
                Square::from_str(&v[0..2]).unwrap(),
                Square::from_str(&v[2..4]).unwrap(),
            ])
        }
    }

    deserializer.deserialize_str(StrVisitor)
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
