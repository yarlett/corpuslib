use std::cmp;
use std::collections::HashMap;

extern crate rand;

pub struct Corpus {
    pub corpus: Vec<usize>,
    pub suffix: Vec<usize>,
    pub wordmap: HashMap<String, usize>,
}

impl Corpus {
    pub fn new(tokens: Vec<String>) -> Corpus {
        // Allocate corpus and suffix.
        let n = tokens.len();
        let mut corpus: Vec<usize> = Vec::with_capacity(n);
        // Assign integers to corpus.
        let mut wordmap = HashMap::new();
        let mut i:usize = 0;
        for token in tokens.iter() {
            match wordmap.get(token) {
                Some(&v) => { corpus.push(v) },
                _ => { wordmap.insert(token.to_string(), i); corpus.push(i); i += 1}
            }
        }
        // Set suffix array.
        let mut suffix: Vec<usize> = Vec::with_capacity(corpus.len());
        for i in 0..corpus.len() {
            suffix.push(i);
        }
        {
            let suffix_ordering = |a: &usize, b: &usize| {
                seq_ordering(&corpus[*a..], &corpus[*b..])
            };
            suffix.sort_by(suffix_ordering);
        }
        // Return.
        Corpus { corpus: corpus, suffix: suffix, wordmap: wordmap }
    }
}

fn seq_ordering(seq1: &[usize], seq2: &[usize]) -> cmp::Ordering {
    let (n1, n2) = (seq1.len(), seq2.len());
    let n = cmp::min(n1, n2);
    // Make comparisons.
    for pos in 0..n {
        if seq1[pos] < seq2[pos] {
            return cmp::Ordering::Less;
        }
        else if seq2[pos] < seq1[pos] {
            return cmp::Ordering::Greater;
        }
    }
    // If comparisons fail to find a difference, go by length.
    if n1 == n2 {
        return cmp::Ordering::Equal;
    }
    else if n1 < n2 {
        return cmp::Ordering::Less;
    }
    else {
        return cmp::Ordering::Greater;
    }
}

#[test]
fn check_corpus() {
    let (ntypes, ntokens) = (100, 10000);
    // Generate a corpus of strings.
    let mut tokens: Vec<String> = Vec::with_capacity(ntokens);
    for _ in 0..ntokens {
        let token = rand::random::<usize>() % ntypes;
        tokens.push(format!("{}", token));
    }
    // Create the corpus.
    let c = Corpus::new(tokens);
    // Check corpus and suffix array are the same length.
    if c.corpus.len() != ntokens || c.suffix.len() != ntokens {
        assert!(false);
    }
    // Check the ordering of corpus suffixes.
    for i in 0..(c.suffix.len() - 1) {
        let seq1 = &c.corpus[c.suffix[i]..];
        let seq2 = &c.corpus[c.suffix[i + 1]..];
        let ord = seq_ordering(seq1, seq2);
        println!("{:?}", ord);
        assert!(ord != cmp::Ordering::Greater);
    }
}
