use domain::{
    command::git_branch::{
        GitBranchCommandInputTrait, GitBranchCommandOptionTrait, GitBranchCommandTrait,
    },
    types::Result,
    value::Branch, service::git_branch::GitBranchServiceTrait,
};

pub struct GitBranchUsecase<T: GitBranchCommandTrait, I: GitBranchServiceTrait> {
    command: T,
    service: I,
}

impl<T: GitBranchCommandTrait, I: GitBranchServiceTrait> GitBranchUsecase<T, I> {
    pub fn new(command: T, service: I) -> Self {
        Self { command, service }
    }
    pub fn run(
        &self,
        input: Option<impl GitBranchCommandInputTrait>,
        option: Option<impl GitBranchCommandOptionTrait>,
    ) -> Result<Vec<Branch>> {
        let command_output = self.command.execute(input, option)?;
        self.service.parse(&command_output)
    }
}
