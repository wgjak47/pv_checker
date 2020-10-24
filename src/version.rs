use std::collections::HashMap;
use std::fmt;

pub trait VersionInfo: fmt::Display {
    // TODO use this to integrate with template engine
    fn ToMap(&self) -> HashMap<&'static str, String>;
}

pub struct GithubCommitVersion {
    sha: String,
    date: String,
}

impl VersionInfo for GithubCommitVersion {
    fn ToMap(&self) -> HashMap<&'static str, String> {
        [("sha", self.sha.clone()), ("date", self.date.clone())]
            .iter()
            .cloned()
            .collect()
    }
}

impl fmt::Display for GithubCommitVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sha: {}, date: {}", self.sha, self.date)
    }
}

impl GithubCommitVersion {
    pub fn new(sha: String, date: String) -> Self {
        GithubCommitVersion { sha, date }
    }
}
