pub mod search_commits {
    use std::error::Error;

    use async_trait::async_trait;
    use reqwest::header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT};
    use serde::Serialize;

    use crate::search::{GitHubRequest, SearchResponse, Search};
    use crate::entities::CommitRepository;

    #[derive(Serialize, Debug)]
    pub struct SearchCommitsRequest {
        pub query: String,
        pub sort: Option<String>,
        pub order: Option<String>,
        pub per_page: Option<i16>,
        pub page: Option<i16>,
    }

    impl GitHubRequest for SearchCommitsRequest {}

    #[async_trait]
    impl Search<SearchCommitsRequest, CommitRepository> for SearchCommitsRequest {
        async fn search(&self) -> Result<SearchResponse<CommitRepository>, Box<dyn Error>> {
            let request_url = format!(
                "https://api.github.com/search/commits?q={query}",
                query = self.query);
            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.cloak-preview+json"));
            headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
            // println!("headers: {:?}", headers);
            let client = reqwest::Client::new();
            let response = client.get(&request_url)
                .headers(headers).send().await?;
            // println!("{:?}", response);
            let commits_response: SearchResponse<CommitRepository> = response.json().await?;
            // println!("{:?}", commits_response);
            Ok(commits_response)
        }
    }
}