
use std::{env, error::Error};
use reqwest::header::{ACCEPT, AUTHORIZATION};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let personal_github_token_key = "PERSONAL_GITHUB_TOKEN";

    let github_token = match env::var(personal_github_token_key) {
        Ok(value) => value,
        Err(err) => panic!("${} is not set on env ({})", personal_github_token_key ,err)
    };

    let client = reqwest::Client::new();

    println!("token {}", github_token);
    
    let request = client.get("https://api.github.com/notifications")
            .header(ACCEPT, "application/vnd.github+json")
            .header(AUTHORIZATION, format!("token {}", github_token))
            .bearer_auth(github_token);

    println!("{:?}", request);

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
