use crate::fetcher::fetch;

struct Scrapper {
    url: String,
}

impl Scrapper {
    fn new(url: &str) -> Self {
        Scrapper {
            url: String::from(url),
        }
    }

    fn contains(&self, word: &str) -> bool {
        fetch(self.url.as_str()).unwrap().contains(word)
    }
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
    fn correctly_checks_for_contains() {
        let scrapper = Scrapper::new("https://motherfuckingwebsite.com/");

        assert!(scrapper.contains("motherfucking"));
        assert!(!scrapper.contains("motherfuckig"));
    }
}
