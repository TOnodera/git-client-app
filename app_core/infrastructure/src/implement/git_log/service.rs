use domain::{
    service::git_log::GitLogServiceTrait,
    types::Result,
    value::{Branch, CommitInfo},
};
use regex::Regex;

pub struct GitLogCommandService;
impl GitLogServiceTrait for GitLogCommandService {
    fn new() -> Self {
        Self
    }
    fn parse(&self, input: &Vec<u8>) -> Result<Vec<CommitInfo>> {
        todo!()
    }
}
