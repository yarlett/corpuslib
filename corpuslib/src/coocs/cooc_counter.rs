extern crate csv;


use std::collections::{HashMap, HashSet};
use std::path::Path;


pub struct CoocCounter<'a> {
    freqs: HashMap<(&'a String, &'a String), usize>,
    num_b: usize,
    num_f: usize,
    vocabulary: HashSet<String>,
    window: Vec<Option<&'a String>>,
}


fn window_initial<'a>(num_b: usize, num_f: usize) -> Vec<Option<&'a String>> {
    vec![None; num_b + num_f + 1]
}


impl<'a> CoocCounter<'a> {
    pub fn new(num_b: usize, num_f: usize) -> CoocCounter<'a> {
        CoocCounter{
            freqs: HashMap::new(),
            num_b: num_b,
            num_f: num_f,
            vocabulary: HashSet::new(),
            window: window_initial(num_b, num_f),
        }
    }

    pub fn freqs(&self) -> &HashMap<(&String, &String), usize> {
        &self.freqs
    }

    pub fn to_csv(&self, filename: &str) {
        let path = Path::new(filename);
        let writer = csv::Writer::from_file(&path);
        match writer {
            Ok(mut w) => {
                for (cooc, freq) in &self.freqs {
                    let _ = w.encode((cooc.0, cooc.1, freq));
                }
            },
            _ => {},
        }

    }

    pub fn update(&mut self, word: &str) {
        // Slide items in window one position to the left.
        let last_index = self.window.len() - 1;
        for i in 0..last_index {
            self.window[i] = self.window[i + 1];
        }
        // Add new word at right end of window.
        let w = word.to_string();
        self.vocabulary.insert(w);
        self.window[last_index] = self.vocabulary.get(&w);
        // Update co-occurrence frequencies.
        match self.window[self.num_b] {
            Some(target) => {
                // Backward window.
                for b in 0..self.num_b {
                    match self.window[b] {
                        Some(context) => {
                            let cooc = (target, context);
                            let freq = self.freqs.entry(cooc).or_insert(0);
                            *freq += 1;
                        },
                        _ => {},
                    };
                };
                // Forward window.
                for f in 0..self.num_f {
                    match self.window[self.num_b + 1 + f] {
                        Some(context) => {
                            let cooc = (target, context);
                            let freq = self.freqs.entry(cooc).or_insert(0);
                            *freq += 1;
                        },
                        _ => {},
                    };
                };
            },
            _ => {},
        }
    }

    pub fn window(&self) -> &Vec<Option<&String>> {
        &self.window
    }

}