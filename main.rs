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
    
}