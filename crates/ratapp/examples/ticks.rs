use ratapp::{App, Navigator, Screen, Screens, State};
use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    text::Text,
};
use std::time::Duration;
use tokio::task::JoinHandle;

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
    tick: State<usize>,
    ticker: Option<JoinHandle<()>>,
}

impl Screen<ScreenID> for TickBasedScreen {
    fn draw(&mut self, frame: &mut Frame) {
        let text = Text::from(format!(
            "{} Dummy loading... (press Q to exit)",
            get_tick(*self.tick.get())
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

    async fn on_enter(&mut self, navigator: Navigator<ScreenID>) {
        let tick = self.tick.clone();

        self.ticker = Some(tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(200)).await;
                *tick.get() += 1;
                navigator.rerender();
            }
        }));
    }

    async fn on_exit(&mut self, _navigator: Navigator<ScreenID>) {
        if let Some(ticker) = self.ticker.take() {
            ticker.abort();
        }
    }

    async fn on_resume(&mut self, navigator: Navigator<ScreenID>) {
        self.on_enter(navigator).await;
    }

    async fn on_pause(&mut self, navigator: Navigator<ScreenID>) {
        self.on_exit(navigator).await;
    }
}
