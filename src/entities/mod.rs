use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    id: i128,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type: String,
    site_admin: bool,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    id: i128,
    node_id: String,
    pub name: String,
    full_name: String,
    private: bool,
    owner: User,
    html_url: String,
    description: Option<String>,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    deployments_url: String,
    created_at: Option<String>,
    //for commits ? as below
    updated_at: Option<String>,
    pushed_at: Option<String>,
    git_url: Option<String>,
    ssh_url: Option<String>,
    clone_url: Option<String>,
    svn_url: Option<String>,
    homepage: Option<String>,
    size: Option<i32>,
    stargazers_count: Option<i32>,
    watchers_count: Option<i32>,
    language: Option<String>,
    has_issues: Option<bool>,
    has_projects: Option<bool>,
    has_downloads: Option<bool>,
    has_wiki: Option<bool>,
    has_pages: Option<bool>,
    forks_count: Option<i32>,
    mirror_url: Option<String>,
    archived: Option<bool>,
    disabled: Option<bool>,
    open_issues_count: Option<i32>,
    license: Option<License>,
    forks: Option<i32>,
    open_issues: Option<i32>,
    watchers: Option<i32>,
    default_branch: Option<String>,
    score: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct License {
    key: String,
    name: String,
    spdx_id: String,
    url: Option<String>,
    node_id: String,
}

#[derive(Deserialize, Debug)]
pub struct CommitRepository {
    pub url: String,
    pub sha: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: Commit,
    pub commit_author: Option<CommitAuthor>,
    pub author: Option<User>,
    pub committer: Option<User>,
    pub parents: Vec<ParentCommit>,
    pub repository: Repository,
    pub score: f64,
    pub node_id: String,
}


#[derive(Deserialize, Debug)]
pub struct ParentCommit {
    url: String,
    html_url: String,
    sha: String,
}

#[derive(Deserialize, Debug)]
pub struct CommitterNullable {}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub url: String,
    pub author: CommitAuthor,
    pub committer: CommitCommitter,
    pub message: String,
    pub tree: CommitTree,
    pub comment_count: i16,
}

#[derive(Deserialize, Debug)]
pub struct CommitTree {
    pub url: String,
    pub sha: String,
}

#[derive(Deserialize, Debug)]
pub struct CommitCommitter {
    pub date: String,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct CommitAuthor {
    pub date: String,
    pub name: String,
    pub email: String,
}
