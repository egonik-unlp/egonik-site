#![allow(clippy::from_over_into)]
use std::todo;

use crate::publications;
use crate::publications::dto::PublicationItemWithMetadataDto;
use crate::publications::metadata::MetadataTableDto;
use crate::publications::model::PublicationItem;
use crate::publications::{dto::PublicationItemDto, repository::PublicationsRepository};
use anyhow::Context;
use serde::{Deserialize, Serialize};

const PUBLICATION_LOOKUP_URL: &str = "https://api.openalex.org/works?filter=authorships.author.id:A5070154461&select=id,title,publication_year,doi,type,cited_by_count&sort=publication_date:desc&per-page=200";
const PUBLICATION_METADATA_URL: &str = "https://egonik-unlp.github.io/assets/data/works.json";
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

    pub async fn get_all_publications(&self) -> anyhow::Result<Vec<PublicationItemDto>> {
        let publications_db = self.repo.clone().get_all()?;
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
    async fn get_metadata(&self) -> anyhow::Result<MetadataTableDto> {
        let response = reqwest::get(PUBLICATION_METADATA_URL)
            .await
            .context("Error requesting metadata")?
            .text()
            .await
            .context("Error getting body")?;
        serde_json::from_str(&response).context("Error serializing")
    }
    pub async fn get_publications_with_metadata(
        &self,
    ) -> anyhow::Result<Vec<PublicationItemWithMetadataDto>> {
        let publications_list = self
            .get_all_publications()
            .await
            .context("error fetching pubs from db")?;
        let metadata = self
            .get_metadata()
            .await
            .context("error getting metadata")?;
        let pub_with_metadata: Vec<PublicationItemWithMetadataDto> = publications_list
            .into_iter()
            .filter_map(|publication| {
                metadata
                    .works
                    .iter()
                    .find(|work| publication.is_its_metadata(work))
                    .cloned()
                    .map(|work| PublicationItemWithMetadataDto::new(publication, work))
            })
            .collect();
        Ok(pub_with_metadata)
    }
    pub async fn sync_publication_history(mut self) -> anyhow::Result<()> {
        let response = reqwest::get(PUBLICATION_LOOKUP_URL)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let scientist_response: ScientistResponse = serde_json::from_str(&response).unwrap();
        for article in scientist_response.results {
            let insert_article: PublicationItemDto = article.into();
            self.repo
                .create_article(insert_article.clone())
                .await
                .with_context(|| format!("Couldn't insert {:?} into db", insert_article))?
        }
        Ok(())
    }
}
