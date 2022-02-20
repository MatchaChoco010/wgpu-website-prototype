use instant::Duration;
#[cfg(target_arch = "wasm32")]
use instant::Instant;
use std::future::Future;
use std::sync::Arc;

#[derive(Clone)]
pub struct Runtime {
    #[cfg(not(target_arch = "wasm32"))]
    runtime: Arc<tokio::runtime::Runtime>,
    #[cfg(target_arch = "wasm32")]
    runtime: step_runtime::Runtime,
    #[cfg(target_arch = "wasm32")]
    previous_time: Instant,
}
impl Runtime {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        );
        Self { runtime }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        Self {
            runtime: step_runtime::Runtime::new(),
            previous_time: instant::Instant::now(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        self.runtime.spawn(future);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static) {
        self.runtime.spawn(future);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn block_on(&self, future: impl Future<Output = ()>) {
        self.runtime.block_on(future);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn delay(&self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn delay(&self, duration: Duration) {
        self.runtime.delay(duration).await;
    }

    #[cfg(target_arch = "wasm32")]
    pub fn step(&mut self) {
        let now = instant::Instant::now();
        self.runtime.step(now - self.previous_time);
        self.previous_time = now;
    }
}
