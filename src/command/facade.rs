use faststr::FastStr;

use super::error::*;
use super::model::*;

pub struct BoxSafeRunnableCommand(pub(crate) Box<dyn SafeRunnableCommand>);

impl BoxSafeRunnableCommand {
    pub fn self_check(&self) -> Result<()> {
        if self.0.command().get_name().contains(".") {
            return Err(Error::CommandNameIllegal {
                hint: FastStr::new("command name cannot contains '.'"),
            });
        }
        Ok(())
    }
}

impl<T: RunnableCommand> From<T> for BoxSafeRunnableCommand {
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

#[async_trait::async_trait]
pub(crate) trait SafeRunnableCommand {
    async fn callback(&self, cx: crate::context::Context) -> Result<CallbackStatus>;
    fn command(&self) -> clap::Command;
    fn bind_matches(&mut self, matches: clap::ArgMatches) -> Result<()>;
}

#[async_trait::async_trait]
impl<T: RunnableCommand> SafeRunnableCommand for T {
    async fn callback(&self, cx: crate::context::Context) -> Result<CallbackStatus> {
        self.callback(cx).await
    }
    fn command(&self) -> clap::Command {
        T::command()
    }
    fn bind_matches(&mut self, matches: clap::ArgMatches) -> Result<()> {
        self.bind_matches(matches)
    }
}

#[async_trait::async_trait]
pub trait RunnableCommand: clap::Parser + Send + Sync + 'static {
    async fn callback(&self, cx: crate::context::Context) -> Result<CallbackStatus> {
        Ok(CallbackStatus::Continue)
    }
    fn bind_matches(&mut self, mut matches: clap::ArgMatches) -> Result<()> {
        <Self as clap::FromArgMatches>::update_from_arg_matches(self, &mut matches).map_err(|e| {
            Error::Param {
                hint: FastStr::new(e.to_string()),
            }
        })
    }
}
