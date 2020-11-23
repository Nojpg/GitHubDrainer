#[cfg(test)]
mod tests{
    use githubdrainer::repositories::{LanguageRequest, ListRepositoryLanguages};

    #[tokio::test]
    async fn list_repositories_test() {
        let request = LanguageRequest{
            owner: String::from("clap-rs"),
            repository: String::from("clap"),
        };
        match request.list_repository_langs().await {
            Ok(resp) => println!("{:?}", resp),
            Err(_) => {
                println!("maybe used all request \
                https://docs.github.com/en/rest/overview/resources-in-the-rest-api#rate-limiting");
                assert_eq!(true, false)
            },
        };
    }
}