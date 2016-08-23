use std::collections::HashMap;


struct VocabularyItem {
    string: String,
    id: usize,
}


pub struct Vocabulary<'a> {
    bystring: HashMap<String, &'a VocabularyItem>,
    bycode: HashMap<usize, &'a VocabularyItem>,
    idctr: usize,
    vis: Vec<VocabularyItem>,
}


impl<'a> Vocabulary<'a> {
    pub fn code_for(&self, string: &str) -> usize {
        match self.bystring.get(string) {
            Some(vi) => vi.id,
            None => 0,
        }
    }

    pub fn new() -> Vocabulary<'a> {
        Vocabulary {
            bystring: HashMap::new(),
            bycode: HashMap::new(),
            idctr: 0,
            vis: Vec::new(),
        }
    }

    fn next_id(&mut self) -> usize {
        self.idctr += 1;
        self.idctr
    }

    pub fn register(&mut self, string: &str) {
        if !self.bystring.contains_key(string) {
            let vi = VocabularyItem{ string: string.to_string(), id: self.next_id() };
            self.vis.push(vi);
            self.bystring.insert(string.to_string(), &self.vis[self.vis.len() - 1]);
            self.bycode.insert(vi.id, &vi);
        }
    }

    pub fn string_for(&self, code: usize) -> String {
        match self.bycode.get(&code) {
            Some(vi) => vi.string.to_string(),
            None => "<UNKNOWN>".to_string(),
        }
    }
}
