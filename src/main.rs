#[macro_use]
extern crate diesel;
extern crate reqwest;
extern crate scraper;
extern crate dotenv;

use scraper::{Html, Selector};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use self::models::{Word, NewWord};

pub mod schema;
pub mod models;

type Counter = HashMap<String, i32>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn save_word<'a>(conn: &PgConnection, word: &'a str, counter: i32) -> Word {
    use schema::words;

    let new_word = NewWord {
        word,
        counter,
    };

    diesel::insert_into(words::table)
        .values(&new_word)
        .get_result(conn)
        .expect("Error saving new post")
}

fn counter_words (text: &str) -> Counter {
    let mut counter: Counter = HashMap::new();

    text.split(" ").for_each(| word | {
        if counter.contains_key(word) {
            *counter.get_mut(word).unwrap() += 1;
        } else {
            counter.insert(word.to_string(), 1);
        }
    });

    counter
}

fn main() {
    let mut res = reqwest::get("https://potter1.bib.bz/glava-1-malchik-kotoryy-vyzhil")
        .expect("Unable to connect");

    let conn = establish_connection();

    let content = res.text().expect("Unable to extract response");
    let fragment = Html::parse_fragment(&content);
    let selector = Selector::parse("#main p").unwrap();

    let mut text = String::from("");

    for element in fragment.select(&selector) {
        for text_node in element.text() {
            text.push_str(text_node);
        }
    }

    let counted_words = counter_words(&text);

    for (word, counter) in counted_words {
        save_word(&conn, &word, counter);
    }
}