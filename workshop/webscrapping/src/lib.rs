mod fetcher;
mod scrapper;

const WORDS: [&str; 4] = ["hello", "This", "bye", "website"];

pub fn run() {
    let mut v = Vec::new();

    let handles: Vec<_> = WORDS.iter().map(|word| {
        std::thread::spawn(move || {
            let scrapper = scrapper::Scrapper::new("https://motherfuckingwebsite.com");
            if scrapper.contains(word) {
                v.push(word);
            }
        })
    }).collect();


    for handle in handles {
        handle.join().unwrap()
    }

    println!("{:?}", v);
}
