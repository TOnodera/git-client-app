use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AppError {
    #[error("コマンドエラー")]
    CommandError(CommandError),

    #[error("ドメインエラー")]
    DomainError(DomainError),
}

#[derive(Error, Debug, PartialEq)]
pub enum CommandError {
    #[error("git branchコマンドの実行中にエラーが発生しました")]
    GitBranchCommandError,

    #[error("git logコマンドの実行中にエラーが発生しました")]
    GitLogCommandError,
}

#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("バリデーションエラー")]
    ValidationError,
}
