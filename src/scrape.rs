use scraper::{Html, Selector};

const BASE_URL: &str = "https://filmarks.com";

fn decade_links() -> Vec<String> {
    let list_year_body = reqwest::get("https://filmarks.com/list/year").unwrap().text().unwrap();
    let document = Html::parse_document(&list_year_body);
    let selector = Selector::parse(".c-search-panel__head > a").unwrap();

    document.select(&selector).map(|elem| {
        let path = elem.value().attr("href").unwrap();
        format!("{}{}", BASE_URL, path)
    }).collect()
}

fn scrape_pages(url: &str) -> Vec<String> {
    let body = reqwest::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a.c-pagination__last").unwrap();

    let page_elem = document.select(&selector).next();

    match page_elem {
        Some(s) => {
            // error
            let last_path = s.value().attr("href").unwrap();

            (1..=page_number).flat_map(|i| {
                let page_url = format!("{}?page={}", url, i);
                list_titles(&page_url)
            }).collect()
        },
        None => {
            list_titles(url)
        }
    }
}

fn list_titles(url: &str) -> Vec<String> {
    let list_year_body = reqwest::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&list_year_body);
    let selector = Selector::parse("h3.p-movie-cassette__title").unwrap();

    document.select(&selector).map(|elem| {
        elem.text().next().unwrap().to_string()
    }).collect()
}

fn main() {
    // let links = scrape_pages("https://filmarks.com/list/year/1890s/1899");
    let links = scrape_pages("https://filmarks.com/list/year/1910s/1912");
    println!("{:?}", links);
}