use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, Utc};

// コミットハッシュオブジェクト
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommitHash {
    pub hash: String,
}
impl CommitHash {
    pub fn new(hash: impl ToString) -> Self {
        Self {
            hash: hash.to_string(),
        }
    }
}

// コミット情報オブジェクト
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommitInfo {
    commit_hash: Option<String>,
    tree_hash: Option<String>,
    parent_hash: Option<String>,
    author_name: Option<String>,
    author_email: Option<String>,
    #[serde(deserialize_with = "from_ts")]
    author_date: DateTime<Utc>,
    committer_name: Option<String>,
    committer_email: Option<String>,
    #[serde(deserialize_with = "from_ts")]
    commit_date: DateTime<Utc>,
    comment: Option<String>,
}

// ブランチオブジェクト
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub head: CommitHash,
    pub is_current: bool,
}
impl Branch {
    pub fn new(name: impl ToString, head: impl ToString, is_current: bool) -> Self {
        Self {
            name: name.to_string(),
            head: CommitHash::new(head),
            is_current,
        }
    }
}

// 環境情報
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Env {
    pub git_dir: String,
    pub is_test: bool,
}