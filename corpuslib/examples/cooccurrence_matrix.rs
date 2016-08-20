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
    let mut f: HashMap<String, usize> = HashMap::new();
    for line in corpuslib::stream::LineStreamer::new(&DIRECTORY) {
        for word in line.split_whitespace() {
            let freq = f.entry(word.to_string()).or_insert(0);
            *freq += 1;
        }
    }
    let t2 = precise_time_ns();
    println!("Frequencies of {} word types computed ({} ns).", f.len(), t2 - t1);

    // Make a set of words which occurred 10 or more times.
    let t1 = precise_time_ns();
    let mut w: HashSet<String> = HashSet::new();
    for (word, freq) in &f {
        if *freq >= MIN_FREQ { w.insert(word.to_string()); }
    }
    let t2 = precise_time_ns();
    println!("{} word types with frequency >= {} retained in word set ({} ns)", w.len(), MIN_FREQ, t2 - t1);

    // Count co-occurrences.
    let t1 = precise_time_ns();
    let mut cooc_counter = corpuslib::coocs::CoocCounter::new(NB, NF);
    for line in corpuslib::stream::LineStreamer::new(&DIRECTORY) {
        for mut word in line.split_whitespace() {
            if !w.contains(word) { word = "<UNKNOWN>"; }
            cooc_counter.update(word);
        }
    }

    let freqs = cooc_counter.freqs();
    let mut ks: Vec<(String, String)> = Vec::new();
    for key in freqs.keys() { ks.push(key.clone()); }
    ks.sort();
    for k in &ks {
        println!("{:?} -> {:?}", k, freqs.get(k));
    }


    //println!("{:?}", cooc_counter.freqs());
    let t2 = precise_time_ns();
    println!("Foo. ({} ns)", t2 - t1);
}
