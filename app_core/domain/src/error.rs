use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("コマンドエラーが発生しました")]
    CommandError,

    #[error("git branchコマンドの実行中にエラーが発生しました")]
    GitBranchCommandError,

    #[error("バリデーションエラー")]
    ValidationError,
}
