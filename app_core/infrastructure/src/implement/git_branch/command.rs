use domain::{
    command::git_branch::{
        GitBranchCommandInputTrait, GitBranchCommandOptionTrait, GitBranchCommandOutput,
        GitBranchCommandTrait,
    },
    error::AppError,
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
        /*
        // git branch実行
        let result = std::process::Command::new("git")
            .arg(format!("--git-dir={}",))
            .arg("branch")
            .arg("-v")
            .output()?;
        // エラーの場合は早期リターン
        if !result.status.success() {
            return Err(Box::new(AppError::CommandError));
        }
        // コマンド出力を返す
        Ok(result.stdout)
        */
        todo!()
    }
}
