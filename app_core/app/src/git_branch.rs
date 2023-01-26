use domain::{command::command::{GitBranchCommandTrait, GitBranchCommandInputTrait, GitBranchCommandOptionTrait, GitBranchCommandOutput}, types::Result};



pub struct GitBranchUsecase<T: GitBranchCommandTrait>
{
    command: T 
}

pub type GitBranchUsecaseOutput = Result<Vec<u8>>;

impl<T: GitBranchCommandTrait> GitBranchUsecase<T>{
    pub fn run(&self, input: Option<impl GitBranchCommandInputTrait>, option: Option<impl GitBranchCommandOptionTrait>) -> GitBranchUsecaseOutput{
        let command_output = self.command.execute(input, option)?;
        todo!()
    }
}
