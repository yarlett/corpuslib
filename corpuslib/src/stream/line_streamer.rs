use std::fs::{File};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use walkdir::{DirEntry, WalkDir};


fn get_directory_entries(directory: &str) -> Vec<DirEntry> {
    /*
    Walks a directory structure recursively and returns a list of directory entries.
    */
    let mut entries = Vec::new();
    for entry in WalkDir::new(&directory) {
        entries.push(entry.unwrap());
    }
    entries
}


pub struct LineStreamer {
    entries: Vec<DirEntry>,
    entries_ctr: usize,
    buffer: Result<BufReader<File>, Error>,
    line: Result<String, Error>,
    line_function: Option<fn(String) -> Vec<String>>,
    line_vector: Result<Vec<String>>,
    line_vector_ctr: usize,
}


impl LineStreamer {
    pub fn new(directory: &str, line_function: Option<fn(String) -> String>) -> LineStreamer {
        let entries = get_directory_entries(directory);
        let mut ls = LineStreamer{
            entries: entries,
            entries_ctr: 0,
            buffer: Err(Error::new(ErrorKind::Other, "No valid buffer.")),
            line: Err(Error::new(ErrorKind::Other, "No valid line.")),
            line_function: None,
            line_vector:  Err(Error::new(ErrorKind::Other, "No valid line vector.")),
            line_vector_ctr: 0,
        };
        ls.line_next();
        ls
    }

    fn buffer_next(&mut self) {
        /*
        Advances the internal buffer to the next valid one. If there isn't one then the buffer is set to an error.
        */
        self.buffer = Err(Error::new(ErrorKind::Other, "No valid buffer."));
        while self.buffer.is_err() && self.entries_ctr < self.entries.len() {
            let entry = &self.entries[self.entries_ctr];
            match File::open(entry.path()) {
                Ok(f) => { self.buffer = Ok(BufReader::new(f)); },
                Err(e) => self.buffer = Err(e),
            }
            self.entries_ctr += 1;
            // println!("{:?}", entry.path());
        }
    }

    fn line_next(&mut self) {
        /*
        Advances the internal line to the next valid line. If there isn't one then the line is set to an error.
        */
        // If the buffer is uninitialized then initialize it.
        if self.buffer.is_err() { self.buffer_next(); }
        // Try to read next line from the buffer.
        if self.line_next_inner() == 0 {
            self.buffer_next();
            self.line_next_inner();
        };
    }

    fn line_next_inner(&mut self) -> usize {
        self.line = Err(Error::new(ErrorKind::Other, "No valid line."));
        let mut bytes_read: usize = 0;
        match self.buffer {
            Ok(ref mut b) => {
                let mut line_string = String::new();
                match b.read_line(&mut line_string) {
                    Ok(n) => {
                        bytes_read = n;
                        self.line = Ok(line_string);
                    },
                    _ => {},
                };
            },
            _ => {},
        }
        bytes_read
    }

    fn line_vector_next(&mut self) -> String {
        // Initialize the line vector if it has not been set yet.
        if self.line_vector.is_err() { self.line_next(); }

        match self.line_function {
            Some(func) => {
                self.line_vector = Ok(vec![l.to_string()]);
                self.line_vector_ctr = 0;
            },
            None => {
                self.line_vector = Ok(vec![l.to_string()]);
                self.line_vector_ctr = 0;
            },
        }


    }
}


impl Iterator for LineStreamer {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        // Advance to the next item in the line vector.
        self.line_vector_next();

        // // Advance to the next line.
        // self.line_next();

        // Return the next string or None to end the iteration.
        match self.line {
            Ok(ref l) => {
                // let string = l.to_string();
                // Some(string)
            },
            _ => { None },
        }
    }
}
