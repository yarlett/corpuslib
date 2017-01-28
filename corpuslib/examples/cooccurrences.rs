extern crate corpuslib;
extern crate time;


//use std::collections::{HashMap, HashSet};
use corpuslib::vocabulary::Vocabulary;
use time::precise_time_s;


// Directory containing a number of corpus text files to be crawled (files can be nested).
const DIRECTORY: &'static str = "/Users/yarlett/Desktop/data/one-billion/\
    training-monolingual.tokenized.shuffled";

const MIN_FREQ: usize = 50;

const NB: usize = 10;
const NF: usize = 10;


fn main() {
    // Build vocabulary of words.
    let t1 = precise_time_s();
    let mut vocab = Vocabulary::new();
    for line in corpuslib::stream::LineStreamer::new(&DIRECTORY) {
        for word in line.split_whitespace() {
            vocab.add(word);
        }
    }
    let t2 = precise_time_s();
    println!("Frequencies of {} word types computed ({} s).",
             vocab.len(),
             t2 - t1);

    // Vocabulary is words that have occurred MIN_FREQ or more times.
    let t1 = precise_time_s();
    vocab.filter_by_minimum_frequency(MIN_FREQ);
    let t2 = precise_time_s();
    println!("{} word types with frequency >= {} retained in word set ({} s)",
             vocab.len(),
             MIN_FREQ,
             t2 - t1);

    // Count co-occurrences and write to CSV.
    let t1 = precise_time_s();
    let mut cooc_counter = corpuslib::coocs::CoocCounter::new(NB, NF);
    for line in corpuslib::stream::LineStreamer::new(&DIRECTORY) {
        for mut word in line.split_whitespace() {
            if !vocab.contains(word) {
                word = "<UNKNOWN>"
            }
            cooc_counter.register(&word);
            let window = cooc_counter.window();
            println!("{}: {:?}", window.len(), window);
        }
    }
    // cooc_counter.to_csv("coocs.csv");
    let t2 = precise_time_s();
    println!("{:} distinct co-occurrences counted ({} s).",
             cooc_counter.freqs().len(),
             t2 - t1);
}
