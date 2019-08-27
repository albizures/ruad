extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};
use std::collections::HashMap;

type Counter = HashMap<String, u16>;

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
    let mut res = reqwest::get("https://www.wuxiaworld.com/novel/terror-infinity/ti-vol-1-chapter-1-1")
        .expect("Unable to connect");

    let content = res.text().expect("Unable to extract response");
    let fragment = Html::parse_fragment(&content);
    let selector = Selector::parse("#chapter-content p").unwrap();

    let mut text = String::from("");

    for element in fragment.select(&selector) {
        for text_node in element.text() {
            text.push_str(text_node);
        }
    }


    println!("{:?}", counter_words(&text));
}