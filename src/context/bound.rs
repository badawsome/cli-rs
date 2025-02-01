pub trait OutputBound: tokio::io::AsyncWrite + Sync + Send + Unpin + 'static {}

impl<T: tokio::io::AsyncWrite + Sync + Send + Unpin + 'static> OutputBound for T {}
