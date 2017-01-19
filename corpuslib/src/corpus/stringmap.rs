use std::collections::HashMap;

pub struct Stringmap {
    pub code_ctr: usize,
    pub map: HashMap<String, usize>,
}

impl Stringmap {
    pub fn new() -> Stringmap {
        let code_ctr = 0;
        let map = HashMap::new();
        Stringmap {
            code_ctr: code_ctr,
            map: map,
        }
    }

    pub fn add(&mut self, s: &String) -> usize {
        let return_code: usize;
        match self.get(s) {
            Some(&code_value) => return_code = code_value,
            None => {
                self.map.insert(s.to_string(), self.code_ctr);
                return_code = self.code_ctr;
                self.code_ctr += 1;
            }
        }
        return_code
    }

    pub fn contains(&self, s: &String) -> bool {
        self.map.contains_key(s)
    }

    pub fn get(&self, s: &String) -> Option<&usize> {
        self.map.get(s)
    }
}
