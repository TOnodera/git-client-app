use crate::types::Result;

///
/// git branch コマンド
///

//入力型
pub struct GitBranchCommandInput;
pub trait GitBranchCommandInputTrait{}
impl GitBranchCommandInputTrait for GitBranchCommandInput{}
// 出力型
pub trait GitBranchCommandOutputTrait{}
impl GitBranchCommandOutputTrait for Result<Vec<u8>>{}
pub type GitBranchCommandOutput = Result<Vec<u8>>;
// コマンドオプション型
pub struct GitBranchCommandOption;
pub trait GitBranchCommandOptionTrait{}
impl GitBranchCommandOptionTrait for GitBranchCommandOption{}

pub trait GitBranchCommandTrait {
    fn new() -> Self;
    fn execute(&self, input: Option<impl GitBranchCommandInputTrait>, option: Option<impl GitBranchCommandOptionTrait>) -> GitBranchCommandOutput;
}