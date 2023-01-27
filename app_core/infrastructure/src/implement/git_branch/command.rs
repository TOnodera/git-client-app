use domain::{
    command::git_branch::{
        GitBranchCommandInputTrait, GitBranchCommandOptionTrait, GitBranchCommandOutput,
        GitBranchCommandTrait,
    },
    types::Result,
};
struct GitBranchCommand;
impl GitBranchCommandTrait for GitBranchCommand {
    fn new() -> Self {
        Self
    }
    fn execute(
        &self,
        input: Option<impl GitBranchCommandInputTrait>,
        option: Option<impl GitBranchCommandOptionTrait>,
    ) -> GitBranchCommandOutput {
        todo!()
    }
}
