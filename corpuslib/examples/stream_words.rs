extern crate corpuslib;


fn main() {
    // Directory containing a number of corpus text files to be crawled (files can be nested).
    let directory = "/Users/yarlett/Desktop/data/one-billion/training-monolingual.tokenized.\
                     shuffled";

    // Create line streamer and stream and count words in files.
    let line_streamer = corpuslib::stream::LineStreamer::new(&directory);
    let mut n = 0;
    for line in line_streamer {
        for word in line.split_whitespace() {
            println!("{}", word);
            n += 1;
        }
    }
    println!("{} words counted.", n);
}
