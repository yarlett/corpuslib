use std::collections::HashMap;


pub struct Vocabulary {
    pub frequencies: HashMap<String, usize>,
}


impl Vocabulary {
    pub fn new() -> Vocabulary {
        Vocabulary { frequencies: HashMap::new() }
    }

    pub fn add(&mut self, word: &str) {
        let freq = self.frequencies.entry(word.to_string()).or_insert(0);
        *freq += 1;
    }

    pub fn contains(&self, word: &str) -> bool {
        self.frequencies.contains_key(word)
    }

    pub fn filter_by_minimum_frequency(&mut self, min_freq: usize) {
        let removes: Vec<_> = self.frequencies
            .iter()
            .filter(|&(_, &freq)| freq < min_freq)
            .map(|(word, _)| word.clone())
            .collect();
        for r in removes {
            self.frequencies.remove(&r);
        }
    }

    pub fn len(&self) -> usize {
        self.frequencies.len()
    }
}
