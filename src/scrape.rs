use scraper::{Html, Selector};
use regex::Regex;
use std::{thread, time};

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

fn scrape_pages(url: &str) {
    let body = reqwest::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a.c-pagination__last").unwrap();

    let page_elem = document.select(&selector).next();

    match page_elem {
        Some(s) => {
            let last_path = s.value().attr("href").unwrap();
            let re = Regex::new(r"\?page=(\d+)").unwrap();
            let caps = re.captures(last_path).unwrap();
            let page_number: i32 = caps.get(1).unwrap().as_str().parse().unwrap();

            (1..=page_number).for_each(|i| {
                thread::sleep(time::Duration::from_millis(500));
                let page_url = format!("{}?page={}", url, i);
                print_titles(&page_url);
            });
        },
        None => {
            print_titles(&url);
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

fn print_titles(url: &str) {
    let list_year_body = reqwest::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&list_year_body);
    let selector = Selector::parse("h3.p-movie-cassette__title").unwrap();

    document.select(&selector).for_each(|elem| {
        println!("{}", elem.text().next().unwrap());
    });
}

fn main() {
    let links = decade_links();
    links.iter().for_each(|link| {
        println!("{}", link);
        scrape_pages(link);
    })
    // scrape_pages("https://filmarks.com/list/year/1940s");
    // scrape_pages("https://filmarks.com/list/year/1910s/1912");
    // println!("{:?}", links);
}