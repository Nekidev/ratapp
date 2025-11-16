use std::future;

use ratatui::{Frame, crossterm::event::Event};

use crate::navigation::Navigator;

/// The state of the application screen.
/// 
/// All methods but `navigate()` are maps to the underlying active [`Screen`]'s methods. Since it's
/// boilerplate-y to implement those methods just to forward them to the active screen, prefer the
/// [`Screens`](crate::Screens) derive macro to automatically generate this trait implementation.
/// 
/// In case you wanted to implement it manually, map all methods to the active screen like so:
/// 
/// ```rust
/// impl ScreenState for AppScreens {
///     type ID = ScreenID;
/// 
///     fn draw(&mut self, frame: &mut Frame) {
///         match self {
///             ScreenID::First => self.first.draw(frame),
///             ScreenID::Second => self.second.draw(frame),
///         }
///     }
/// 
///     async fn on_event(&mut self, event: Event, navigator: &Navigator<Self::ID>) {
///         match self {
///             ScreenID::First => self.first.on_event(event, navigator).await,
///             ScreenID::Second => self.second.on_event(event, navigator).await,
///         }
///     }
/// 
///     ...
/// }
/// ```
/// 
/// Repeat it for all methods. The only exception is `navigate()`, which should switch the active
/// screen to the one identified by the given ID. Your implementation could look like this:
/// 
/// ```rust
/// fn navigate(&mut self, id: &Self::ID) {
///     *self = match id {
///         ScreenID::First => AppScreens::First(Default::default()),
///         ScreenID::Second => AppScreens::Second(Default::default()),
///     };
/// }
/// ```
pub trait ScreenState: Default {
    type ID: Copy;

    fn draw(&mut self, frame: &mut Frame);
    async fn on_event(&mut self, event: Event, navigator: &Navigator<Self::ID>);
    async fn on_enter(&mut self);
    async fn on_exit(&mut self);
    async fn rerender(&mut self);
    fn navigate(&mut self, id: &Self::ID);
}

/// A screen in the application.
/// 
/// There's a few important methods to implement:
/// - [`draw()`](Screen::draw): Draws the screen.
/// - [`on_event()`](Screen::on_event): Handles an event.
/// - [`on_enter()`](Screen::on_enter): Called when the screen is entered.
/// - [`on_exit()`](Screen::on_exit): Called when the screen is exited.
/// - [`rerender()`](Screen::rerender): An async method that when returns causes the screen to
///   rerender. By default it never returns.
/// 
/// All methods are asynchronous except for `draw()`.
/// 
/// Implementors must also implement [`Default`] to provide an initial state for the screen.
/// 
/// 
pub trait Screen<ID>: Default {
    /// Draws the screen.
    /// 
    /// Even though this method takes `&mut self`, it's usually not a good idea to modify the
    /// screen state here, as it can lead to unexpected behavior.
    /// 
    /// Arguments:
    /// * `frame` - The frame to draw on.
    fn draw(&mut self, frame: &mut Frame);

    /// Handles a terminal event.
    /// 
    /// Every time an event is received, this method is called with the event and a navigator. Once
    /// it returns, the screen is rerendered.
    /// 
    /// Arguments:
    /// * `event` - The terminal event to handle.
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    async fn on_event(&mut self, event: Event, navigator: &Navigator<ID>);

    /// Called when the screen is entered.
    /// 
    /// This method is called when the screen is first displayed.
    /// 
    /// It can be used to initialize the screen state or perform any setup tasks.
    async fn on_enter(&mut self) {}

    /// Called when the screen is exited.
    /// 
    /// This method is called when the screen is about to be replaced by another screen.
    /// 
    /// It can be used to clean up the screen state or perform any teardown tasks.
    async fn on_exit(&mut self) {}

    /// An async method that when returns causes the screen to rerender on return.
    /// 
    /// It can be used to wait for some condition to be met before requesting a rerender. An
    /// example use of this is when waiting for some data to be loaded asynchronously (sent from a
    /// background task), or to make the screen re-render every certain time (ticks).
    /// 
    /// For example, to rerender every second, you could do:
    /// 
    /// ```
    /// async fn rerender(&mut self) {
    ///     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    /// }
    /// ```
    async fn rerender(&mut self) {
        future::pending::<()>().await;
    }
}
