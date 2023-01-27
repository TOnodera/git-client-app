use crate::{types::Result, value::Branch};

pub trait GitBranchServiceTrait {
    fn new() -> Self;
    fn parse(&self, input: &Vec<u8>) -> Result<Vec<Branch>>;
}
