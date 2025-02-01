use faststr::FastStr;

mod bound;
mod inner;

pub use bound::*;
use tokio::io::AsyncWriteExt;

#[derive(Clone, Debug)]
pub struct Context(pub std::sync::Arc<tokio::sync::Mutex<inner::Context>>);

impl Context {
    pub fn new_stdout() -> Self {
        Self(std::sync::Arc::new(tokio::sync::Mutex::new(
            inner::Context::new_stdout(),
        )))
    }
    pub fn new_with_output<O: OutputBound>(o: std::sync::Arc<tokio::sync::Mutex<O>>) -> Self {
        Self(std::sync::Arc::new(tokio::sync::Mutex::new(
            inner::Context::new_with_output(o),
        )))
    }

    pub async fn set_abort_reason(&self, reason: FastStr) {
        self.0.as_ref().lock().await.abort_reason = Some(reason);
    }

    pub async fn write_all(&self, src: &[u8]) -> tokio::io::Result<()> {
        self.0.lock().await.output.lock().await.write_all(src).await
    }
}
