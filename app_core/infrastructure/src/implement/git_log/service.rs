use chrono::{DateTime, Utc};
use domain::{service::git_log::GitLogServiceTrait, types::Result, value::CommitInfo};
use regex::Regex;

pub struct GitLogCommandService;
impl GitLogServiceTrait for GitLogCommandService {
    fn new() -> Self {
        Self
    }
    fn parse(&self, input: &Vec<u8>) -> Result<Vec<CommitInfo>> {
        let output = std::str::from_utf8(&input)?;

        // git log --pretty=format:"%H %T %P [%an] %ae [%ad] [%cn] %ce [%cd] [%s]"
        let re =
            Regex::new(r"^(.*) (.*) (.*) \[(.*)\] (.*) \[(.*)\] \[(.*)\] (.*) \[(.*)\] \[(.*)\]$")?;
        let mut result = Vec::<CommitInfo>::new();
        for line in output.lines() {
            if let Some(captures) = re.captures(line) {
                let commit_hash = if &captures[1] != "" {
                    Some(captures[1].to_string())
                } else {
                    None
                };

                let tree_hash = if &captures[2] != "" {
                    Some(captures[2].to_string())
                } else {
                    None
                };

                let parent_hash = if &captures[3] != "" {
                    Some(captures[3].to_string())
                } else {
                    None
                };

                let author_name = if &captures[4] != "" {
                    Some(captures[4].to_string())
                } else {
                    None
                };

                let author_email = if &captures[5] != "" {
                    Some(captures[5].to_string())
                } else {
                    None
                };

                let author_date = DateTime::parse_from_rfc2822(&captures[6])?.with_timezone(&Utc);

                let committer_name = if &captures[7] != "" {
                    Some(captures[7].to_string())
                } else {
                    None
                };

                let committer_email = if &captures[8] != "" {
                    Some(captures[8].to_string())
                } else {
                    None
                };

                let commit_date = DateTime::parse_from_rfc2822(&captures[8])?.with_timezone(&Utc);

                let comment = if &captures[9] != "" {
                    Some(captures[9].to_string())
                } else {
                    None
                };

                result.push(CommitInfo::new(
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
                ));
            }
        }
        Ok(result)
    }
}
