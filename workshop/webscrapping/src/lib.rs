use reqwest::blocking::get;

struct Scrapper;

fn fetch(url: &str) -> Result<String, ()> {
    let result = get(url);
    let text_result = match result {
        Ok(response) => response.text(),
        Err(_) => return Err( () ),
    };

    match text_result {
        Ok(body) => Ok(body),
        Err(_) => return Err ( () ),
    }
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
        let text = fetch("https://motherfuckingwebsite.com/").unwrap();
        let mut lines = text.lines();

        assert_eq!(lines.next(), Some("<!DOCTYPE html>"));
        assert_eq!(lines.next(), Some("<html>"));
        assert_eq!(lines.next(), Some("<head>"));
    }

    #[test]
    fn handle_errors_when_fetching() {
        let text = match fetch("https://koko.lala") {
            Ok(_) => panic!("should not fetch non-existent website"),
            Err(_) => return,
        };
    }
}
