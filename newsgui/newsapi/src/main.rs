use newsapi::{Category, Country, Endpoint, NewsAPI, Article};
use std::error::Error;


fn draw_articles(articles: &Vec<Article>) {
    println!("# Top headlines\n\n");

    for article in articles {
        println!("`{}`", article.get_title());
        println!("> *{}*", article.get_url());
        println!("---")
    }
}

pub fn main () ->Result<(), Box<dyn Error>> {
    let api = "9ecdffd98f02404eaaf09707c9ac1662";
    let mut newsapi = NewsAPI::new(api);
    newsapi.endpoint(newsapi::Endpoint::TopHeadlines).country(newsapi::Country::Us);

    let newsapi_response = newsapi.fetch()?;

    draw_articles(&newsapi_response.articles());
    dbg!(newsapi_response);
    println!("Testing...");

    Ok(())
}