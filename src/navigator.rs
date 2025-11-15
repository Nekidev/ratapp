use std::sync::Arc;

use tokio::sync::{Mutex, watch};

#[derive(Clone)]
pub struct Navigator<S> {
    pub(crate) inner: Arc<Mutex<NavigatorInner<S>>>,
}

pub(crate) struct NavigatorInner<S> {
    pub next_screen: Option<S>,
    pub should_exit: bool,
    pub rerenders: watch::Sender<()>,
}

impl<S> Navigator<S> {
    pub fn new(rerenders: watch::Sender<()>) -> Self {
        Navigator {
            inner: Arc::new(Mutex::new(NavigatorInner {
                next_screen: None,
                should_exit: false,
                rerenders,
            })),
        }
    }

    pub async fn goto(&self, screen: S) {
        self.inner.lock().await.next_screen = Some(screen);
    }

    pub async fn exit(&self) {
        self.inner.lock().await.should_exit = true;
    }

    pub async fn rerender(&self) {
        let inner = self.inner.lock().await;
        let _ = inner.rerenders.send(());
    }
}
