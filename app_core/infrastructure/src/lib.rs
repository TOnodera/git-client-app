
use domain::command::command::{GitBranchCommandTrait, GitBranchCommandOutput, GitBranchCommandInputTrait, GitBranchCommandOptionTrait};


struct GitBranchCommand;
impl GitBranchCommandTrait for GitBranchCommand{
    fn new() -> Self {
        Self
    }
    fn execute(&self, input: Option<impl GitBranchCommandInputTrait>, option: Option<impl domain::command::command::GitBranchCommandOptionTrait>) -> GitBranchCommandOutput {
        todo!()
    }
}