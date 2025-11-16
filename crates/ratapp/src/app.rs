//! The main application loop and event handling.

use ratatui::crossterm::event::{self, Event};
use tokio::sync::{mpsc, watch};

use crate::{navigation::Navigator, screen::ScreenState};

/// The main application struct that runs the event loop and manages screens.
///
/// To create an instance of `App`, use the [`App::new()`] method with your
/// [`Screens`](crate::Screens)-derived type.
///
/// ```
/// let mut app = App::<MyScreens>::new();
/// ```
///
/// Then, to run the application, call the asynchronous [`App::run()`] method:
///
/// ```
/// let mut app = App::<MyScreens>::new();
///
/// app.run().await?;
/// ```
pub struct App<S>
where
    S: ScreenState,
{
    renders: (watch::Sender<()>, watch::Receiver<()>),
    events: mpsc::UnboundedReceiver<Event>,
    screen: S,
}

impl<S> App<S>
where
    S: ScreenState,
{
    /// Creates a new `App` instance with the default screen.
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
                .draw(|frame| self.screen.draw(frame))
                .inspect_err(|_| {
                    ratatui::restore();
                })?;

            tokio::select! {
                Ok(_) = self.renders.1.changed() => {},
                Some(event) = self.events.recv() => {
                    let navigator = Navigator::new(self.renders.0.clone());

                    self.screen.on_event(event, &navigator).await;

                    let lock = navigator.inner.lock().await;

                    if lock.should_exit {
                        self.screen.on_exit().await;
                        break;
                    }

                    if let Some(id) = lock.next_screen {
                        self.screen.on_exit().await;
                        self.screen.navigate(&id);
                        self.screen.on_enter().await;
                    }
                },
                _ = self.screen.rerender() => {}
            }
        }

        ratatui::restore();

        Ok(())
    }
}

impl<S> Default for App<S>
where
    S: ScreenState,
{
    fn default() -> Self {
        Self::new()
    }
}
