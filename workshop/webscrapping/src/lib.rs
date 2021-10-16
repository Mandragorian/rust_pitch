mod fetcher;
mod scrapper;

const WORDS: [&str; 4] = ["hello", "This", "bye", "website"];

pub fn run() {
    let handles = WORDS.iter().map(|word| {
        std::thread::spawn(move || {
            println!("{}", word);
        })
    }).collect();


    for handle in handles {
        handle.join().unwrap()
    }
}
