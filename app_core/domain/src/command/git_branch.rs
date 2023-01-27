use crate::types::Result;

///
/// git branch コマンド
///

// 出力型
pub type GitBranchCommandOutput = Result<Vec<u8>>;
// コマンドオプション型
pub struct GitBranchCommandOption {
    pub git_dir: String,
}
impl GitBranchCommandOption {
    pub fn new(git_dir: &str) -> Self {
        Self {
            git_dir: git_dir.to_string(),
        }
    }
}

pub trait GitBranchCommandTrait {
    fn new() -> Self;
    fn execute(&self, option: GitBranchCommandOption) -> GitBranchCommandOutput;
}
