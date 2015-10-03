use std::collections::HashMap;

pub struct Stringmap {
    pub map: HashMap<String, usize>,
}

impl Stringmap {
    pub fn new() -> Stringmap {
        Stringmap{ map: HashMap::new() }
    }
}
