pub type Output = std::pin::Pin<Box<dyn tokio::io::AsyncWrite + Send>>;

pub fn make_output<T: tokio::io::AsyncWrite + Send + 'static>(t: T) -> Output {
    Box::pin(t)
}
