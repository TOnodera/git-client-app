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
                result.push(CommitInfo::new(
                    &captures[1],
                    &captures[2],
                    &captures[3],
                    &captures[4],
                    &captures[5],
                    &captures[6],
                    &captures[7],
                    &captures[8],
                    &captures[9],
                    &captures[10],
                )?);
            }
        }
        Ok(result)
    }
}
