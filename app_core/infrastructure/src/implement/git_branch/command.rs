use domain::{command::command::{GitBranchCommandTrait, GitBranchCommandOutput, GitBranchCommandInputTrait, GitBranchCommandOptionTrait}, types::Result};
struct GitBranchCommand;
impl GitBranchCommandTrait for GitBranchCommand{
    fn new() -> Self {
        Self
    }
    fn execute(&self, input: Option<impl GitBranchCommandInputTrait>, option: Option<impl GitBranchCommandOptionTrait>) -> GitBranchCommandOutput {
        todo!()
    }
}