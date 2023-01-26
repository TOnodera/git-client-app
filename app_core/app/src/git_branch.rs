use domain::{command::command::{GitBranchCommandTrait, GitBranchCommandInputTrait, GitBranchCommandOptionTrait }, types::Result, service::GitBranchServiceTrait, value::Branch};

pub struct GitBranchUsecase<T: GitBranchCommandTrait, I: GitBranchServiceTrait>
{
    command: T,
    service: I
}

impl<T: GitBranchCommandTrait, I: GitBranchServiceTrait> GitBranchUsecase<T, I>{
    pub fn run(&self, input: Option<impl GitBranchCommandInputTrait>, option: Option<impl GitBranchCommandOptionTrait>) -> Result<Vec<Branch>>{
        let command_output = self.command.execute(input, option)?;
        self.service.parse(&command_output)
    }
}
