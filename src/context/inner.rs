use std::any::Any;

use super::bound::*;
use faststr::FastStr;

pub struct Context {
    pub output: std::sync::Arc<tokio::sync::Mutex<dyn OutputBound>>,
    pub abort_reason: Option<FastStr>,
}

impl Context {
    pub fn new_stdout() -> Self {
        Self {
            output: std::sync::Arc::new(tokio::sync::Mutex::new(tokio::io::stdout())),
            abort_reason: None,
        }
    }
    pub fn new_with_output<O: OutputBound>(o: std::sync::Arc<tokio::sync::Mutex<O>>) -> Self {
        Self {
            output: o,
            abort_reason: None,
        }
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("output", &self.output.type_id())
            .field("abort_reason", &self.abort_reason)
            .finish()
    }
}
