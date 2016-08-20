use std::collections::HashMap;


pub struct CoocCounter {
    num_b: usize,
    num_f: usize,
    window: Vec<Option<String>>,
    freqs: HashMap<(String, String), usize>,
}


fn window_initial(num_b: usize, num_f: usize) -> Vec<Option<String>> {
    vec![None; num_b + num_f + 1]
}


impl CoocCounter {
    pub fn new(num_b: usize, num_f: usize) -> CoocCounter {
        CoocCounter{
            num_b: num_b,
            num_f: num_f,
            window: window_initial(num_b, num_f),
            freqs: HashMap::new(),
        }
    }

    pub fn freqs(&self) -> &HashMap<(String, String), usize> {
        &self.freqs
    }

    pub fn update(&mut self, word: &str) {
        // Slide items in window one position to the left.
        let last_index = self.window.len() - 1;
        for i in 0..last_index {
            self.window[i] = self.window[i + 1].clone();
        }
        // Add new word at right end of window.
        self.window[last_index] = Some(word.to_string());
        // Update frequencies.
        match self.window[self.num_b] {
            Some(ref target) => {
                // Backward window.
                for b in 0..self.num_b {
                    match self.window[b] {
                        Some(ref context) => {
                            let cooc = (target.to_string(), context.to_string());
                            let freq = self.freqs.entry(cooc).or_insert(0);
                            *freq += 1;
                        },
                        _ => {},
                    };
                };
                // Forward window.
                for f in 0..self.num_f {
                    match self.window[self.num_b + 1 + f] {
                        Some(ref context) => {
                            let cooc = (target.to_string(), context.to_string());
                            let freq = self.freqs.entry(cooc).or_insert(0);
                            *freq += 1;
                        },
                        _ => {},
                    };
                };
            },
            _ => {},
        }
    }

    pub fn window(&self) -> &Vec<Option<String>> {
        &self.window
    }
}
