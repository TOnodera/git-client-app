use domain::{
    command::git_log::{GitLogCommandInputTrait, GitLogCommandOptionTrait, GitLogCommandTrait},
    service::git_log::GitLogServiceTrait,
    types::Result,
    value::CommitInfo,
};

pub struct GitLogUsecase<T: GitLogCommandTrait, I: GitLogServiceTrait> {
    command: T,
    service: I,
}

impl<T: GitLogCommandTrait, I: GitLogServiceTrait> GitLogUsecase<T, I> {
    pub fn new(command: T, service: I) -> Self {
        Self { command, service }
    }
    pub fn run(
        &self,
        input: Option<impl GitLogCommandInputTrait>,
        option: Option<impl GitLogCommandOptionTrait>,
    ) -> Result<Vec<CommitInfo>> {
        let command_output = self.command.execute(input, option)?;
        self.service.parse(&command_output)
    }
}
