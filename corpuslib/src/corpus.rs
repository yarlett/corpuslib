use std::cmp;
use std::collections::HashMap;

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

    pub fn search_linear(&self, seq: Vec<usize>) -> Vec<usize> {
        let mut pos: Vec<usize> = Vec::new();
        for p in 0..self.corpus.len() {
            if seq_ordering_n(&self.corpus[p..], &seq[..], seq.len()) == cmp::Ordering::Equal {
                pos.push(p)
            }
        }
        pos
    }
}

pub fn seq_ordering_n(seq1: &[usize], seq2: &[usize], n: usize) -> cmp::Ordering {
    let (n1, n2) = (seq1.len(), seq2.len());
    let mut look_n = cmp::min(n1, n2);
    if n < look_n {
        look_n = n;
    }
    // Make comparisons.
    for pos in 0..look_n {
        if seq1[pos] < seq2[pos] {
            return cmp::Ordering::Less;
        }
        else if seq2[pos] < seq1[pos] {
            return cmp::Ordering::Greater;
        }
    }
    // All assigned elements are the same, so make comparison based on length (shorter is lesser).
    if look_n == n {
        return cmp::Ordering::Equal;
    } else if n1 < n2 {
        return cmp::Ordering::Less;
    }
    else {
        return cmp::Ordering::Greater;
    }
}

pub fn seq_ordering(seq1: &[usize], seq2: &[usize]) -> cmp::Ordering {
    let (n1, n2) = (seq1.len(), seq2.len());
    seq_ordering_n(seq1, seq2, cmp::max(n1, n2) + 1)
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use std::cmp;
    use super::*;

    fn random_corpus(ntypes: usize, ntokens: usize) -> Corpus {
        // Generate a corpus of strings.
        let mut tokens: Vec<String> = Vec::with_capacity(ntokens);
        for _ in 0..ntokens {
            let token = rand::random::<usize>() % ntypes;
            tokens.push(format!("{}", token));
        }
        // Create the corpus.
        let c = Corpus::new(tokens);
        c
    }

    #[test]
    fn check_corpus() {
        // Generate random corpus.
        let (ntypes, ntokens) = (100, 10000);
        let c = random_corpus(ntypes, ntokens);
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

    #[test]
    fn check_search_linear() {
        // Generate random corpus.
        let (ntypes, ntokens) = (10, 10000);
        let c = random_corpus(ntypes, ntokens);
        //
        let mut seq = Vec::new();
        seq.extend(c.corpus[0..3].iter());
        let cpos = c.search_linear(seq);
        println!("{:?}", cpos);
    }
}
