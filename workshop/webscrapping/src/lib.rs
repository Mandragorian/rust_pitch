mod fetcher;
mod scrapper;

use std::sync::Arc;

const WORDS: [&str; 4] = ["hello", "This", "bye", "website"];

pub fn run() {
    let v: Arc<Vec<&str>> = Arc::new(Vec::new());

    let handles: Vec<_> = WORDS.iter().map(|word| {
        let v_cloned = Arc::clone(&v);
        std::thread::spawn(move || {
            let scrapper = scrapper::Scrapper::new("https://motherfuckingwebsite.com");
            if scrapper.contains(word) {
                println!("{:?}", v_cloned);
            }
        })
    }).collect();


    for handle in handles {
        handle.join().unwrap()
    }

    println!("{:?}", v);
}
