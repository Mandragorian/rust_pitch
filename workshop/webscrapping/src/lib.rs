mod fetcher;
mod scrapper;

const WORDS: [&str; 4] = ["hello", "This", "bye", "website"];

pub fn run() {
    let v: Vec<&str> = Vec::new();
    let ref_v = &v;

    let handles: Vec<_> = WORDS.iter().map(|word| {
        std::thread::spawn(move || {
            let scrapper = scrapper::Scrapper::new("https://motherfuckingwebsite.com");
            if scrapper.contains(word) {
                println!("{:?}", ref_v);
            }
        })
    }).collect();


    for handle in handles {
        handle.join().unwrap()
    }

    println!("{:?}", v);
}
