use domain::{
    command::git_branch::{GitBranchCommandOption, GitBranchCommandOutput, GitBranchCommandTrait},
    error::{AppError, CommandError},
};
struct GitBranchCommand;
impl GitBranchCommandTrait for GitBranchCommand {
    fn new() -> Self {
        Self
    }
    fn execute(&self, option: GitBranchCommandOption) -> GitBranchCommandOutput {
        // git branch実行
        let result = std::process::Command::new("git")
            .arg(format!("--git-dir={}", &option.git_dir))
            .arg("branch")
            .arg("-v")
            .output()?;
        // エラーの場合は早期リターン
        if !result.status.success() {
            return Err(AppError::CommandError(CommandError::GitBranchCommandError).into());
        }
        // コマンド出力を返す
        Ok(result.stdout)
    }
}
