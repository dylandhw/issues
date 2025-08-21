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
