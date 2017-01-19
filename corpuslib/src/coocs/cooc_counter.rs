extern crate csv;


use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;


pub struct CoocCounter {
    events: HashSet<Rc<String>>,
    freqs: HashMap<(Rc<String>, Rc<String>), usize>,
    num_b: usize,
    num_f: usize,
    vocabulary: HashSet<Rc<String>>,
    window: Vec<Rc<String>>,
    window_size: usize,
}


impl CoocCounter {
    pub fn new(num_b: usize, num_f: usize) -> CoocCounter {
        CoocCounter {
            events: HashSet::new(),
            freqs: HashMap::new(),
            num_b: num_b,
            num_f: num_f,
            vocabulary: HashSet::new(),
            window: Vec::new(),
            window_size: num_b + 1 + num_f,
        }
    }

    pub fn freqs(&self) -> &HashMap<(Rc<String>, Rc<String>), usize> {
        &self.freqs
    }

    pub fn to_csv(&self, filename: &str) {
        let path = Path::new(filename);
        let writer = csv::Writer::from_file(&path);
        match writer {
            Ok(mut w) => {
                // Get sorted cooccurrences.
                let mut coocs = Vec::new();
                for cooc in self.freqs.keys() {
                    coocs.push(cooc);
                }
                coocs.sort();
                // Write sorted co-occurrence frequencies to file.
                for cooc in &coocs {
                    let _ = w.encode((&cooc.0, &cooc.1, self.freqs.get(cooc)));
                }
            }
            _ => {}
        }
    }

    pub fn register(&mut self, word: &str) {
        // Update window (remove left-most item; insert new item at right-most point).
        if self.window.len() >= self.window_size {
            self.window.remove(0);
        }
        let word_inner = Rc::new(word.to_string());
        self.vocabulary.insert(word_inner.clone());
        self.window.push(word_inner.clone());
        // Update co-occurrences if window is required length.
        if self.window.len() == self.window_size {
            let context = self.window[self.num_b].clone();
            // Get set of events observed in current context.
            self.events.clear();
            for b in 0..self.num_b {
                self.events.insert(self.window[b].clone());
            }
            for f in (self.num_b + 1)..(self.num_b + 1 + self.num_f) {
                self.events.insert(self.window[f].clone());
            }
            // Update co-occurrence counts of events that occurred.
            for e in &self.events {
                let cooc = (context.clone(), e.clone());
                let freq = self.freqs.entry(cooc).or_insert(0);
                *freq += 1;
            }
        }
    }

    pub fn window(&self) -> &Vec<Rc<String>> {
        &self.window
    }
}
