use ratatui::{Frame, crossterm::event::Event};

use crate::navigation::Navigator;

/// The state of the application screen.
///
/// All methods but `new()` are maps to the underlying active [`Screen`]'s methods. Since it's
/// boilerplate-y to implement those methods just to forward them to the active screen, prefer the
/// [`Screens`](crate::Screens) derive macro to automatically generate this trait implementation.
///
/// In case you wanted to implement it manually, map all methods to the active screen like so:
///
/// ```ignore
/// impl ScreenState for AppScreens {
///     type ID = ScreenID;
///
///     fn new(id: Self::ID) -> Self {
///         match id {
///             ScreenID::First => AppScreens::First(FirstScreen::default()),
///             ScreenID::Second => AppScreens::Second(SecondScreen::default()),
///         }
///     }
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
/// Once that's done, you'll also need to create a `ScreenID` enum to identify the screens, like
/// so:
///
/// ```rust
/// enum ScreenID {
///     First,
///     Second,
/// }
/// ```
///
/// And that's it! You can now use your `ScreenState` implementation with the [`App`](crate::App)
/// struct to run your application.
pub trait ScreenState<S = ()>: Default {
    type ID: Copy;

    fn new(id: Self::ID) -> Self;
    fn draw(&mut self, frame: &mut Frame, state: &S);
    async fn on_event(&mut self, event: Event, navigator: Navigator<Self::ID>, state: &mut S);
    async fn on_enter(&mut self, navigator: Navigator<Self::ID>, state: &mut S);
    async fn on_exit(&mut self, navigator: Navigator<Self::ID>, state: &mut S);
    async fn on_pause(&mut self, navigator: Navigator<Self::ID>, state: &mut S);
    async fn on_resume(&mut self, navigator: Navigator<Self::ID>, state: &mut S);
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
    async fn on_event(&mut self, event: Event, navigator: Navigator<ID>);

    /// Called when the screen is entered.
    ///
    /// This method is called when the screen is first displayed.
    ///
    /// It can be used to initialize the screen state or perform any setup tasks.
    ///
    /// Arguments:
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    async fn on_enter(&mut self, navigator: Navigator<ID>) {}

    /// Called when the screen is exited.
    ///
    /// This method is called when the screen is about to be replaced by another screen.
    ///
    /// It can be used to clean up the screen state or perform any teardown tasks.
    ///
    /// Arguments:
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    async fn on_exit(&mut self, navigator: Navigator<ID>) {}

    /// Called when the screen is paused (sent to the background because of [`Navigator::push()`]).
    ///
    /// This method can be used to pause any ongoing tasks or animations.
    ///
    /// Arguments:
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    async fn on_pause(&mut self, navigator: Navigator<ID>) {}

    /// Called when the screen is resumed (brought back to the foreground by [`Navigator::back()`]
    /// or similar).
    ///
    /// This method can be used to resume any paused tasks or animations.
    ///
    /// Arguments:
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    async fn on_resume(&mut self, navigator: Navigator<ID>) {}
}

/// A screen in the application with access to global application state.
pub trait ScreenWithState<ID, State> {
    /// Draws the screen.
    ///
    /// Even though this method takes `&mut self`, it's usually not a good idea to modify the
    /// screen state here, as it can lead to unexpected behavior.
    ///
    /// Arguments:
    /// * `frame` - The frame to draw on.
    /// * `state` - The state of the application.
    fn draw(&mut self, frame: &mut Frame, state: &State);

    /// Handles a terminal event.
    ///
    /// Every time an event is received, this method is called with the event and a navigator. Once
    /// it returns, the screen is rerendered.
    ///
    /// Arguments:
    /// * `event` - The terminal event to handle.
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    /// * `state` - The state of the application.
    async fn on_event(&mut self, event: Event, navigator: Navigator<ID>, state: &mut State);

    /// Called when the screen is entered.
    ///
    /// This method is called when the screen is first displayed.
    ///
    /// It can be used to initialize the screen state or perform any setup tasks.
    ///
    /// Arguments:
    /// * `state` - The state of the application.
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    async fn on_enter(&mut self, navigator: Navigator<ID>, state: &mut State) {}

    /// Called when the screen is exited.
    ///
    /// This method is called when the screen is about to be replaced by another screen.
    ///
    /// It can be used to clean up the screen state or perform any teardown tasks.
    ///
    /// Arguments:
    /// * `state` - The state of the application.
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    async fn on_exit(&mut self, navigator: Navigator<ID>, state: &mut State) {}

    /// Called when the screen is paused (sent to the background because of [`Navigator::push()`]).
    ///
    /// This method can be used to pause any ongoing tasks or animations.
    ///
    /// Arguments:
    /// * `state` - The state of the application.
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    async fn on_pause(&mut self, navigator: Navigator<ID>, state: &mut State) {}

    /// Called when the screen is resumed (brought back to the foreground by [`Navigator::back()`]
    /// or similar).
    ///
    /// This method can be used to resume any paused tasks or animations.
    ///
    /// Arguments:
    /// * `state` - The state of the application.
    /// * `navigator` - The navigator to navigate between screens or request rerenders.
    async fn on_resume(&mut self, navigator: Navigator<ID>, state: &mut State) {}
}

// All [`Screen`]s are a [`ScreenWithState`] under the hood.
impl<ID, T, S> ScreenWithState<ID, T> for S
where
    S: Screen<ID>,
{
    fn draw(&mut self, frame: &mut Frame, _state: &T) {
        self.draw(frame);
    }

    async fn on_event(&mut self, event: Event, navigator: Navigator<ID>, _state: &mut T) {
        self.on_event(event, navigator).await;
    }

    async fn on_enter(&mut self, navigator: Navigator<ID>, _state: &mut T) {
        self.on_enter(navigator).await;
    }

    async fn on_exit(&mut self, navigator: Navigator<ID>, _state: &mut T) {
        self.on_exit(navigator).await;
    }

    async fn on_pause(&mut self, navigator: Navigator<ID>, _state: &mut T) {
        self.on_pause(navigator).await;
    }

    async fn on_resume(&mut self, navigator: Navigator<ID>, _state: &mut T) {
        self.on_resume(navigator).await;
    }
}
