mod fetcher;
mod scrapper;

const WORDS: [&str; 4] = ["hello", "This", "bye", "website"];

pub fn run() {
    for word in WORDS {
        std::thread::spawn(move || {
            println!("{}", word);
        });
    }
}
