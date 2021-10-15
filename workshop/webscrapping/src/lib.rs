struct Scrapper;

fn fetch(url: &str) -> String {
    String::new()
}

impl Scrapper {
    fn new(url: &str) -> Self {
        Scrapper
    }

    fn contains(&self, word: &str) -> bool {
        true
    }
}

pub fn run() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_api() {
        let scrapper = Scrapper::new("https://motherfuckingwebsite.com/");

        assert!(scrapper.contains("motherfucking"));
        assert!(scrapper.contains("motherfucking"));
    }

    #[test]
    fn fetch_a_website() {
        let text = fetch("https://motherfuckingwebsite.com/");
    }
}
