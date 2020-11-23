#[cfg(test)]
mod tests {
    use githubdrainer::search::{Search, SearchCommitsRequest};

    // curl   -H "Accept: application/vnd.github.cloak-preview+json"   https://api.github.com/search/commits?q=tetris+language:assembly&sort=stars&order=desc
    #[tokio::test]
    async fn search_commit_test() {
        let request = SearchCommitsRequest {
            query: String::from("tetris"),
            sort: None,
            order: None,
            per_page: None,
            page: None,
        };
        let res = request.search().await.unwrap();
        println!("{:?}", res);
    }
}