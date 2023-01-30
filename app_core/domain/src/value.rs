use crate::error::DomainError;
use crate::types::Result;
use anyhow::anyhow;
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
impl CommitInfo {
    pub fn new(
        commit_hash: &str,
        tree_hash: &str,
        parent_hash: &str,
        author_name: &str,
        author_email: &str,
        author_date: &str,
        committer_name: &str,
        committer_email: &str,
        commit_date: &str,
        comment: &str,
    ) -> Result<Self> {
        let commit_hash = if commit_hash != "" {
            Some(commit_hash.to_string())
        } else {
            None
        };

        let tree_hash = if tree_hash != "" {
            Some(tree_hash.to_string())
        } else {
            None
        };

        let parent_hash = if parent_hash != "" {
            Some(parent_hash.to_string())
        } else {
            None
        };

        let author_name = if author_name != "" {
            Some(author_name.to_string())
        } else {
            None
        };

        let author_email = if author_email != "" {
            Some(author_email.to_string())
        } else {
            None
        };

        if author_date.is_empty() {
            return Err(anyhow!(DomainError::ValidationError));
        }
        let author_date = DateTime::parse_from_rfc2822(author_date)?.with_timezone(&Utc);

        let committer_name = if committer_name != "" {
            Some(committer_name.to_string())
        } else {
            None
        };

        let committer_email = if committer_email != "" {
            Some(committer_email.to_string())
        } else {
            None
        };

        if commit_date.is_empty() {
            return Err(anyhow!(DomainError::ValidationError));
        }
        let commit_date = DateTime::parse_from_rfc2822(commit_date)?.with_timezone(&Utc);

        let comment = if comment != "" {
            Some(comment.to_string())
        } else {
            None
        };

        Ok(Self {
            commit_hash,
            tree_hash,
            parent_hash,
            author_name,
            author_email,
            author_date,
            committer_name,
            committer_email,
            commit_date,
            comment,
        })
    }
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
