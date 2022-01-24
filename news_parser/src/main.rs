mod theme;

use std::error::Error;
use colour::{dark_magenta, dark_green, yellow};
use dotenv::dotenv;

use newsapi::{Category, Country, Endpoint, NewsAPI, Article};
// use newsapi::{Articles, get_articles};


fn draw_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");

    for article in articles {
        theme.print_text(&format!("`{}`", article.get_title()));
        theme.print_text(&format!("> *{}*", article.get_url()));
        theme.print_text("---")
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    dotenv();
    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Us);

    let newsapi_response = newsapi.fetch()?;

    draw_articles(&newsapi_response.articles());

    Ok(())
}

