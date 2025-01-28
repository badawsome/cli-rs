use faststr::FastStr;

use super::{make_output, Output};

pub struct Context {
    pub output: Output,
    pub abort_reason: Option<FastStr>,
}

impl Context {
    pub fn new_stdout() -> Self {
        Self {
            output: make_output(tokio::io::stdout()),
            abort_reason: None,
        }
    }

    pub fn new_with_output<O: tokio::io::AsyncWrite + Send + 'static>(o: O) -> Self {
        Self {
            output: make_output(o),
            abort_reason: None,
        }
    }
}
