mod fetcher;
mod scrapper;

use std::rc::Rc;

const WORDS: [&str; 4] = ["hello", "This", "bye", "website"];

pub fn run() {
    let v: Rc<Vec<&str>> = Rc::new(Vec::new());

    let handles: Vec<_> = WORDS.iter().map(|word| {
        let v_cloned = Rc::clone(&v);
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
