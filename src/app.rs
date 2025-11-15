use ratatui::crossterm::event::{self, Event};
use tokio::sync::{mpsc, watch};

use crate::{navigator::Navigator, screen::ScreenState};

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
