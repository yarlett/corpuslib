extern crate corpuslib;
extern crate time;


use std::collections::{HashMap, HashSet};
use time::precise_time_ns;


// Directory containing a number of corpus text files to be crawled (files can be nested).
const DIRECTORY: &'static str = "/Users/yarlett/Desktop/1-billion-word-language-modeling-benchmark-r13output/training-monolingual.tokenized.shuffled/news.en-00001-of-00100";

const MIN_FREQ: usize = 10;

const NB: usize = 4;
const NF: usize = 4;


fn main() {
    // Count frequencies of words.
    let t1 = precise_time_ns();
    let mut freqs: HashMap<String, usize> = HashMap::new();
    for line in corpuslib::stream::LineStreamer::new(&DIRECTORY) {
        for word in line.split_whitespace() {
            let freq = freqs.entry(word.to_string()).or_insert(0);
            *freq += 1;
        }
    }
    let t2 = precise_time_ns();
    println!("Frequencies of {} word types computed ({} ns).", freqs.len(), t2 - t1);

    // Make a set of words which occurred 10 or more times.
    let t1 = precise_time_ns();
    let mut wordset: HashSet<String> = HashSet::new();
    for (word, freq) in &freqs {
        if *freq >= MIN_FREQ { wordset.insert(word.to_string()); }
    }
    let t2 = precise_time_ns();
    println!("{} word types with frequency >= {} retained in word set ({} ns)", wordset.len(), MIN_FREQ, t2 - t1);

    // Count co-occurrences and write to CSV.
    let t1 = precise_time_ns();
    let mut cooc_counter = corpuslib::coocs::CoocCounter::new(NB, NF);
    for line in corpuslib::stream::LineStreamer::new(&DIRECTORY) {
        for mut word in line.split_whitespace() {
            if !wordset.contains(word) { word = "<UNKNOWN>" }
            cooc_counter.update(&word.to_string());
        }
    }
    // cooc_counter.to_csv("coocs.csv");
    // let t2 = precise_time_ns();
    //println!("{:} distinct co-occurrences counted ({} ns).", cooc_counter.freqs().len(), t2 - t1);
}
