use app::git_branch::GitBranchUsecase;
use domain::{
    command::git_branch::{GitBranchCommandOption, GitBranchCommandTrait},
    service::git_branch::GitBranchServiceTrait,
    types::Result,
};
use fixture::git_branch_fixture::GitBranchCommandNormalFixture;
use infrastructure::implement::git_branch::service::GitBranchCommandService;

mod fixture;

#[test]
fn git_branchコマンドを実行してBranchオブジェクトの配列を取得できる() -> Result<()> {
    let command = GitBranchCommandNormalFixture::new();
    let service = GitBranchCommandService::new();
    let usecase = GitBranchUsecase::new(command, service);
    let result = usecase.run(GitBranchCommandOption::new("git_dir"))?;
    let main = result.get(0).unwrap();
    assert_eq!(main.is_current, false);
    assert_eq!(main.name, "main");
    assert_eq!(main.head.hash, "0d65e2eb24259c418682d723cd547b39fcb6e0d1");

    let feature_branch1 = result.get(1).unwrap();
    assert_eq!(feature_branch1.is_current, false);
    assert_eq!(feature_branch1.name, "feature/branch1");
    assert_eq!(
        feature_branch1.head.hash,
        "0d65e2eb24259c418682d723cd547b39fcb6e0d2"
    );

    let feature_branch2 = result.get(2).unwrap();
    assert_eq!(feature_branch2.is_current, false);
    assert_eq!(feature_branch2.name, "feature/branch2");
    assert_eq!(
        feature_branch2.head.hash,
        "0d65e2eb24259c418682d723cd547b39fcb6e0d3"
    );

    let develop = result.get(3).unwrap();
    assert_eq!(develop.is_current, true); // カレントブランチ
    assert_eq!(develop.name, "develop");
    assert_eq!(
        develop.head.hash,
        "0d65e2eb24259c418682d723cd547b39fcb6e0d4"
    );

    let staging = result.get(4).unwrap();
    assert_eq!(staging.is_current, false);
    assert_eq!(staging.name, "staging");
    assert_eq!(
        staging.head.hash,
        "0d65e2eb24259c418682d723cd547b39fcb6e0d5"
    );
    Ok(())
}
