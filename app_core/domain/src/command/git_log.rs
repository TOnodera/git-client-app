use crate::types::Result;

///
/// git log コマンド
///

// 出力型
pub type GitLogCommandOutput = Result<Vec<u8>>;
// コマンドオプション型
pub struct GitLogCommandOption {
    pub git_dir: String,
    pub hash: String,
    pub num: u32,
}

pub trait GitLogCommandTrait {
    fn new() -> Self;
    fn execute(&self, option: GitLogCommandOption) -> GitLogCommandOutput;
}
