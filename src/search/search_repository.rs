pub mod search_repository {
    use std::error::Error;

    use async_trait::async_trait;
    use reqwest::header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT};
    use serde::Serialize;

    use crate::search::{GitHubRequest, Search, SearchResponse};
    use crate::entities::Repository;

    #[derive(Serialize, Debug)]
    pub struct SearchRepositoryRequest {
        pub query: String,
        pub sort: String,
        pub order: String,
    }

    impl GitHubRequest for SearchRepositoryRequest {}

    #[async_trait]
    impl Search<SearchRepositoryRequest, Repository> for SearchRepositoryRequest {
        async fn search(&self) -> Result<SearchResponse<Repository>, Box<dyn Error>> {
            let request_url = format!("https://api.github.com/search/repositories?q={query}&sort={sort}&order={order}",
                                      query = self.query, sort = self.sort, order = self.order);
            let mut headers = HeaderMap::new();
            headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
            headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.cloak-preview+json"));
            // println!("headers: {:?}", headers);
            let client = reqwest::Client::new();
            let response = client.get(&request_url).headers(headers).send().await?;
            // println!("Response: {:?}", response);
            let repositories_response: SearchResponse<Repository> = response.json().await?;
            // println!("Repo response: {:?}", repositories_response);
            Ok(repositories_response)
        }
    }
}