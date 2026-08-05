#![allow(clippy::from_over_into)]
use crate::publications::model::PublicationItem;
use crate::publications::{dto::PublicationItemDto, repository::PublicationsRepository};
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
#[derive(Debug, Clone)]
pub struct PublicationsService {
    repo: PublicationsRepository,
}
impl PublicationsService {
    pub fn new(repo: PublicationsRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all_publications(mut self) -> anyhow::Result<Vec<PublicationItemDto>> {
        let publications_db = self.repo.get_all()?;
        let publications_type = publications_db
            .into_iter()
            .map(|publication| {
                PublicationItemDto::new(
                    publication.title,
                    publication.abs,
                    publication.year,
                    publication.journal,
                    publication.link,
                )
            })
            .collect();
        Ok(publications_type)
    }

    pub async fn sync_publication_history(mut self) {
        let response = reqwest::get(PUBLICATION_LOOKUP_URL)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let scientist_response: ScientistResponse = serde_json::from_str(&response).unwrap();
        scientist_response.results.into_iter().for_each(|article| {
            let insert_article: PublicationItemDto = article.into();
            self.repo.create_article(insert_article).unwrap()
        });
        let articles_in_db = self.repo.get_all().unwrap();
        println!("{articles_in_db:#?}")
    }
}
