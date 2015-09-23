struct Corpus {
    corpus: i32,
    suffix: i32,
}

impl Corpus {
    fn new(corpus: i32, suffix: i32) -> Corpus {
        Corpus { corpus: corpus, suffix: suffix }
    }
}
