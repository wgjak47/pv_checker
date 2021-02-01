use std::collections::HashMap;
use std::fmt;

pub trait VersionInfo: fmt::Display {
    // TODO use this to integrate with template engine
    fn to_map(&self) -> HashMap<&'static str, String>;
}

pub struct GithubCommitVersion {
    sha: String,
    date: String,
}

impl VersionInfo for GithubCommitVersion {
    fn to_map(&self) -> HashMap<&'static str, String> {
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

pub struct GithubTagVersion {
    tag_version: String,
}

impl GithubTagVersion {
    pub fn new(tag_version: String) -> Self {
        GithubTagVersion { tag_version }
    }
}

impl fmt::Display for GithubTagVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tag version: {}", self.tag_version)
    }
}

impl VersionInfo for GithubTagVersion {
    fn to_map(&self) -> HashMap<&'static str, String> {
        [("tag_version", self.tag_version.clone())]
            .iter()
            .cloned()
            .collect()
    }
}

pub struct PypiVersion {
    version: String,
}

impl PypiVersion {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}

impl fmt::Display for PypiVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "version: {}", self.version)
    }
}

impl VersionInfo for PypiVersion {
    fn to_map(&self) -> HashMap<&'static str, String> {
        [("version", self.version.clone())]
            .iter()
            .cloned()
            .collect()
    }
}
