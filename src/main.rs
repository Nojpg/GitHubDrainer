use std::rc::Rc;

use clap::{App, Arg};

use githubdrainer::repositories::{LanguageRequest, ListRepositoryLanguages};
use githubdrainer::search::{Search, SearchCommitsRequest};

#[derive(Debug)]
pub struct CommandLineArguments {
    pub commits_query: String,
    repository_query: String,
    language_query: String,
    pub output: String,
}

fn make_search_query(cli_args: &CommandLineArguments) -> SearchCommitsRequest {
    SearchCommitsRequest {
        query: cli_args.commits_query.clone(),
        page: Some(2),
        per_page: None,
        order: None,
        sort: None,
    }
}

#[tokio::main]
async fn main() {
    let cli_args = parse_args();
    println!("CLI = {:?}", &cli_args);
    let request = make_search_query(&cli_args);
    let response = match request.search().await {
        Ok(search_resp) => search_resp,
        Err(_) => panic!("no results"),
    };
    println!("size of response {}", response.items.len());
    let mut writer = match csv::WriterBuilder::new().from_path(&cli_args.output) {
        Ok(csv_writer) => csv_writer,
        Err(err) => panic!("Cannot create writer {}", err),
    };
    writer.write_record(&["Repository name", "Commit title", "Commit url"]);
    for item in response.items {
        let shared_item = Rc::new(item);
        let repo_name = &Rc::clone(&shared_item).repository.name;
        let author_login = match &Rc::clone(&shared_item).author {
            Some(user) => String::from(&user.login),
            None => String::from(""),
        };
        let lang_request = LanguageRequest {
            repository: repo_name.clone(),
            owner: author_login.to_string(),
        };
        let languages = match lang_request.list_repository_langs().await {
            Ok(resp) => resp,
            Err(e) => {
                println!("{}", e.to_string());
                continue;
            }
        };
        println!("languages : {:?}", languages);
        languages.languages
            .iter()
            .filter(|&(k, _)| k.as_str() == "C++")
            .for_each(|_| {
                println!("repo find");
                println!("repository name : {}", &repo_name);
                println!("repository commit title : {}", &shared_item.commit.message);
                println!("commit url : {}", &shared_item.html_url);
                writer.write_record(&[&repo_name, &shared_item.commit.message, &shared_item.html_url]);
            });
    }
    match writer.flush() {
        Ok(_) => (),
        Err(err) => panic!("Cannot flush writer {}", err),
    }
}

pub fn parse_args() -> CommandLineArguments {
    let args = App::new("GitHubDrainer")
        .arg(Arg::with_name("commits")
            .short("c")
            .long("commits")
            .help("Sets commits search query")
            .takes_value(true)
            .default_value("")
            .value_name("QUERY"))
        .arg(Arg::with_name("repository")
            .short("r")
            .long("repository")
            .help("Sets repository search query")
            .takes_value(true)
            .value_name("QUERY")
            .default_value(""))
        .arg(Arg::with_name("language")
            .short("l")
            .long("lang")
            .help("Sets language to search")
            .takes_value(true)
            .value_name("QUERY")
            .default_value("C++"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Sets output csv filename")
            .takes_value(true)
            .value_name("FILENAME")
            .default_value("commits_result.csv"))
        .get_matches();
    let repository_query_str = args.value_of("repository").unwrap_or_default().to_string();
    let commits_query_str = args.value_of("commits").unwrap_or_default().to_string();
    let language_query_str = args.value_of("language").unwrap_or_default().to_string();
    let filename = args.value_of("output").unwrap_or_default().to_string();
    CommandLineArguments {
        commits_query: commits_query_str,
        repository_query: repository_query_str,
        language_query: language_query_str,
        output: filename,
    }
}