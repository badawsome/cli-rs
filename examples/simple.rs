use clap::{arg, Parser};
use faststr::FastStr;

use clap_runner::{CallbackStatus, Context, Dispatcher, Error, RunnableCommand};
use tokio::io::AsyncWriteExt;

#[derive(Parser, Debug, Default)]
#[command(name = "rt")]
struct RootCommand {
    #[arg(short, long)]
    debug: bool,
}

#[async_trait::async_trait]
impl RunnableCommand for RootCommand {}

#[derive(Parser, Debug, Default)]
#[command(name = "hi")]
struct HiCommand {
    #[arg(short, long)]
    name: String,
}

#[async_trait::async_trait]
impl RunnableCommand for HiCommand {
    async fn callback(&self, cx: Context) -> Result<CallbackStatus, Error> {
        cx.0.as_ref()
            .lock()
            .await
            .output
            .write_all(format!("hi {}", self.name).as_bytes())
            .await
            .map_err(|e| Error::Unknown(anyhow::anyhow!("write failed {e}")))?;
        Ok(CallbackStatus::Abort {
            reason: FastStr::new("done"),
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dispatcher = Dispatcher::new(RootCommand::default())?.register(HiCommand::default())?;
    // dispatcher.dispatch_with_stdio().await?;
    dispatcher
        .dispatch(
            &["rt", "--debug", "hi", "--name", "me"],
            tokio::io::stdout(),
        )
        .await?;
    Ok(())
}
