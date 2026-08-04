#![allow(clippy::from_over_into)]

use egonik_site::{
    core::Repository,
    database::connection::get_connection_pool,
    publications::{dto::PublicationItemDto, repository::PublicationsRepository},
};
use serde::{Deserialize, Serialize};

const PUBLICATION_LOOKUP_URL: &str = "https://api.openalex.org/works?filter=authorships.author.id:A5070154461&select=id,title,publication_year,doi,type,cited_by_count&sort=publication_date:desc&per-page=200";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScientistResponse {
    meta: Metadata,
    results: Vec<Article>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Metadata {
    count: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Article {
    title: String,
    publication_year: i32,
    doi: String,
    cited_by_count: i32,
}

impl Into<PublicationItemDto> for Article {
    fn into(self) -> PublicationItemDto {
        PublicationItemDto {
            title: self.title,
            link: self.doi,
            year: self.publication_year,
            journal: "".into(),
            abs: "".into(),
        }
    }
}

#[tokio::main]
async fn main() {
    let pool = get_connection_pool();
    let mut repo = PublicationsRepository::new(pool);
    let response = reqwest::get(PUBLICATION_LOOKUP_URL)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let scientist_response: ScientistResponse = serde_json::from_str(&response).unwrap();
    scientist_response.results.into_iter().for_each(|article| {
        let insert_article: PublicationItemDto = article.into();
        repo.create_article(insert_article).unwrap()
    });
    let articles_in_db = repo.get_all().unwrap();
    println!("{articles_in_db:#?}")
}
