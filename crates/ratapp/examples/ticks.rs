use ratapp::{App, Navigator, Screen, Screens};
use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    text::Text,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.run::<AppScreens>().await.unwrap();
}

#[derive(Screens)]
enum AppScreens {
    Home(TickBasedScreen),
}

impl Default for AppScreens {
    fn default() -> Self {
        AppScreens::Home(TickBasedScreen::default())
    }
}

fn get_tick(tick: usize) -> char {
    match tick % 4 {
        0 => '-',
        1 => '\\',
        2 => '|',
        3 => '/',
        _ => unreachable!(),
    }
}

#[derive(Default)]
struct TickBasedScreen {
    tick: usize,
}

impl Screen<ScreenID> for TickBasedScreen {
    fn draw(&mut self, frame: &mut Frame) {
        let text = Text::from(format!(
            "{} Dummy loading... (press Q to exit)",
            get_tick(self.tick)
        ));

        frame.render_widget(text, frame.area());
    }

    async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
        if let Event::Key(key_event) = event
            && key_event.code == KeyCode::Char('q')
        {
            navigator.exit();
        }
    }

    async fn task(&mut self, navigator: Navigator<ScreenID>) {
        tokio::time::sleep(Duration::from_millis(200)).await;
        self.tick = self.tick.wrapping_add(1);
        navigator.redraw();
    }
}
