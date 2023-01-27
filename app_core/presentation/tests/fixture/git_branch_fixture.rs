use domain::command::git_branch::{
    GitBranchCommandInputTrait, GitBranchCommandOutput, GitBranchCommandTrait,
};

pub struct GitBranchCommandNormalFixture;
impl GitBranchCommandTrait for GitBranchCommandNormalFixture {
    fn new() -> Self {
        Self
    }
    fn execute(
        &self,
        input: Option<impl GitBranchCommandInputTrait>,
        option: Option<impl domain::command::git_branch::GitBranchCommandOptionTrait>,
    ) -> GitBranchCommandOutput {
        let output = r#"
  main            0d65e2eb24259c418682d723cd547b39fcb6e0d1 comment
  feature/branch1 0d65e2eb24259c418682d723cd547b39fcb6e0d2 comment 
  feature/branch2 0d65e2eb24259c418682d723cd547b39fcb6e0d3 comment
* develop         0d65e2eb24259c418682d723cd547b39fcb6e0d4 comment
  staging         0d65e2eb24259c418682d723cd547b39fcb6e0d5 comment
  "#
        .as_bytes()
        .to_vec();
        Ok(output)
    }
}
