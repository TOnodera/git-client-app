use domain::{
    command::git_branch::{GitBranchCommandOption, GitBranchCommandTrait},
    service::git_branch::GitBranchServiceTrait,
    types::Result,
    value::Branch,
};

pub struct GitBranchUsecase<T: GitBranchCommandTrait, I: GitBranchServiceTrait> {
    command: T,
    service: I,
}

impl<T: GitBranchCommandTrait, I: GitBranchServiceTrait> GitBranchUsecase<T, I> {
    pub fn new(command: T, service: I) -> Self {
        Self { command, service }
    }
    pub fn run(&self, option: GitBranchCommandOption) -> Result<Vec<Branch>> {
        let command_output = self.command.execute(option)?;
        self.service.parse(&command_output)
    }
}
