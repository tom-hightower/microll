use scraper::{Html, Selector};

pub fn parse_html(html: &String) -> scraper::Html {
    let document = Html::parse_document(html);
    return document;
}

pub fn get_elements(document: scraper::Html, key: &str) -> Vec<String> {
    let selector = Selector::parse(key).unwrap();
    let mut elements = Vec::new();
    for item in document.select(&selector) {
        elements.push(item.text().collect::<String>());
    }
    return elements;
}
