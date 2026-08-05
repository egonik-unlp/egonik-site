use crate::portfolio::{
    dto::PortfolioItemDto,
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
pub async fn get_public_github_repos(username: &str) -> Result<Vec<GithubRepo>, reqwest::Error> {
    let url = format!("https://api.github.com/users/{username}/repos");

    let repos = reqwest::Client::new()
        .get(url)
        .header(USER_AGENT, "egonik-site")
        .header(ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<GithubRepo>>()
        .await?;

    Ok(repos)
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
            .map(Into::into)
        {
            self.repo.clone().create_article(portfolio_item).await?
        }

        Ok(())
    }

    pub async fn get_all(mut self) -> anyhow::Result<Vec<PortfolioItemDto>> {
        self.repo.get_all().await
    }
}
