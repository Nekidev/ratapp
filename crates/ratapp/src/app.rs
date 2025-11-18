//! The main application loop and event handling.

use std::collections::VecDeque;

use ratatui::crossterm::event::{self, Event};
use tokio::sync::mpsc;

use crate::{
    navigation::{Action, Navigator},
    screen::ScreenState,
};

/// The main application struct that runs the event loop and manages screens.
///
/// To create an instance of `App`, use the [`App::new()`] method with your
/// [`Screens`](crate::Screens)-derived type.
///
/// ```ignore
/// let mut app = App::new();
/// ```
///
/// Then, to run the application, call the asynchronous [`App::run()`] method:
///
/// ```ignore
/// let mut app = App::new();
///
/// app.run::<MyScreens>().await?;
/// ```
///
/// You can also create an `App` instance with a global application state using the
/// [`App::with_state()`] method:
///
/// ```ignore
/// let initial_state = MyAppState { /* initialize your state here */ };
/// let mut app = App::with_state(initial_state);
/// ```
///
/// To use application state within your screens, implement the
/// [`ScreenWithState`](crate::ScreenWithState) trait for your screens instead of the
/// [`Screen`](crate::Screen) trait. This allows your screens to access and modify the shared
/// application state.
pub struct App<T = ()> {
    events: mpsc::UnboundedReceiver<Event>,
    state: T,
}

impl App<()> {
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

        Self {
            events: events_rx,
            state: (),
        }
    }
}

impl<T> App<T> {
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

        Self {
            events: events_rx,
            state,
        }
    }

    /// Runs the main application loop, handling events and screen rendering.
    ///
    /// Returns:
    /// `std::io::Result<()>` - Result of the application run.
    pub async fn run<S>(&mut self) -> std::io::Result<()>
    where
        S: ScreenState<T>,
    {
        let mut terminal = ratatui::init();

        let mut screens = VecDeque::from([S::default()]);

        let (events_tx, mut events_rx) = mpsc::unbounded_channel();
        let navigator = Navigator::new(events_tx);

        screens
            .back_mut()
            .unwrap()
            .on_enter(navigator.clone(), &mut self.state)
            .await;

        let mut draw = true;

        loop {
            let screen = screens.back_mut().expect("No screen in the stack!");

            if draw {
                terminal
                    .draw(|frame| screen.draw(frame, &self.state))
                    .inspect_err(|_| {
                        ratatui::restore();
                    })?;

                draw = false;
            }

            tokio::select! {
                Some(event) = self.events.recv() => {
                    if let Event::Resize(_, _) = event {
                        draw = true;
                    }

                    screen.on_event(event, navigator.clone(), &mut self.state).await;
                },
                Some(action) = events_rx.recv() => {
                    match action {
                        Action::Push(id) => {
                            screen.on_pause(navigator.clone(), &mut self.state).await;

                            let mut screen = S::new(id);
                            screen.on_enter(navigator.clone(), &mut self.state).await;

                            screens.push_back(screen);

                            draw = true;
                        }
                        Action::Replace(id) => {
                            let mut old_screen = screens.pop_back().unwrap();
                            old_screen.on_exit(navigator.clone(), &mut self.state).await;

                            let mut new_screen = S::new(id);
                            new_screen.on_enter(navigator.clone(), &mut self.state).await;
                            screens.push_back(new_screen);

                            draw = true;
                        }
                        Action::Back => {
                            if screens.len() > 1 {
                                let mut old_screen = screens.pop_back().unwrap();
                                old_screen.on_exit(navigator.clone(), &mut self.state).await;

                                let current_screen = screens.back_mut().unwrap();
                                current_screen.on_resume(navigator.clone(), &mut self.state).await;

                                draw = true;
                            }
                        }
                        Action::Clear => {
                            let current_screen = screens.pop_back().unwrap();

                            while let Some(mut old_screen) = screens.pop_back() {
                                old_screen.on_exit(navigator.clone(), &mut self.state).await;
                            }

                            screens.push_back(current_screen);
                        }
                        Action::Restart => {
                            while let Some(mut old_screen) = screens.pop_back() {
                                old_screen.on_exit(navigator.clone(), &mut self.state).await;
                            }

                            let mut new_screen = S::default();
                            new_screen.on_enter(navigator.clone(), &mut self.state).await;
                            screens.push_back(new_screen);

                            draw = true;
                        }
                        Action::Exit => {
                            while let Some(mut old_screen) = screens.pop_back() {
                                old_screen.on_exit(navigator.clone(), &mut self.state).await;
                            }

                            break;
                        }
                        Action::Rerender => {
                            draw = true;
                        }
                    }
                }
            }
        }

        ratatui::restore();

        Ok(())
    }
}

impl<T> Default for App<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::with_state(T::default())
    }
}
