use reqwest::blocking::get;

pub fn fetch(url: &str) -> Result<String, String> {
    let result = get(url);
    let text_result = match result {
        Ok(response) => response.text(),
        Err(err) => return Err(format!("fetching website failed: {}", err)),
    };

    match text_result {
        Ok(body) => Ok(body),
        Err(_) => return Err (String::from("response has no body")),
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
            Err(msg) => assert_eq!(msg, "fetching website failed: error sending request for url (https://koko.lala/): error trying to connect: dns error: failed to lookup address information: Name or service not known"),
        };
    }
}
