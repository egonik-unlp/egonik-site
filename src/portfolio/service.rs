use std::collections::HashMap;

use crate::portfolio::{
    dto::{PortfolioItemDto, PortfolioItemWithMetadataDto},
    metadata::{ProjectMetadataDto, ProjectMetadataTableDto},
    model::{PortfolioItemRow, TagRow},
    repository::{PortfolioItemsRepository, PortfolioTuple},
};

#[derive(Debug, Clone)]
pub struct PortfolioService {
    repo: PortfolioItemsRepository,
}
use anyhow::Context;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};

const GITHUB_ACCOUT: &str = "egonik-unlp";
const PROJECT_METADATA_URL: &str = "https://egonik-unlp.github.io/assets/data/projects.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubRepo {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,

    pub fork: bool,
    pub archived: bool,

    pub language: Option<String>,

    #[serde(default)]
    pub topics: Vec<String>,

    pub stargazers_count: u64,
}
impl Into<PortfolioTuple> for GithubRepo {
    fn into(self) -> PortfolioTuple {
        let GithubRepo {
            id,
            name,
            full_name,
            html_url,
            description,
            fork,
            archived,
            language,
            topics,
            stargazers_count,
        } = self;
        let description = description.unwrap_or("".to_string());
        let title = name;
        let public = true;
        let public_url = Some(html_url);
        let tags = topics
            .into_iter()
            .map(|value| TagRow {
                value,
                portfolio_item_id: None,
            })
            .collect();
        (
            PortfolioItemRow {
                description,
                title,
                public_url,
                public,
            },
            tags,
        )
    }
}
/// GitHub's maximum page size for this endpoint.
const GITHUB_PAGE_SIZE: usize = 100;

pub async fn get_public_github_repos(username: &str) -> Result<Vec<GithubRepo>, reqwest::Error> {
    // This endpoint paginates and defaults to 30 items per page. The original request
    // sent no `per_page`/`page` and kept only the first page, so the sync silently
    // capped the portfolio at the 30 most recently pushed repositories -- which is why
    // repos present in projects.json had no row in the database at all.
    let client = reqwest::Client::new();
    let mut repos = Vec::new();

    for page in 1u32.. {
        let url = format!(
            "https://api.github.com/users/{username}/repos?per_page={GITHUB_PAGE_SIZE}&page={page}"
        );
        let batch = client
            .get(url)
            .header(USER_AGENT, "egonik-site")
            .header(ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<GithubRepo>>()
            .await?;

        let is_last_page = batch.len() < GITHUB_PAGE_SIZE;
        repos.extend(batch);
        if is_last_page {
            break;
        }
    }

    Ok(repos)
}
/// Takes the entry for `key` out of the map, first by exact hit and then by a
/// case-insensitive scan -- GitHub repository names are case-preserving but not
/// case-sensitive, so `projects.json` and the database can disagree on casing.
fn remove_ignore_ascii_case(
    repositories: &mut HashMap<String, ProjectMetadataDto>,
    key: &str,
) -> Option<ProjectMetadataDto> {
    if let Some(metadata) = repositories.remove(key) {
        return Some(metadata);
    }
    let matched = repositories
        .keys()
        .find(|name| name.eq_ignore_ascii_case(key))?
        .clone();
    repositories.remove(&matched)
}

impl PortfolioService {
    pub fn new(repo: PortfolioItemsRepository) -> Self {
        Self { repo }
    }

    pub async fn sync_from_github(self) -> anyhow::Result<()> {
        // TODO: Es medio criminal pero zafa por ahora
        for portfolio_item in get_public_github_repos(GITHUB_ACCOUT)
            .await
            .context("Error syncing with gh")?
            .into_iter()
            // Forks are other people's work and archived repositories are retired --
            // neither belongs in a "selected work" listing.
            .filter(|repo| !repo.fork && !repo.archived)
            .map(Into::into)
        {
            self.repo.clone().create_article(portfolio_item).await?
        }

        Ok(())
    }
    async fn get_metadata(&self) -> anyhow::Result<ProjectMetadataTableDto> {
        let response = reqwest::get(PROJECT_METADATA_URL)
            .await
            .context("Error fetching portfolio metadata")?
            .text()
            .await
            .context("Error getting response body")?;
        serde_json::from_str(&response).context("Error serializing metadata")
    }
    pub async fn get_all_with_metadata(&self) -> anyhow::Result<Vec<PortfolioItemWithMetadataDto>> {
        let metadata = self
            .get_metadata()
            .await
            .context("Error getting metadata")?;
        let projects = self
            .get_all()
            .await
            .context("Error getting projects from db")?;
        // `repositories` is keyed by GitHub repository name, which is exactly what the
        // sync stores in `portfolio_items.title` (see `Into<PortfolioTuple> for GithubRepo`).
        // The inner `metadata.title` is the *display* name ("app" -> "Pathfinder"), so
        // matching on it only ever succeeded when the two happened to coincide.
        //
        // Draining the map instead of cloning it keeps this O(n) and stops one metadata
        // entry from being handed to two different projects.
        let mut repositories = metadata.repositories;
        let projects_with_metadata = projects
            .into_iter()
            .filter_map(|project| {
                let project_metadata = remove_ignore_ascii_case(&mut repositories, &project.title)?;
                Some(PortfolioItemWithMetadataDto::new(project, project_metadata))
            })
            .collect();
        // Projects with no entry in projects.json are dropped on purpose: that file is
        // the curation list for this section, not just an enrichment source.
        Ok(projects_with_metadata)
    }
    pub async fn get_all(&self) -> anyhow::Result<Vec<PortfolioItemDto>> {
        self.repo.clone().get_all().await
    }
}
