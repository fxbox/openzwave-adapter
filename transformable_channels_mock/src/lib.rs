pub mod mpsc {
    pub trait ExtSender<T>: Send + 'static where T: Send + 'static {
        fn send(&self, t: T) -> Result<(), ()>;
    }
}
