mod fixture;

use app::git_log::GitLogUsecase;
use domain::{
    command::git_log::{GitLogCommandOption, GitLogCommandTrait},
    service::git_log::GitLogServiceTrait,
    types::Result,
};
use fixture::git_log_fixture::GitLogCommandNormalFixture;
use infrastructure::implement::git_log::service::GitLogCommandService;
#[test]
fn 指定したhashから遡れるコミットをすべて取得できる() -> Result<()> {
    let command = GitLogCommandNormalFixture::new();
    let service = GitLogCommandService::new();
    let usecase = GitLogUsecase::new(command, service);
    let results = usecase.run(GitLogCommandOption::new("git_dir", "hash", 0))?;

    assert_eq!(results.len(), 12);

    // TODO 何個か確認する

    Ok(())
}

#[test]
fn 指定したhashからN個のコミットを取得できる() {
    todo!()
}
