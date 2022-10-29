use std::{error::Error, env, str::FromStr};

use reqwest::header::{ACCEPT, USER_AGENT, HeaderMap, HeaderValue, AUTHORIZATION};

use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Notifications = Vec<Notification>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    //pub repository: Repository,
    //pub subject: Subject,
    //pub reason: String,
    //pub unread: bool,
    //#[serde(rename = "updated_at")]
    //pub updated_at: String,
    //#[serde(rename = "last_read_at")]
    //pub last_read_at: String,
    //pub url: String,
    //#[serde(rename = "subscription_url")]
    //pub subscription_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub title: String,
    pub url: String,
    #[serde(rename = "latest_comment_url")]
    pub latest_comment_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let personal_github_token_key = "PERSONAL_GITHUB_TOKEN";

    let github_token = match env::var(personal_github_token_key) {
        Ok(value) => value,
        Err(err) => panic!("${} is not set on env ({})", personal_github_token_key ,err)
    };


    let mut headers = HeaderMap::new();

    let github_token_value = format!("token {}", github_token);
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&github_token_value).unwrap());

    let client = reqwest::Client::new();

    let request = client.get("https://api.github.com/notifications")
        .headers(headers);

    //println!("{:?}", request);

    let response = request.send().await?;

    println!("{:#?}", response);
    match response.status() {
        reqwest::StatusCode::OK => {
            let notifications = response.json::<Notifications>().await?;
            println!("{:#?}", notifications);
        },
        _ => {
            panic!("Uh oh! Something unexpected happened!!! {:?}", response);
        }
    }

    Ok(())
}
