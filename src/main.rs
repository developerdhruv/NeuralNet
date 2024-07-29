use std::collections::HashMap;
use std::f64;
use std::cmp::Ordering;

fn tokenize(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

fn count_words<'a>(tokens: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut counter = HashMap::new();
    for &token in tokens {
        *counter.entry(token).or_insert(0) += 1;
    }
    counter
}

fn dot_product(counter1: &HashMap<&str, usize>, counter2: &HashMap<&str, usize>) -> usize {
    counter1.iter().fold(0, |acc, (key, val)| {
        acc + val * counter2.get(key).unwrap_or(&0)
    })
}

fn magnitude(counter: &HashMap<&str, usize>) -> f64 {
    counter.values().fold(0.0, |acc, &val| acc + (val * val) as f64).sqrt()
}

fn cosine_similarity(query: &str, document: &str) -> f64 {
    let query_tokens = tokenize(query);
    let document_tokens = tokenize(document);
    let query_counter = count_words(&query_tokens);
    let document_counter = count_words(&document_tokens);

    let dot = dot_product(&query_counter, &document_counter);
    let magnitude_query = magnitude(&query_counter);
    let magnitude_document = magnitude(&document_counter);

    if magnitude_query > 0.0 && magnitude_document > 0.0 {
        (dot as f64) / (magnitude_query * magnitude_document)
    } else {
        0.0
    }
}

fn return_response<'a>(query: &str, corpus: &'a [&str]) -> &'a str {
    corpus.iter()
        .max_by(|&doc1, &doc2| {
            cosine_similarity(query, doc1)
                .partial_cmp(&cosine_similarity(query, doc2))
                .unwrap_or(Ordering::Equal)
        })
        .unwrap_or(&"")
}

fn main() {
    let data_bunch = [
        "Python is a language used for ai and machine learning",
        "Javascripta is a all rounder language can do both backend and frontend",
        "React-JS concference was held in bengaluru",
        "Go lang is the fastest language and can do multiple things",
        "Rust is evolving very rapidly because it is super fast",
        "Explore a new tech by reading tech news",
        "You can go to Geek for geeks for dsa classes",
        "Offline coding institutes like coding blocks and gfg are doing good job."
    ];

    let user_input = "my love is to build ai and machine learning application";
    let relevant_document = return_response(user_input, &data_bunch);
    println!("Relevant document: {}", relevant_document);
}
