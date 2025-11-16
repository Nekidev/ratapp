//! Navigation between screens, exiting the app, and requesting rerenders.
//! 
//! It exposes the [`Navigator`] struct, which is passed to each screen to allow them to
//! navigate between each other, request rerenders, or exit the application.
//! 
//! Check out the documentation of the [`Navigator`] for more information.

use std::sync::Arc;

use tokio::sync::{Mutex, watch};

/// Allows screens to navigate between each other, request rerenders, or exit the application.
/// 
/// It has 3 main functions:
/// - [`goto(ScreenID)`](Navigator::goto): Switch to another screen.
/// - [`exit()`](Navigator::exit): Exit the application.
/// - [`rerender()`](Navigator::rerender): Request a rerender of the current screen.
/// 
/// They're all asynchronous functions, so you need to `.await` them.
/// 
/// The inner state of the navigator is an `Arc<Mutex<...>>`, so it can be cloned and shared
/// between tasks safely.
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
    pub(crate) fn new(rerenders: watch::Sender<()>) -> Self {
        Navigator {
            inner: Arc::new(Mutex::new(NavigatorInner {
                next_screen: None,
                should_exit: false,
                rerenders,
            })),
        }
    }

    /// Navigate to another screen identified by `screen`.
    /// 
    /// Arguments:
    /// * `screen` - The ID of the screen to navigate to.
    pub async fn goto(&self, screen: S) {
        self.inner.lock().await.next_screen = Some(screen);
    }

    /// Exit the application.
    pub async fn exit(&self) {
        self.inner.lock().await.should_exit = true;
    }

    /// Request a rerender of the current screen.
    #[deprecated(since = "0.4.0", note = "Use Screen::rerender() and ScreenWithState::rerender() instead.")]
    pub async fn rerender(&self) {
        let inner = self.inner.lock().await;
        let _ = inner.rerenders.send(());
    }
}
