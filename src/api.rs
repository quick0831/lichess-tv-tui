use std::{
    io::{BufRead, BufReader},
    str::FromStr,
    sync::mpsc::SyncSender,
};

use color_eyre::Result;

use serde::{Deserialize, Deserializer, de::Visitor};
use shakmaty::{Square, fen::Fen};

#[allow(dead_code)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum TvData {
    #[serde(rename = "featured")]
    Featured(FeaturedData),
    #[serde(rename = "fen")]
    Fen(FenData),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FeaturedData {
    pub id: String,
    pub orientation: Color,
    pub players: [Player; 2],
    pub fen: Fen,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Player {
    pub color: Color,
    pub user: User,
    pub rating: i32,
    pub seconds: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub title: Option<String>,
    pub flair: Option<String>,
    #[serde(default)]
    pub patron: bool,
    pub id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FenData {
    pub fen: Fen,
    #[serde(deserialize_with = "parse_lm")]
    pub lm: [Square; 2],
    pub wc: i32,
    pub bc: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub enum Color {
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

pub fn get_lichess_tv(tx: SyncSender<TvData>) {
    let a = ureq::get("https://lichess.org/api/tv/feed")
        .call()
        .unwrap()
        .into_body()
        .into_reader();

    let a = BufReader::new(a);

    for line in a.lines() {
        match line {
            Ok(line) => match serde_json::from_str::<TvData>(&line) {
                Ok(data) => tx.send(data).expect("channel is closed"),
                Err(err) => panic!("JSON parse failed: {err}"),
            },
            Err(_) => todo!(),
        }
    }
}
