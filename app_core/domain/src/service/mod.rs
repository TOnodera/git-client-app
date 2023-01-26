use crate::{types::Result, value::Branch};

pub trait GitBranchServiceTrait{
    fn new() -> Self;
    fn parse(&self, input: &Vec<u8>) -> Result<Vec<Branch>>;
}
/* 
use regex::Regex;

use crate::{types::Result, value::Branch};

pub struct GitBranchCommandService;
impl GitBranchCommandService {
    pub fn new()-> Self {
        Self
    }
    pub fn parse(input: Vec<u8>) -> Result<Vec<Branch>> {
        let output = std::str::from_utf8(&input)?;
        let re = Regex::new(r"^\*?\s+(.+?)\s+(\w+?)\s+(.+)$")?;
        let mut result = Vec::<Branch>::new();
        for line in output.lines() {
            if let Some(captures) = re.captures(line) {
                let name = &captures[1];
                let head = &captures[2];
                let is_current = Regex::new(r"^\*")?.is_match(line);
                result.push(Branch::new(name, head, is_current));
            }
        }
        Ok(result)
    }
}
*/