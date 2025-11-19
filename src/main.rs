mod tools;
mod config;

use chrono::Datelike;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use std::{env, fs};

#[derive(Deserialize)]
struct ConfigFile {
    github_token: String,
    org_name: Option<String>,
    repo: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default values
    let mut config_path = "config.json".to_string();
    let mut weeks_ahead: u32 = 26;

    // Simple flag parsing
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" => {
                if i + 1 < args.len() {
                    config_path = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("-f requires a file path");
                    std::process::exit(1);
                }
            }
            "-w" => {
                if i + 1 < args.len() {
                    weeks_ahead = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("Invalid number for -w, using default 26");
                        26
                    });
                    i += 1;
                } else {
                    eprintln!("-w requires a number of weeks");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
            }
        }
        i += 1;
    }

    let config = read_config(&config_path);

    let client = Client::new();
    let repos = fetch_repos(&client, &config);
    println!("Found {} repositories", repos.len());

    let labels = generate_labels(weeks_ahead);

    for repo in repos {
        apply_labels_to_repo(&client, &config.github_token, &config.org_name, &repo, &labels);
    }
}


/// Reads JSON config
fn read_config(path: &str) -> ConfigFile {
    let content = fs::read_to_string(path).expect("Failed to read config JSON file");
    serde_json::from_str(&content).expect("Failed to parse JSON")
}

/// Fetch repositories based on config
fn fetch_repos(client: &Client, config: &ConfigFile) -> Vec<serde_json::Value> {
    if let Some(repo_name) = &config.repo {
        match client
            .get(&format!("https://api.github.com/repos/{}", repo_name))
            .bearer_auth(&config.github_token)
            .header("User-Agent", "week-labeler")
            .send()
        {
            Ok(resp) => match resp.json() {
                Ok(json) => vec![json],
                Err(e) => {
                    eprintln!("Failed to parse JSON for repo {}: {:?}", repo_name, e);
                    vec![]
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch repo {}: {:?}", repo_name, e);
                vec![]
            }
        }
    } else if let Some(org_name) = &config.org_name {
        fetch_org_repos(client, org_name, &config.github_token)
    } else {
        fetch_personal_repos(client, &config.github_token)
    }
}

/// Fetch all organization repositories
fn fetch_org_repos(client: &Client, org_name: &str, token: &str) -> Vec<serde_json::Value> {
    let mut repos = Vec::new();
    let mut page = 1;

    loop {
        let resp: serde_json::Value = match client
            .get(&format!("https://api.github.com/orgs/{}/repos?per_page=100&page={}", org_name, page))
            .bearer_auth(token)
            .header("User-Agent", "week-labeler")
            .send()
        {
            Ok(r) => match r.json() {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("Failed to parse org repos JSON: {:?}", e);
                    break;
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch org repos: {:?}", e);
                break;
            }
        };

        let page_repos = resp.as_array().cloned().unwrap_or_default();
        if page_repos.is_empty() { break; }

        repos.extend(page_repos);
        page += 1;
    }

    repos
}

/// Fetch all personal repositories
fn fetch_personal_repos(client: &Client, token: &str) -> Vec<serde_json::Value> {
    let mut repos = Vec::new();
    let mut page = 1;

    loop {
        let resp: serde_json::Value = match client
            .get(&format!("https://api.github.com/user/repos?per_page=100&page={}", page))
            .bearer_auth(token)
            .header("User-Agent", "week-labeler")
            .send()
        {
            Ok(r) => match r.json() {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("Failed to parse personal repos JSON: {:?}", e);
                    break;
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch personal repos: {:?}", e);
                break;
            }
        };

        let page_repos = resp.as_array().cloned().unwrap_or_default();
        if page_repos.is_empty() { break; }

        repos.extend(page_repos);
        page += 1;
    }

    repos
}

/// Generate weekly labels
fn generate_labels(weeks_ahead: u32) -> Vec<(String, String)> {
    let today = chrono::Local::now();
    let mut year = today.iso_week().year();
    let mut week = today.iso_week().week();

    let mut labels = Vec::new();
    for _ in 0..weeks_ahead {
        let week_str = format!("{:02}", week);
        let label_name = format!("week-{}-{}", year, week_str);
        let color = tools::color::week_color(week);
        labels.push((label_name, color));

        week += 1;
        if week > 52 {
            week = 1;
            year += 1;
        }
    }

    labels
}

/// Apply labels to a single repository safely
fn apply_labels_to_repo(
    client: &Client,
    token: &str,
    org_name: &Option<String>,
    repo: &serde_json::Value,
    labels: &[(String, String)],
) {
    let repo_name = match repo.get("name").and_then(|n| n.as_str()) {
        Some(name) => name,
        None => {
            eprintln!("Repo JSON missing name, skipping");
            return;
        }
    };

    let owner_name = if let Some(o) = org_name {
        o.as_str()
    } else if let Some(owner) = repo.get("owner").and_then(|v| v.get("login")).and_then(|l| l.as_str()) {
        owner
    } else {
        eprintln!("Repo {} missing owner info, skipping", repo_name);
        return;
    };

    println!("Processing repo: {}/{}", owner_name, repo_name);

    for (label_name, color) in labels {
        let body = json!({
            "name": label_name,
            "color": color,
            "description": format!("ISO week {}", label_name)
        });

        match client
            .post(&format!("https://api.github.com/repos/{}/{}/labels", owner_name, repo_name))
            .bearer_auth(token)
            .header("User-Agent", "week-labeler")
            .json(&body)
            .send()
        {
            Ok(r) if r.status().is_success() => {
                println!("  ✅ Created label {} in {}", label_name, repo_name);
            }
            Ok(r) if r.status().as_u16() == 422 => {
                // Label exists → update
                match client
                    .patch(&format!("https://api.github.com/repos/{}/{}/labels/{}", owner_name, repo_name, label_name))
                    .bearer_auth(token)
                    .header("User-Agent", "week-labeler")
                    .json(&body)
                    .send()
                {
                    Ok(u) if u.status().is_success() => {
                        println!("  🔄 Updated existing label {} in {}", label_name, repo_name);
                    }
                    Ok(u) => eprintln!("  ❌ Failed to update label {} in {}: {:?}", label_name, repo_name, u.text()),
                    Err(e) => eprintln!("  ❌ Request failed updating label {} in {}: {:?}", label_name, repo_name, e),
                }
            }
            Ok(r) => eprintln!("  ❌ Failed to create label {} in {}: {:?}", label_name, repo_name, r.text()),
            Err(e) => eprintln!("  ❌ Request failed for label {} in {}: {:?}", label_name, repo_name, e),
        }
    }
}
