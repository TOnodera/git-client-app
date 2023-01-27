use crate::types::Result;

///
/// git branch コマンド
///

//入力型
pub struct GitBranchCommandInput {
    git_dir: String,
}
pub trait GitBranchCommandInputTrait {
    fn git_dir(&self) -> String;
}
impl GitBranchCommandInputTrait for GitBranchCommandInput {
    fn git_dir(&self) -> String {
        self.git_dir.to_string()
    }
}
// 出力型
pub trait GitBranchCommandOutputTrait {}
impl GitBranchCommandOutputTrait for Result<Vec<u8>> {}
pub type GitBranchCommandOutput = Result<Vec<u8>>;
// コマンドオプション型
pub struct GitBranchCommandOption;
pub trait GitBranchCommandOptionTrait {}
impl GitBranchCommandOptionTrait for GitBranchCommandOption {}

pub trait GitBranchCommandTrait {
    fn new() -> Self;
    fn execute(
        &self,
        input: Option<impl GitBranchCommandInputTrait>,
        option: Option<impl GitBranchCommandOptionTrait>,
    ) -> GitBranchCommandOutput;
}
