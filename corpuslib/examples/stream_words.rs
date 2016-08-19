extern crate corpuslib;


fn main() {
    // List of files.
    let directory = "/Users/yarlett/Desktop/1-billion-word-language-modeling-benchmark-r13output/training-monolingual.tokenized.shuffled";

    // Create line streamer.
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
