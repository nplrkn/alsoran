use async_std::task::JoinHandle;
use stop_token::StopSource;

pub struct TransportTasks {
    handle: JoinHandle<()>,
    stop_source: StopSource,
}

impl TransportTasks {
    pub fn new(handle: JoinHandle<()>, stop_source: StopSource) -> Self {
        Self {
            handle,
            stop_source,
        }
    }
    pub async fn graceful_shutdown(self) {
        drop(self.stop_source);
        self.handle.await
    }
}
