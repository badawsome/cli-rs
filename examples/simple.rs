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
        cx.write_all(format!("hi {}", self.name).as_bytes())
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
    let buf = vec![];
    let w = tokio::io::BufWriter::new(buf);
    let w = std::sync::Arc::new(tokio::sync::Mutex::new(w));
    let _cx = dispatcher
        .dispatch(&["rt", "--debug", "hi", "--name", "me"], w.clone())
        .await?;
    w.lock().await.flush().await?;
    let output = String::from_utf8(w.lock().await.get_ref().to_owned())?;
    debug_assert_eq!(output, "hi me");
    println!("{}", output);
    Ok(())
}
