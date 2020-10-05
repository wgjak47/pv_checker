use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[async_trait]
pub trait PackageRemote {
    async fn fetch_latest_version(&self) -> Result<HashMap<String, String>>;
}

pub fn get_package_remote(
    url: String,
    package_type: String,
    version_type: String,
) -> Result<Box<dyn PackageRemote>> {
    if package_type == "github" {
        let remote = GitHubRemote::new(url, version_type)?;
        return Ok(Box::new(remote));
    }

    Err(anyhow!("invalid package type"))
}

#[derive(Serialize, Deserialize)]
struct Author {
    name: String,
    email: String,
    date: String,
}

#[derive(Serialize, Deserialize)]
struct Tree {
    // this is the tree sha, don't use this
    sha: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Commit {
    author: Author,
}

#[derive(Serialize, Deserialize)]
struct GitHubCommitResp {
    // this is the commit sha, use this
    sha: String,
    node_id: String,
    commit: Commit,
}

struct GitHubRemote {
    owner: String,
    repo: String,
    version_type: String,
}

#[async_trait]
impl PackageRemote for GitHubRemote {
    async fn fetch_latest_version(&self) -> Result<HashMap<String, String>> {
        let request_url = format!(
            "https://api.github.com/repos/{owner}/{repo}/commits",
            owner = self.owner,
            repo = self.repo,
        );

        let client = reqwest::Client::new();

        let resp = client
            .get(&request_url)
            .query(&[("per_page", "1"), ("page", "1")])
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "pv-checker-v0.1.0")
            .send()
            .await?;

        let status_code = resp.status();

        if status_code != StatusCode::OK {
            let rest_text = resp.text().await?;
            return Err(anyhow!(
                "failed to request github, status code: {} response: {}",
                rest_text,
                status_code
            ));
        }

        let commits: Vec<GitHubCommitResp> = resp.json().await?;

        if commits.is_empty() {
            return Err(anyhow!("repo has commit"));
        }

        let newest_commit = &commits[0];

        Ok([
            ("sha".to_string(), newest_commit.sha.clone()),
            ("date".to_string(), newest_commit.commit.author.date.clone()),
        ]
        .iter()
        .cloned()
        .collect())
    }
}

impl GitHubRemote {
    fn new(url: String, version_type: String) -> Result<Self> {
        let github_url = Url::parse(&url)?;
        let path_segments: Vec<&str> = github_url
            .path_segments()
            .ok_or_else(|| anyhow!("url invalid"))?
            .collect();

        if path_segments.len() != 2 {
            return Err(anyhow!("url invalid"));
        }

        Ok(GitHubRemote {
            owner: path_segments[0].to_string(),
            repo: path_segments[1].to_string(),
            version_type,
        })
    }
}