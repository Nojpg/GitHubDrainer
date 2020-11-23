use std::error::Error;

use async_trait::async_trait;
use serde::Deserialize;
use crate::entities::{Repository, CommitRepository};

mod search_repository;
mod search_commits;

pub use self::{
    search_commits::search_commits::SearchCommitsRequest,
    search_repository::search_repository::SearchRepositoryRequest,
};

#[derive(Deserialize, Debug)]
pub struct SearchResponse<T: Item> {
    pub total_count: i32,
    pub incomplete_results: bool,
    pub items: Vec<T>,
}

pub trait GitHubRequest {}

pub trait Item {}

impl Item for Repository {}

impl Item for CommitRepository {}

#[async_trait]
pub trait Search<T: GitHubRequest, R: Item> {
    async fn search(&self) -> Result<SearchResponse<R>, Box<dyn Error>>;
}