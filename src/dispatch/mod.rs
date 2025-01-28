use crate::{context::Context, error::*};
use faststr::FastStr;
use std::collections::HashMap;

use crate::command::BoxSafeRunnableCommand;

mod inner;
mod model;

pub struct Dispatcher {
    handlers_finder: HashMap<FastStr, tokio::sync::Mutex<BoxSafeRunnableCommand>>,
    cmd: clap::Command,
}

impl Dispatcher {
    pub fn new<H: Into<BoxSafeRunnableCommand>>(h: H) -> Result<Self> {
        let h: BoxSafeRunnableCommand = h.into();
        h.self_check()?;

        let cmd = h.0.command();
        let mut handlers_finder = HashMap::new();
        handlers_finder.insert(FastStr::new(cmd.get_name()), tokio::sync::Mutex::new(h));
        Ok(Self {
            handlers_finder,
            cmd,
        })
    }

    pub fn register<H: Into<BoxSafeRunnableCommand>>(mut self, h: H) -> Result<Self> {
        let h: BoxSafeRunnableCommand = h.into();
        h.self_check()?;

        let cmd = h.0.command();
        self.handlers_finder
            .insert(FastStr::new(cmd.get_name()), tokio::sync::Mutex::new(h));
        self.cmd = self.cmd.subcommand(cmd);

        Ok(self)
    }

    pub async fn dispatch_with_stdio(self) -> Result<Context> {
        let cx = Context::new_stdout();
        let matches = self.cmd.clone().get_matches();
        self.dispatch_inner(cx, matches).await
    }

    pub async fn dispatch<I, T, O>(self, i: I, o: O) -> Result<Context>
    where
        O: tokio::io::AsyncWrite + Send + 'static,
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let cx = Context::new_with_output(o);
        let matches = self.cmd.clone().get_matches_from(i);
        self.dispatch_inner(cx, matches).await
    }
}
