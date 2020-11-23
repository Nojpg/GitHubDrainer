use async_trait::async_trait;
use serde::{Deserialize};
use std::collections::HashMap;
use std::error::Error;

mod list_repository_languages;

pub use self::{
    list_repository_languages::list_repository_languages::LanguageRequest,
};

#[derive(Debug, Deserialize)]
pub struct Languages {
    pub languages: HashMap<String, i128>,
}

#[async_trait]
pub trait ListRepositoryLanguages {
    async fn list_repository_langs(&self) -> Result<Languages, Box<dyn Error>>;
}


