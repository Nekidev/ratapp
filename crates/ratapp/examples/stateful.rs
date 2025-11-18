//! The same app from examples/tutorial.rs, but using the global app state for the counter instead
//! of screen state.

use ratapp::{App, Navigator, Screen, ScreenWithState, Screens};
use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Layout},
    text::Line,
    widgets::{List, ListItem, ListState, Paragraph},
};

#[tokio::main]
async fn main() {
    let mut app = App::with_state(State { counter: 5 });

    app.run::<AppScreens>().await.unwrap();
}

struct State {
    counter: u32,
}

#[derive(Screens)]
enum AppScreens {
    Home(HomeScreen),
    List(ListScreen),
}

impl Default for AppScreens {
    fn default() -> Self {
        AppScreens::Home(HomeScreen)
    }
}

#[derive(Default)]
struct HomeScreen;

impl ScreenWithState<ScreenID, State> for HomeScreen {
    fn draw(&mut self, frame: &mut Frame, state: &State) {
        let text = Paragraph::new(vec![
            Line::from("Hello ratapp!"),
            Line::from(""),
            Line::from("This is the home screen. Welcome!"),
            Line::from(""),
            Line::from(format!("Counter: {}", state.counter)),
            Line::from(""),
            Line::from("Use the arrows up and down to update the counter."),
            Line::from("Press enter to go to the list screen."),
            Line::from("Press Q to exit."),
        ]);

        frame.render_widget(text, frame.area());
    }

    async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>, state: &mut State) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Up => {
                    state.counter = state.counter.saturating_add(1);
                }
                KeyCode::Down => {
                    state.counter = state.counter.saturating_sub(1);
                }
                KeyCode::Enter => {
                    navigator.push(ScreenID::List);
                }
                KeyCode::Char('q') => {
                    navigator.exit();
                }
                _ => {}
            }

            navigator.rerender();
        }
    }
}

struct ListScreen {
    state: ListState,
}

impl Default for ListScreen {
    fn default() -> Self {
        ListScreen {
            state: ListState::default().with_selected(Some(0)),
        }
    }
}

impl Screen<ScreenID> for ListScreen {
    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1), // Gap
            Constraint::Fill(1),
        ]);

        let [list_area, _, text_area] = layout.areas(frame.area());

        let list = List::new(vec![
            ListItem::new("1"),
            ListItem::new("2"),
            ListItem::new("3"),
        ])
        .highlight_symbol("> ");

        let text = Paragraph::new(vec![
            Line::from("Use the arrows up and down to change the selected item."),
            Line::from(""),
            Line::from("Press enter to go back home."),
            Line::from(""),
            Line::from("Press Q to exit."),
        ]);

        frame.render_stateful_widget(list, list_area, &mut self.state);
        frame.render_widget(text, text_area);
    }

    async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Up => {
                    self.state.select_previous();
                }
                KeyCode::Down => {
                    self.state.select_next();
                }
                KeyCode::PageUp => {
                    self.state.select_first();
                }
                KeyCode::PageDown => {
                    self.state.select_last();
                }
                KeyCode::Enter => {
                    navigator.back();
                }
                KeyCode::Char('q') => {
                    navigator.exit();
                }
                _ => {}
            }

            navigator.rerender();
        }
    }
}
