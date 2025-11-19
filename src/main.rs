mod config;
mod tools;

use chrono::Datelike;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use std::{env, fs};

#[derive(Deserialize)]
struct ConfigFile {
    github_token: String,
    org_name: String,
}

fn main() {

    let config = config::app::get_config();
    tools::logo::show(&config.unwrap());

    // Read config
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <CONFIG_JSON_PATH> [WEEKS]", args[0]);
        std::process::exit(1);
    }

    let config_path = &args[1];
    let config_content = fs::read_to_string(config_path)
        .expect("Failed to read config JSON file");
    let config: ConfigFile = serde_json::from_str(&config_content)
        .expect("Failed to parse JSON");

    let weeks_ahead: u32 = if args.len() >= 3 {
        args[2].parse().unwrap_or(26)
    } else {
        26
    };

    let client = Client::new();

    // 1️⃣ Get all repositories
    let mut page = 1;
    let mut repos = Vec::new();
    loop {
        let resp: serde_json::Value = client
            .get(&format!(
                "https://api.github.com/orgs/{}/repos?per_page=100&page={}",
                config.org_name, page
            ))
            .bearer_auth(&config.github_token)
            .header("User-Agent", "week-labeler")
            .send()
            .unwrap()
            .json()
            .unwrap();

        let repos_page = match resp.as_array() {
            Some(arr) => arr.clone(),
            None => {
                eprintln!("Failed to parse repos list: {:?}", resp);
                break;
            }
        };

        if repos_page.is_empty() {
            break;
        }

        repos.extend(repos_page);
        page += 1;
    }

    println!("Found {} repositories in org '{}'", repos.len(), config.org_name);

    // 2️⃣ Generate weekly labels
    let today = chrono::Local::now();
    let mut year = today.iso_week().year();
    let mut week = today.iso_week().week();

    let mut labels = Vec::new();
    for _ in 0..weeks_ahead {
        let week_str = format!("{:02}", week);
        let label_name = format!("week-{}-{}", year, week_str);
        let color = tools::color::week_color(week); // consistent gradient
        labels.push((label_name, color));

        week += 1;
        if week > 52 {
            week = 1;
            year += 1;
        }
    }

    // 3️⃣ Apply labels to each repo (force create or update)
    for repo in repos {
        let repo_name = repo["name"].as_str().unwrap();
        println!("Processing repo: {}", repo_name);

        for (label_name, color) in &labels {
            let body = json!({
                "name": label_name,
                "color": color,
                "description": format!("ISO week {}", label_name)
            });

            // Try to create label
            let resp = client
                .post(&format!(
                    "https://api.github.com/repos/{}/{}/labels",
                    config.org_name, repo_name
                ))
                .bearer_auth(&config.github_token)
                .header("User-Agent", "week-labeler")
                .json(&body)
                .send();

            match resp {
                Ok(r) if r.status().is_success() => {
                    println!("  ✅ Created label {} in {}", label_name, repo_name);
                }
                Ok(r) if r.status().as_u16() == 422 => {
                    // Label exists → update
                    let update_resp = client
                        .patch(&format!(
                            "https://api.github.com/repos/{}/{}/labels/{}",
                            config.org_name, repo_name, label_name
                        ))
                        .bearer_auth(&config.github_token)
                        .header("User-Agent", "week-labeler")
                        .json(&body)
                        .send();

                    match update_resp {
                        Ok(u) if u.status().is_success() => {
                            println!("  🔄 Updated existing label {} in {}", label_name, repo_name);
                        }
                        Ok(u) => {
                            eprintln!("  ❌ Failed to update label {} in {}: {:?}", label_name, repo_name, u.text());
                        }
                        Err(e) => {
                            eprintln!("  ❌ Request failed updating label {} in {}: {:?}", label_name, repo_name, e);
                        }
                    }
                }
                Ok(r) => {
                    eprintln!("  ❌ Failed to create label {} in {}: {:?}", label_name, repo_name, r.text());
                }
                Err(e) => {
                    eprintln!("  ❌ Request failed for label {} in {}: {:?}", label_name, repo_name, e);
                }
            }
        }
    }
}
