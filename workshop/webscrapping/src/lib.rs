struct Scrapper;

impl Scrapper {
    fn new(url: &str) -> Self {
        return Scrapper;
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
    }
}
