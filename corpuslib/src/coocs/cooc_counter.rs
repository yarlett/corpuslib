extern crate csv;


use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;


pub struct CoocCounter<> {
    freqs:      HashMap<(Rc<String>, Rc<String>), usize>,
    num_b:      usize,
    num_f:      usize,
    vocabulary: HashSet<Rc<String>>,
    window:     Vec<Option<Rc<String>>>,
}


fn window_initial(num_b: usize, num_f: usize) -> Vec<Option<Rc<String>>> {
    vec![None; num_b + num_f + 1]
}


impl CoocCounter {
    pub fn new(num_b: usize, num_f: usize) -> CoocCounter {
        CoocCounter{
            freqs: HashMap::new(),
            num_b: num_b,
            num_f: num_f,
            vocabulary: HashSet::new(),
            window: window_initial(num_b, num_f),
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
                for cooc in self.freqs.keys() { coocs.push(cooc); }
                coocs.sort();
                // Write sorted co-occurrence frequencies to file.
                for cooc in &coocs {
                    let _ = w.encode((&cooc.0, &cooc.1, self.freqs.get(cooc)));
                }
            },
            _ => {},
        }
    }

    pub fn update(&mut self, word: &str) {
        // Remove left-most item and add new item to right of window.
        self.window.remove(0);
        let word_inner = Rc::new(word.to_string());
        self.vocabulary.insert(word_inner.clone());
        self.window.push(Some(word_inner.clone()));
        // Update co-occurrence frequencies.
        match self.window[self.num_b] {
            Some(ref target) => {
                // Backward window.
                for b in 0..self.num_b {
                    match self.window[b] {
                        Some(ref context) => {
                            let cooc = (target.clone(), context.clone());
                            let freq = self.freqs.entry(cooc).or_insert(0);
                            *freq += 1;
                        },
                        _ => {},
                    };
                };
                // Forward window.
                for f in 0..self.num_f {
                    match self.window[self.num_b + 1 + f] {
                        Some(ref context) => {
                            let cooc = (target.clone(), context.clone());
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

    pub fn window(&self) -> &Vec<Option<Rc<String>>> {
        &self.window
    }
}
