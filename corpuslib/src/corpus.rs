use std::collections::HashMap;

extern crate rand;

struct Corpus {
    corpus: Vec<usize>,
    suffix: Vec<usize>,
}

impl Corpus {
    fn new(tokens: Vec<String>) -> Corpus {
        // Allocate corpus and suffix.
        let n = tokens.len();
        let mut corpus: Vec<usize> = vec![0; n];
        // Assign integers to corpus.
        let mut map = HashMap::new();
        let mut i:usize = 0;
        for token in tokens.iter() {
            match map.get(token) {
                Some(&v) => { corpus.push(v) },
                _ => { map.insert(token, i); corpus.push(i); i += 1}
            }
        }
        // Set suffix array.
        let mut suffix: Vec<usize> = vec![0; n];
        for i in 0..suffix.len() {
            suffix[i] = i
        }

        {
            let sortby_closure = |a: &usize, b: &usize| {
                corpus[*a].cmp(&corpus[*b])
            };
            suffix.sort_by(sortby_closure);
        }
        // Return.
        Corpus { corpus: corpus, suffix: suffix }
    }
}

#[test]
fn check_suffix_ordering() {
    // make strings.
    let vocab = 100;
    let n = 10000;
    let mut tokens: Vec<String> = Vec::with_capacity(n);
    for _ in 0..n {
        let x = rand::random::<usize>() % vocab;
        tokens.push(format!("{}", x));
    }
    //
    let c = Corpus::new(tokens);
    //
    for i in 0..(c.suffix.len()-1) {
        let w1 = c.corpus[c.suffix[i]];
        let w2 = c.corpus[c.suffix[i + 1]];
        if w1 > w2 {
            assert!(false);
        }
    }

}
