#[cfg(test)]
mod tests {

// TODO mock

    use githubdrainer::search::{Search, SearchRepositoryRequest};

    // curl https://api.github.com/search/repositories?q=tetris+language:assembly&sort=stars&order=desc
    // GET https://api.github.com/search/repositories?q=tetris language:assembly&sort=stars&order=desc
    #[tokio::test]
    async fn search_repo_test() /*-> Result<(), Box<dyn Error>>*/ {
        let request = SearchRepositoryRequest {
            query: String::from("tetris"),
            sort: String::from("stars"),
            order: String::from("desc"),
        };
        let res = request.search().await;
        println!("{:?}", res);
    }
}
