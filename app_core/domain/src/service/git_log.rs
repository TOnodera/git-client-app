use crate::{types::Result, value::CommitInfo};

pub trait GitLogServiceTrait {
    fn new() -> Self;
    fn parse(&self, input: &Vec<u8>) -> Result<Vec<CommitInfo>>;
}
