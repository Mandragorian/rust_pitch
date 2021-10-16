mod fetcher;
mod scrapper;

pub fn run() {
    for word in ["hello", "This", "bye", "website"] {
        std::thread::spawn();
    }
}
