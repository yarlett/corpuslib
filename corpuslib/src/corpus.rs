use std::collections::HashMap;

struct Corpus {
    corpus: Vec<u64>,
    suffix: Vec<u64>,
}

impl Corpus {
    fn new(tokens: Vec<String>) -> Corpus {
        // Allocate corpus and suffix.
        let n = tokens.len();
        let mut corpus = vec![0; n];
        let mut suffix = vec![0; n];
        // Assign integers to corpus.
        let mut map = HashMap::new();
        let mut i:u64 = 0;
        for token in tokens.iter() {
            match map.get(token) {
                Some(&v) => { corpus.push(v) },
                _ => { map.insert(token, i); corpus.push(i); i += 1}
            }
        }
        // Return.
        Corpus { corpus: corpus, suffix: suffix }
    }
}
