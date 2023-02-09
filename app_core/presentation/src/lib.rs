use std::{error, result};

use app::git_branch::GitBranchUsecase;
use domain::{
    command::git_branch::{GitBranchCommandOption, GitBranchCommandTrait},
    service::git_branch::GitBranchServiceTrait,
    types::Result,
    value::{Branch, CommitInfo, Env},
};
use infrastructure::implement::git_branch::{
    command::GitBranchCommand, service::GitBranchCommandService,
};

trait GitCommandTrait {
    fn new(git_dir: &str, is_test: bool) -> Self;
    fn get_branches(&self) -> ApiResult<Vec<Branch>>;
    fn get_commits(&self, head: &str) -> ApiResult<Vec<CommitInfo>>;
}

pub struct ApiResult<T> {
    pub is_success: bool,
    pub result: Result<T>,
    pub error_message: Option<String>,
}
impl ApiResult<Vec<Branch>> {
    pub fn new(is_success: bool, result: Result<Vec<Branch>>, error_message: Option<&str>) -> Self {
        let error_message = if let Some(error_message) = error_message {
            Some(error_message.to_string())
        } else {
            None
        };
        Self {
            is_success,
            result,
            error_message,
        }
    }
}
impl ApiResult<Vec<CommitInfo>> {
    pub fn new(
        is_success: bool,
        result: Result<Vec<CommitInfo>>,
        error_message: Option<&str>,
    ) -> Self {
        let error_message = if let Some(error_message) = error_message {
            Some(error_message.to_string())
        } else {
            None
        };
        Self {
            is_success,
            result,
            error_message,
        }
    }
}
pub struct GitCommand {
    env: Env,
}
impl GitCommandTrait for GitCommand {
    fn new(git_dir: &str, is_test: bool) -> Self {
        let env = Env {
            git_dir: git_dir.to_string(),
            is_test,
        };
        Self { env }
    }

    fn get_branches(&self) -> ApiResult<Vec<Branch>> {
        if self.env.is_test {}
        let command = GitBranchCommand::new();
        let service = GitBranchCommandService::new();
        let usecase = GitBranchUsecase::new(command, service);
        let result = usecase.run(GitBranchCommandOption::new(&self.env.git_dir));
        if result.is_err() {
            return ApiResult::<Vec<Branch>>::new(
                false,
                result,
                Some("ブランチの取得に失敗しました。"),
            );
        }
        ApiResult::<Vec<Branch>>::new(true, result, None)
    }

    fn get_commits(&self, head: &str) -> ApiResult<Vec<CommitInfo>> {
        if self.env.is_test {}
        todo!()
    }
}
