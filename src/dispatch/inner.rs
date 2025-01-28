use faststr::FastStr;

use super::Dispatcher;
use crate::command::CallbackStatus;
use crate::context::Context;
use crate::error::*;
impl Dispatcher {
    pub(super) async fn dispatch_inner(
        self,
        cx: Context,
        mut matches: clap::ArgMatches,
    ) -> Result<Context> {
        loop {
            if let Some((name, sub_matches)) = matches.subcommand() {
                let sub_matches = sub_matches.to_owned();
                let handler = self
                    .handlers_finder
                    .get(name)
                    .ok_or_else(|| Error::CommandNotFound(FastStr::new(name)))?;
                handler.lock().await.0.bind_matches(sub_matches.clone())?;
                match handler.lock().await.0.callback(cx.clone()).await {
                    Ok(CallbackStatus::Continue) => {
                        matches = sub_matches;
                    }
                    Ok(CallbackStatus::Abort { reason }) => {
                        cx.set_abort_reason(reason).await;
                        return Ok(cx);
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            } else {
                return Ok(cx);
            }
        }
    }
}
