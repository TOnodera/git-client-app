use domain::{
    command::git_log::{GitLogCommandOption, GitLogCommandOutput, GitLogCommandTrait},
    error::AppError,
    error::CommandError,
};
struct GitLogCommand;
impl GitLogCommandTrait for GitLogCommand {
    fn new() -> Self {
        Self
    }
    fn execute(&self, option: GitLogCommandOption) -> GitLogCommandOutput {
        // git log --pretty=format:"%H %T %P [%an] %ae [%ad] [%cn] %ce [%cd] [%s]"
        let result = std::process::Command::new("git")
            .arg(format!("--git_dir={}", &option.git_dir))
            .arg(format!("{}", &option.hash))
            .arg("log")
            .arg("--pretty=format:'%H %T %P [%an] %ae [%ad] [%cn] %ce [%cd] [%s]'")
            .arg("--date=rfc")
            .output()?;
        // エラーの場合は早期リターン
        if !result.status.success() {
            return Err(Box::new(AppError::CommandError(
                CommandError::GitLogCommandError,
            )));
        }

        Ok(result.stdout)
    }
}
