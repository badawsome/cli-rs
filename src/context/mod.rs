mod output;
use faststr::FastStr;
pub use output::*;

mod inner;

#[derive(Clone)]
pub struct Context(pub std::sync::Arc<tokio::sync::Mutex<inner::Context>>);

impl Context {
    pub fn new_stdout() -> Self {
        Self(std::sync::Arc::new(tokio::sync::Mutex::new(
            inner::Context::new_stdout(),
        )))
    }

    pub fn new_with_output<O: tokio::io::AsyncWrite + Send + 'static>(o: O) -> Self {
        Self(std::sync::Arc::new(tokio::sync::Mutex::new(
            inner::Context::new_with_output(o),
        )))
    }

    pub async fn set_abort_reason(&self, reason: FastStr) {
        self.0.as_ref().lock().await.abort_reason = Some(reason);
    }
}
