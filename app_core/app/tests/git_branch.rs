use domain::{command::command::{GitBranchCommandTrait, GitBranchCommandInputTrait, GitBranchCommandInput, GitBranchCommandOption}, types::Result};
use fixture::git_branch_fixture::{GitBranchCommandNormalFixture};


mod fixture;

#[test]
fn git_branchコマンドを実行してBranchオブジェクトの配列を取得できる() -> Result<()>{
    let command = GitBranchCommandNormalFixture::new(); 
    let result = command.execute(None::<GitBranchCommandInput>, None::<GitBranchCommandOption>)?;
    Ok(())
}
