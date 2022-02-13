#[cfg(feature = "async")]
use reqwest::Method;
use serde::Deserialize;
use crate::error::NewsApiError;
use NewsApiError::{RequestFailed, ConvertingFailed, ParseFailed};
use url::Url;

mod error;

const BASE_URL: &str = "https://newsapi.org/v2";

#[derive(Deserialize, Debug)]
pub struct NewsAPIResponse {
    status: String,
    pub articles: Vec<Article>,
    code: Option<String>
}

impl NewsAPIResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

// #[derive(Deserialize, Debug)]
// pub struct Articles {
//     pub articles: Vec<Article>,
// }
//

#[derive(Deserialize, Debug)]
pub struct Article {
    title: String,
    url: String,
}

impl Article {
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}
//
// pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
//     let response = ureq::get(url).call().map_err(|e|
//         RequestFailed(e))?.into_string().map_err(|e|
//         ConvertingFailed(e))?;
//
//     let articles: Articles = serde_json::from_str(&response).map_err(|e|
//         ParseFailed(e))?;
//
//     Ok(articles)
// }

pub enum Endpoint {
    TopHeadlines,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string()
        }
    }
}

pub enum Country {
    Us,
    Ru,
    De,
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::Ru => "ru".to_string(),
            Self::De => "de".to_string(),
            Self::Us => "us".to_string(),
        }
    }
}

pub enum Category {
    Technology,
    Business,
    Science,
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Self::Technology => "technology".to_string(),
            Self::Business => "business".to_string(),
            Self::Science => "science".to_string()
        }
    }
}

pub struct NewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
    category: Category,
}

impl NewsAPI {
    pub fn new(api_key: &str) -> NewsAPI {
        NewsAPI {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::Ru,
            category: Category::Science,
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsAPI {
        self.endpoint = endpoint;

        self
    }

    pub fn country(&mut self, country: Country) -> &mut NewsAPI {
        self.country = country;

        self
    }

    pub fn category(&mut self, category: Category) -> &mut NewsAPI {
        self.category = category;

        self
    }

    fn prepare_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut()
            .unwrap()
            .push(&self.endpoint.to_string());

        // let country = self.country.to_string();
        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));


        // let category = format!("category={}", self.category.to_string());
        // url.set_query(Some(&category));

        // url.query_pairs_mut()
        //     .append_pair("country", &self.country.to_string())
        //     .append_pair("category", & self.category.to_string());

        // println!("{}", url);
        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response: NewsAPIResponse = req.call()?.into_json()?;

        return match response.status.as_str() {
            "ok" => Ok(response),
            _ => Err(map_response_err(response.code))
        }

    }

    #[cfg(feature = "async")]
    pub async fn async_fetch(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let client = reqwest::Client::new();
        let request = client
            .request(Method::GET, url)
            .header("Authorization", &self.api_key)
            .build()
            .map_err(|e|  NewsApiError::AsyncRequestFailed(e))?;

        let response: NewsAPIRespons = client
            .execute(request).await?
            .json().await
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        return match response.status.as_str() {
            "ok" => Ok(response),
            _ => Err(map_response_err(response.code)),
        }
    }
}

fn map_response_err(code: Option<String>) -> NewsApiError {
        if let Some(code) = code {
            match code.as_str() {
                "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled"),
                _ => NewsApiError::BadRequest("Uknown error"),
            }
        } else {
            NewsApiError::BadRequest("Uknown error")
        }
    }