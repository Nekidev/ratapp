use std::future;

use ratatui::{Frame, crossterm::event::Event};

use crate::navigator::Navigator;

pub trait ScreenState: Default {
    type ID: Copy;

    fn draw(&mut self, frame: &mut Frame);
    async fn on_event(&mut self, event: Event, navigator: &Navigator<Self::ID>);
    async fn on_enter(&mut self);
    async fn on_exit(&mut self);
    async fn rerender(&mut self);
    fn navigate(&mut self, id: &Self::ID);
}

pub trait Screen<ID>: Default {
    fn draw(&mut self, frame: &mut Frame);
    async fn on_event(&mut self, event: Event, navigator: &Navigator<ID>);
    async fn on_enter(&mut self) {}
    async fn on_exit(&mut self) {}
    async fn rerender(&mut self) {
        future::pending::<()>().await;
    }
}
