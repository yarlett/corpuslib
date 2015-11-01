use std::cmp;

use corpus::sequence::{sequence_compare, sequence_compare_n};
use corpus::stringmap::Stringmap;

pub struct Corpus {
    pub sequence: Vec<usize>,
    pub suffix: Vec<usize>,
    pub stringmap: Stringmap,
}

impl Corpus {
    pub fn new(strings: Vec<String>) -> Corpus {
        // Allocate sequence and suffix array.
        let mut sequence: Vec<usize> = Vec::with_capacity(strings.len());
        // Assign integers to corpus.
        let mut stringmap = Stringmap::new();
        for s in strings.iter() {
            match stringmap.get(s) {
                Some(&code_value) => {
                    sequence.push(code_value);
                },
                None => {
                    let code_value = stringmap.add(&s);
                    sequence.push(code_value);
                },
            }
        }
        // Set suffix array.
        let mut suffix: Vec<usize> = Vec::with_capacity(sequence.len());
        for i in 0..sequence.len() {
            suffix.push(i);
        }
        {
            let suffix_ordering = |a: &usize, b: &usize| {
                sequence_compare(&sequence[*a..], &sequence[*b..])
            };
            suffix.sort_by(suffix_ordering);
        }
        // Return.
        Corpus { sequence: sequence, suffix: suffix, stringmap: stringmap }
    }

    // Returns the left-most suffix pointer to a sequence using binary search.
    // Also returns a right-most bound for the sequence which can be used to constrain the maximum search.
    fn binary_search_left(&self, seq: &[usize], suffix_min: usize, suffix_max: usize) -> Result<(usize, usize), bool> {
        let n = seq.len();
        let mut smin = suffix_min;
        let mut smax = suffix_max;
        let mut right_bound = suffix_max;
        while smax > smin {
            let smid = (smin + smax) / 2;
            let cmp = sequence_compare_n(&self.sequence[self.suffix[smid]..], seq, &n);
            // Update the right bound.
            if cmp == cmp::Ordering::Greater && smid < right_bound {
                right_bound = smid;
            }
            // Update the search range.
            if cmp == cmp::Ordering::Less {
                smin = smid + 1;
            } else {
                smax = smid;
            }
        }
        if smax == smin && sequence_compare_n(&self.sequence[self.suffix[smin]..], seq, &n) == cmp::Ordering::Equal {
            return Ok((smin, right_bound))
        }
        return Err(false);
    }

    // Returns the right-most suffix pointer to a sequence using binary search.
    // Also returns a leftmost bound for the sequence which can be used to constrain the minimum search.
    fn binary_search_right(&self, seq: &[usize], suffix_min: usize, suffix_max: usize) -> Result<(usize, usize), bool> {
        let n = seq.len();
        let mut smin = suffix_min;
        let mut smax = suffix_max;
        let mut left_bound = suffix_min;
        while smax > smin {
            let smid = ((smin + smax) / 2) + 1;
            let cmp = sequence_compare_n(&self.sequence[self.suffix[smid]..], seq, &n);
            // Update the left bound.
            if cmp == cmp::Ordering::Less && smid > left_bound {
                left_bound = smid;
            }
            // Update the search range.
            if cmp == cmp::Ordering::Greater {
                smax = smid - 1;
            } else {
                smin = smid;
            }
        }
        if smax == smin && sequence_compare_n(&self.sequence[self.suffix[smin]..], seq, &n) == cmp::Ordering::Equal {
            return Ok((smin, left_bound))
        }
        return Err(false);
    }

    // Returns distinct ngrams in the corpus.
    pub fn ngrams(&self, n: usize) -> Vec<&[usize]> {
        let ngs: Vec<&[usize]> = Vec::new();

        ngs
    }

    // Returns range of suffix array that points to required sequence in corpus using linear search.
    pub fn search_linear(&self, seq: &[usize]) -> Result<(usize, usize), bool> {
        let n = seq.len();
        let mut found: bool = false;
        let mut suffix_lo = 0;
        let mut suffix_hi = 0;
        for suffix_pos in 0..self.suffix.len() {
            if sequence_compare_n(&self.sequence[self.suffix[suffix_pos]..], seq, &n) == cmp::Ordering::Equal {
                if !found || (suffix_pos < suffix_lo) {
                    found = true;
                    suffix_lo = suffix_pos;
                }
                if !found || (suffix_pos > suffix_hi) {
                    found = true;
                    suffix_hi = suffix_pos
                }
            }
        }
        if !found {
            Err(false)
        }
        else {
            Ok((suffix_lo, suffix_hi))
        }
    }

    // Returns range of suffix array that points to required sequence in corpus using binary search.
    pub fn search_binary(&self, seq: &[usize]) -> Result<(usize, usize), bool> {
        let n = seq.len();
        // Binary search to get initial search location.
        let search_by_suffix_probe = | suffix_value: &usize | {
            sequence_compare_n(&self.sequence[*suffix_value..], seq, &n)
        };
        let binary_search_result = self.suffix.binary_search_by(search_by_suffix_probe);
        // Act on binary search result.
        match binary_search_result {
            Err(_) => return Err(false),
            Ok(suffix_pos) => {
                let mut suffix_lo = suffix_pos;
                let mut suffix_hi = suffix_pos;
                // Search lower.
                let left_search = self.binary_search_left(seq, 0, suffix_pos);
                match left_search {
                    Ok(r) => { suffix_lo = r.0; },
                    _ => { },
                }
                // Search higher.
                let right_search = self.binary_search_right(seq, suffix_pos, &self.suffix.len() - 1);
                match right_search {
                    Ok(r) => { suffix_hi = r.0; },
                    _ => { },
                }
                // Return suffix range.
                return Ok((suffix_lo, suffix_hi));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use std::cmp;
    use super::*;

    use corpus::sequence;

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
    fn check_corpus_creation() {
        // Generate random corpus.
        let (ntypes, ntokens) = (100, 10000);
        let c = random_corpus(ntypes, ntokens);
        // Check sequence and suffix array are the same length.
        if c.sequence.len() != ntokens || c.suffix.len() != ntokens {
            assert!(false);
        }
    }

    #[test]
    fn check_suffix_ordering() {
        // Generate random corpus.
        let (ntypes, ntokens) = (100, 10000);
        let c = random_corpus(ntypes, ntokens);
        // Check the ordering of corpus suffixes.
        for i in 0..(c.suffix.len() - 1) {
            let seq1 = &c.sequence[c.suffix[i]..];
            let seq2 = &c.sequence[c.suffix[i + 1]..];
            assert!(sequence::sequence_compare(seq1, seq2) == cmp::Ordering::Less);
        }
    }

    #[test]
    fn check_search() {
        // Generate random corpus.
        let (ntypes, ntokens) = (10, 1000);
        let c = random_corpus(ntypes, ntokens);
        // Compare search results for sub-sequences to make sure they agree.
        for n in 1..4 {
            for seq_pos in 0..(c.sequence.len() - n) {
                let seq = &c.sequence[seq_pos..(seq_pos + n)];
                let r1 = c.search_linear(seq);
                let r2 = c.search_binary(seq);
                println!("Searching for {:?} (seq_pos={:}; n={:}): linear {:?}; binary {:?}", seq, seq_pos, n, r1, r2);
                assert!(r1 == r2);
            }
        }
    }
}
