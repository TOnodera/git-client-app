mod fixture;

use app::git_log::GitLogUsecase;
use chrono::{DateTime, Utc};
use domain::{
    command::git_log::{GitLogCommandOption, GitLogCommandTrait},
    error::DomainError,
    service::git_log::GitLogServiceTrait,
    types::Result,
    value::CommitInfo,
};
use fixture::git_log_fixture::{
    GitLogCommandErrorFixture1, GitLogCommandErrorFixture2, GitLogCommandNormalFixture,
};
use infrastructure::implement::git_log::service::GitLogCommandService;
#[test]
fn 指定したhashから遡れるコミットをすべて取得できる() -> Result<()> {
    let command = GitLogCommandNormalFixture::new();
    let service = GitLogCommandService::new();
    let usecase = GitLogUsecase::new(command, service);
    let results = usecase.run(GitLogCommandOption::new("git_dir", "hash", 0))?;

    assert_eq!(results.len(), 12);

    let mut expecteds = Vec::new();
    // 最初のコミット
    expecteds.push(CommitInfo::new(
        "99eeab74750f9d3098d3bef3681145e257545c9f",
        "f6adf0012fd2a27663af2e94b36de41cbba5823e",
        "",
        "t.onodera",
        "t.onodera.1.2.1.5@gmail.com",
        "Tue, 24 Jan 2023 21:19:56 +0900",
        "t.onodera",
        "t.onodera.1.2.1.5@gmail.com",
        "Tue, 24 Jan 2023 21:19:56 +0900",
        "最初のコミット",
    )?);
    assert_eq!(expecteds[0], results[results.len() - 1]);

    // 直近のコミット
    expecteds.push(CommitInfo::new(
        "0cbdaf6b5211d36df10a578b621e63eab3bc52a4",
        "6bec089d8d748eb1a800955bac06597f1fd22414",
        "19c871de65be6334404cb7d26c7b7bed04ad8c00",
        "t.onodera",
        "t.onodera.1.2.1.5@gmail.com",
        "Fri, 27 Jan 2023 22:34:51 +0900",
        "t.onodera",
        "t.onodera.1.2.1.5@gmail.com",
        "Fri, 27 Jan 2023 22:34:51 +0900",
        "git logの具体的なパーザーを実装",
    )?);
    assert_eq!(expecteds[1], results[0]);

    Ok(())
}

#[test]
fn author_dateとcommit_dateは入力必須とする() -> Result<()> {
    // author_dateがない場合
    let command = GitLogCommandErrorFixture1::new();
    let service = GitLogCommandService::new();
    let usecase = GitLogUsecase::new(command, service);
    let results = usecase.run(GitLogCommandOption::new("git_dir", "hash", 0));
    match results {
        Ok(_) => assert!(false),
        Err(err) => {
            let exp = err.downcast::<DomainError>();
            assert_eq!(exp.unwrap(), DomainError::ValidationError);
        }
    }

    // commit_dateがない場合
    let command = GitLogCommandErrorFixture2::new();
    let service = GitLogCommandService::new();
    let usecase = GitLogUsecase::new(command, service);
    let results = usecase.run(GitLogCommandOption::new("git_dir", "hash", 0));
    match results {
        Ok(_) => assert!(false),
        Err(err) => {
            let exp = err.downcast::<DomainError>();
            assert_eq!(exp.unwrap(), DomainError::ValidationError);
        }
    }

    Ok(())
}

#[test]
fn 指定したhashからN個のコミットを取得できる() {
    todo!()
}
