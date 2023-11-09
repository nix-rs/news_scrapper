use select::document::Document;
use select::predicate::Name;
use reqwest;

fn main() {
    let url = "https://edition.cnn.com/";

    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}
