pub mod list_repository_languages {
    use std::collections::HashMap;
    use std::error::Error;
    use std::rc::Rc;

    use async_trait::async_trait;
    use reqwest::header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT};
    use reqwest::Response;
    use serde::Serialize;

    use crate::repositories::{Languages, ListRepositoryLanguages};

    #[derive(Serialize)]
    pub struct LanguageRequest {
        pub owner: String,
        pub repository: String,
    }

    #[async_trait]
    impl ListRepositoryLanguages for LanguageRequest {
        async fn list_repository_langs(&self) -> Result<Languages, Box<dyn Error>> {
            let request_url = format!(
                "https://api.github.com/repos/{owner}/{repository}/languages",
                owner = self.owner,
                repository = self.repository
            );
            println!("url : {}", request_url);
            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.cloak-preview+json"));
            headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
            println!("headers: {:?}", headers);
            let client = reqwest::Client::new();
            let response = client
                .get(&request_url)
                .headers(headers)
                .send()
                .await;
            // todo change it to function with track of rate limit
            let rate_limit: i32 = match &response {
                Ok(resp) => resp
                    .headers()
                    .iter()
                    .filter(|&(k, _)| k.as_str().starts_with("x-ratelimit-remaining"))
                    .filter_map(|(k, v)| v.to_str().ok())
                    .filter_map(|v| String::from(v).parse().ok())
                    .next()
                    .unwrap(),
                Err(_) => 0,
            };

            if rate_limit == 0 {
                return Err(From::from("Not enough rate limit for request or error while parsing result"));
            }

            &response
                .as_ref()
                .unwrap()
                .headers()
                .iter()
                .filter(|&(k, _)| k.as_str().starts_with("x-ratelimit"))
                .for_each(|(k, v)| println!("{}: {:?}", k, v));
            let languages_response: HashMap<String, i128> = response.unwrap().json().await?;
            println!("{:?}", languages_response);
            Ok(Languages {
                languages: languages_response,
            })
        }
    }
}