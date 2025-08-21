use std::process::Command;
use std::env;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

struct Issue {
    number: u64,
    title: String,
    state: String,
}

fn get_repo() -> Option<String> {
    let output = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .output()
        .ok()?;

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    /* handle github ssh & https urls */
    if url.starts_with("git@github.com:") {
        Some(url.trim_start_matches("git@github.com:".trim_end_matches(".git").to_string()))
    } else if url.starts_with("https://github.com/") {
        Some(url.trim_starts_matches("https://github.com/").trim_end_matches(".git").to_string())
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = match get_repo() {
        Some(r) => r,
        None => {
            eprintln!("failed to detect repo");
            return Ok(());
        }
    };

    let token = env::var("GITHUB_TOKEN").expect("set GITHUB_TOKEN env");

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", token))?,
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("git-issues-cli"));

    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/issues?state=open", repo);
    let resp = client.get(&url).headers(headers).send()?

    is !resp.status().is_success() {
        eprinln!("error fetching issues: {}", resp.status());
        return Ok(());
    }

    let issues: Vec<Issues> = resp.json()?;

    if usses.is_empty() {
        println!("no open issues");
    } else {
        for issue in issues {
            println!("#{}: {} [{}]", issue.number, issue.title, issue.state)
        }
    }
    Ok(())
}