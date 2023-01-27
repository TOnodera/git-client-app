use crate::types::Result;

///
/// git log コマンド
///

//入力型
pub struct GitLogCommandInput;
pub trait GitLogCommandInputTrait {}
impl GitLogCommandInputTrait for GitLogCommandInput {}
// 出力型
pub trait GitLogCommandOutputTrait {}
impl GitLogCommandOutputTrait for Result<Vec<u8>> {}
pub type GitLogCommandOutput = Result<Vec<u8>>;
// コマンドオプション型
pub struct GitLogCommandOption;
pub trait GitLogCommandOptionTrait {}
impl GitLogCommandOptionTrait for GitLogCommandOption {}

pub trait GitLogCommandTrait {
    fn new() -> Self;
    fn execute(
        &self,
        input: Option<impl GitLogCommandInputTrait>,
        option: Option<impl GitLogCommandOptionTrait>,
    ) -> GitLogCommandOutput;
}
