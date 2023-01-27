use domain::command::git_log::{
    GitLogCommandInputTrait, GitLogCommandOptionTrait, GitLogCommandOutput, GitLogCommandTrait,
};
struct GitLogCommand;
impl GitLogCommandTrait for GitLogCommand {
    fn new() -> Self {
        Self
    }
    fn execute(
        &self,
        input: Option<impl GitLogCommandInputTrait>,
        option: Option<impl GitLogCommandOptionTrait>,
    ) -> GitLogCommandOutput {
        todo!()
    }
}
