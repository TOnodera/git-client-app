mod fixture;
use app::git_log::GitLogUsecase;
use chrono::{DateTime, Utc};
use domain::{
    command::git_log::{GitLogCommandOption, GitLogCommandTrait},
    service::git_log::GitLogServiceTrait,
    types::Result,
    value::CommitInfo,
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

    let mut expecteds = Vec::new();
    // 最初のコミット
    expecteds.push(CommitInfo::new(
        Some("99eeab74750f9d3098d3bef3681145e257545c9f".to_string()),
        Some("f6adf0012fd2a27663af2e94b36de41cbba5823e".to_string()),
        None,
        Some("t.onodera".to_string()),
        Some("t.onodera.1.2.1.5@gmail.com".to_string()),
        DateTime::parse_from_rfc2822("Tue, 24 Jan 2023 21:19:56 +0900")?.with_timezone(&Utc),
        Some("t.onodera".to_string()),
        Some("t.onodera.1.2.1.5@gmail.com".to_string()),
        DateTime::parse_from_rfc2822("Tue, 24 Jan 2023 21:19:56 +0900")?.with_timezone(&Utc),
        Some("最初のコミット".to_string()),
    ));
    assert_eq!(expecteds[0], results[results.len() - 1]);

    // 直近のコミット
    expecteds.push(CommitInfo::new(
        Some("0cbdaf6b5211d36df10a578b621e63eab3bc52a4".to_string()),
        Some("6bec089d8d748eb1a800955bac06597f1fd22414".to_string()),
        Some("19c871de65be6334404cb7d26c7b7bed04ad8c00".to_string()),
        Some("t.onodera".to_string()),
        Some("t.onodera.1.2.1.5@gmail.com".to_string()),
        DateTime::parse_from_rfc2822("Fri, 27 Jan 2023 22:34:51 +0900")?.with_timezone(&Utc),
        Some("t.onodera".to_string()),
        Some("t.onodera.1.2.1.5@gmail.com".to_string()),
        DateTime::parse_from_rfc2822("Fri, 27 Jan 2023 22:34:51 +0900")?.with_timezone(&Utc),
        Some("git logの具体的なパーザーを実装".to_string()),
    ));
    assert_eq!(expecteds[1], results[0]);

    Ok(())
}

#[test]
fn 指定したhashからN個のコミットを取得できる() {
    todo!()
}
