//! The main application loop and event handling.

use ratatui::crossterm::event::{self, Event};
use tokio::sync::{mpsc, watch};

use crate::{navigation::Navigator, screen::ScreenState};

/// The main application struct that runs the event loop and manages screens.
///
/// To create an instance of `App`, use the [`App::new()`] method with your
/// [`Screens`](crate::Screens)-derived type.
///
/// ```ignore
/// let mut app = App::<MyScreens>::new();
/// ```
///
/// Then, to run the application, call the asynchronous [`App::run()`] method:
///
/// ```ignore
/// let mut app = App::<MyScreens>::new();
///
/// app.run().await?;
/// ```
///
/// You can also create an `App` instance with a global application state using the
/// [`App::with_state()`] method:
///
/// ```ignore
/// let initial_state = MyAppState { /* initialize your state here */ };
/// let mut app = App::<MyScreens, MyAppState>::with_state(initial_state);
/// ```
///
/// To use application state within your screens, implement the
/// [`ScreenWithState`](crate::ScreenWithState) trait for your screens instead of the
/// [`Screen`](crate::Screen) trait. This allows your screens to access and modify the shared
/// application state.
pub struct App<S, T = ()>
where
    S: ScreenState<T>,
{
    renders: (watch::Sender<()>, watch::Receiver<()>),
    events: mpsc::UnboundedReceiver<Event>,
    screen: S,
    state: T,
}

impl<S> App<S, ()>
where
    S: ScreenState<()>,
{
    /// Creates a new `App` instance with the default screen without any application state.
    ///
    /// Returns:
    /// [`App`] - A new application instance.
    pub fn new() -> Self {
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        tokio::task::spawn_blocking(move || {
            loop {
                if let Ok(event) = event::read()
                    && events_tx.send(event).is_err()
                {
                    break;
                }
            }
        });

        let (renders_tx, renders_rx) = watch::channel(());

        Self {
            renders: (renders_tx, renders_rx),
            events: events_rx,
            screen: S::default(),
            state: (),
        }
    }
}

impl<S, T> App<S, T>
where
    S: ScreenState<T>,
{
    /// Creates a new `App` instance with the default screen and provided application state.
    ///
    /// Parameters:
    /// * `state` - The initial application state.
    ///
    /// Returns:
    /// [`App`] - A new application instance.
    pub fn with_state(state: T) -> Self {
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        tokio::task::spawn_blocking(move || {
            loop {
                if let Ok(event) = event::read()
                    && events_tx.send(event).is_err()
                {
                    break;
                }
            }
        });

        let (renders_tx, renders_rx) = watch::channel(());

        Self {
            renders: (renders_tx, renders_rx),
            events: events_rx,
            screen: S::default(),
            state,
        }
    }

    /// Runs the main application loop, handling events and screen rendering.
    ///
    /// Returns:
    /// `std::io::Result<()>` - Result of the application run.
    pub async fn run(&mut self) -> std::io::Result<()> {
        let mut terminal = ratatui::init();

        loop {
            terminal
                .draw(|frame| self.screen.draw(frame, &mut self.state))
                .inspect_err(|_| {
                    ratatui::restore();
                })?;

            tokio::select! {
                Ok(_) = self.renders.1.changed() => {},
                Some(event) = self.events.recv() => {
                    let navigator = Navigator::new(self.renders.0.clone());

                    self.screen.on_event(event, &navigator, &mut self.state).await;

                    let lock = navigator.inner.lock().await;

                    if lock.should_exit {
                        self.screen.on_exit(&mut self.state).await;
                        break;
                    }

                    if let Some(id) = lock.next_screen {
                        self.screen.on_exit(&mut self.state).await;
                        self.screen.navigate(&id);
                        self.screen.on_enter(&mut self.state).await;
                    }
                },
                _ = self.screen.rerender(&mut self.state) => {}
            }
        }

        ratatui::restore();

        Ok(())
    }
}

impl<S, T> Default for App<S, T>
where
    S: ScreenState<T>,
    T: Default,
{
    fn default() -> Self {
        Self::with_state(T::default())
    }
}
