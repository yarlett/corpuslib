use std::cmp;

use sequence::Sequence;
use stringmap::Stringmap;

pub struct Range {
    pub start: usize,
    pub end: usize,
}

pub struct Corpus {
    pub sequence: Sequence,
    pub suffix: Vec<usize>,
    pub stringmap: Stringmap,
}

impl Corpus {
    pub fn new(strings: Vec<String>) -> Corpus {
        // Allocate sequence and suffix array.
        let n = tokens.len();
        let mut sequence: Sequence = Sequence::new(); //Vec::with_capacity(n);
        // Assign integers to corpus.
        let mut stringmap = Stringmap::new();
        let mut i: usize = 0;
        for token in tokens.iter() {
            match stringmap.map.get(token) {
                Some(&v) => { corpus.push(v) },
                _ => { stringmap.map.insert(token.to_string(), i); corpus.push(i); i += 1}
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
        Corpus { corpus: corpus, suffix: suffix, stringmap: stringmap }
    }

    // Returns range of suffix array that points to required sequence in corpus.
    pub fn search_linear(&self, seq: Vec<usize>) -> Result<Range, usize> {
        let mut range = Range{ start: -1, end: -1};
        for p in 0..self.suffix.len() {
            if seq_ordering_n(&self.corpus[self.suffix[p]..], &seq[..], seq.len()) == cmp::Ordering::Equal {
                if (range.start == -1) || (p < range.start) {
                    range.start = p;
                }
                if (range.end == -1) || (p > range.end) {
                    range.end = p;
                }
            }
        }
        if range.start == -1 {
            Err(-1)
        }
        else {
            Ok(range)
        }
    }

    // pub fn search_binary(&self, seq: Vec<usize>) -> Result<Range, usize> {
    //     let foo = |probe: &usize| {
    //         seq_ordering_n(&seq[..], &self.corpus[probe..], seq.len())
    //     };
    //     self.suffix[..].binary_search_by(foo)
    // }
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
        match cpos {
            Ok(r) => println!("{:} {:}", r.start, r.end),
            _ => println!("fail"),
        }
    }
}
