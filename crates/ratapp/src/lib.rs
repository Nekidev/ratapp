//! A tiny framework to build multi-screen async applications with ratatui.
//!
//! # Introduction
//!
//! Ratapp is a minimal framework designed to facilitate the creation of multi-screen asynchronous
//! applications using the [`ratatui`] library. It provides essential components for screen
//! management, navigation, and asynchronous event handling, allowing developers to focus on
//! building their applications without worrying about the underlying infrastructure.
//!
//! It currently only supports [`tokio`] as the async runtime and
//! [`crossterm`](https://docs.rs/crossterm) as the terminal backend for [`ratatui`].
//!
//! > NOTE: Ratapp is still in early development. APIs may change in future releases.
//!
//! # Installation
//!
//! In your terminal, run
//!
//! ```sh
//! cargo add ratapp tokio -F tokio/macros,tokio/rt-multi-thread
//! ```
//!
//! You'll need [`tokio`] as the async runtime. The command above adds it with the macros and
//! the multi-threaded runtime support.
//!
//! # Quick Start
//!
//! > NOTE: This tutorial assumes you have previously used [`ratatui`] and have a basic
//! > understanding of how to make simple applications with it.
//!
//! A `ratapp` application consists of multiple screens implemented as structs using the [`Screen`]
//! trait, an [`App`] that handles the application and screen state, and a [`Navigator`] to move
//! across screens.
//!
//! To start with, create your `main.rs` file with an asynchronous main function:
//!
//! ```
//! #[tokio::main]
//! async fn main() {
//!     // Our code will go here.
//! }
//! ```
//!
//! ## Project Structure
//!
//! Great! Now, let's create a simple screen. To keep our screens each in their own module, let's
//! create a `screens` folder in the `src` directory, and inside it, create a file named `mod.rs`
//! and one named `home.rs`.
//!
//! ```txt
//! src/
//!   main.rs
//!   screens/
//!     mod.rs
//!     home.rs
//! ```
//!
//! Before implementing our `Home` screen, we have to set up an enum called `AppScreens` in our
//! `screens/mod.rs` file, which will hold all our screens as variants.
//!
//! ## The [`Screens`] Derive
//!
//! The `AppScreens` enum will hold all our screens as variants. It derives from [`Screens`] which
//! will write all the boilerplate code needed to make things work under the hood.
//!
//! ```ignore
//! mod home;
//!
//! use ratapp::Screens;
//!
//! #[derive(Screens)]
//! pub enum AppScreens {
//!     Home(home::HomeScreen),
//! }
//!
//! impl Default for AppScreens {
//!     fn default() -> Self {
//!         AppScreens::Home(home::HomeScreen::default())
//!     }
//! }
//! ```
//!
//! Note that we haven't implemented the `HomeScreen` struct yet; we'll do that next. The
//! [`Default`] implementation is required by `ratapp` to know which screen to display first when
//! the application starts.
//!
//! ## A Barebones [`Screen`]
//!
//! Now, let's implement our `HomeScreen` in the `screens/home.rs` file. We'll make it simple for
//! this example, just displaying a static message, but you can expand it with more complex logic
//! as needed.
//!
//! ```ignore
//! use ratapp::{Navigator, Screen};
//! use ratatui::{Frame, crossterm::event::Event};
//!
//! use crate::screens::ScreenID;
//!
//! #[derive(Default)]
//! struct HomeScreen {
//!     counter: u32,
//! }
//!
//! impl Screen<ScreenID> for HomeScreen {
//!     fn draw(&mut self, frame: &mut Frame) {
//!         // Drawing logic will go here.
//!     }
//!
//!     async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
//!         // Terminal-event-handling logic will go here.
//!     }
//! }
//! ```
//!
//! ## `ScreenID`
//!
//! Perfect! Now we have our first screen set up. There's something off though; Did you notice we
//! imported `ScreenID` from `crate::screens` even when we haven't defined any such enum? That's
//! because the `ScreenID` enum is automatically generated for us by the `#[derive(Screens)]`
//! macro we used on the `AppScreen` enum. It has the same variants as our `AppScreen` enum, but
//! without the screens inside. It looks like this:
//!
//! ```
//! #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
//! pub enum ScreenID {
//!     Home,
//! }
//! ```
//!
//! As you can see, it has the same variants as our `AppScreen` enum, but without the screens
//! inside. It's not nice nor obvious given that a struct appeared out of the blue, so we recommend
//! you to add a comment explaining this in your code.
//!
//! If you wanted to write it the explicit way, you can always swap your `ScreenID` mentions with
//! `<AppScreen as Screen>::ID`, but that would be quite verbose. It's up to you!
//!
//! ## A Simple Screen
//!
//! Now yes, let's draw something. Our screen right now is empty, so let's add some content to it.
//! Let's update our `draw` method to draw a simple paragraph.
//!
//! ```
//! # use ratapp::{App, Navigator, Screen};
//! use ratatui::{Frame, crossterm::event::Event, widgets::Paragraph, text::Line};
//!
//! # enum ScreenID {}
//! # #[derive(Default)]
//! # struct HomeScreen { counter: u32 }
//! #
//! impl Screen<ScreenID> for HomeScreen {
//!     fn draw(&mut self, frame: &mut Frame) {
//!         let text = Paragraph::new(
//!             vec![
//!                 Line::from("Hello ratapp!"),
//!                 Line::from(""),
//!                 Line::from("This is the home screen. Welcome!"),
//!                 Line::from(""),
//!                 Line::from(format!("Counter: {}", self.counter)),
//!                 Line::from(""),
//!                 Line::from("Use the arrows up and down to update the counter."),
//!             ]
//!         );
//!
//!        frame.render_widget(text, frame.area());
//!     }
//!
//!     async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
//!         // Terminal-event-handling logic will go here.
//!     }
//! }
//! ```
//!
//! ## Terminal Events
//!
//! Amazing! Now our screen will display a simple message. Next, let's handle some terminal events
//! so we can interact with our application. We'll update the `on_event` method to listen for key
//! presses.
//!
//! ```
//! # use ratapp::{App, Navigator, Screen};
//! use ratatui::{Frame, crossterm::event::{Event, KeyCode}, widgets::Paragraph};
//!
//! # enum ScreenID {}
//! # #[derive(Default)]
//! # struct HomeScreen { counter: u32 }
//! #
//! impl Screen<ScreenID> for HomeScreen {
//!     fn draw(&mut self, frame: &mut Frame) {
//!         // -- Drawing logic as before --
//!     }
//!
//!     async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
//!         if let Event::Key(key_event) = event {
//!             match key_event.code {
//!                 KeyCode::Up => {
//!                     self.counter = self.counter.saturating_add(1);
//!                 }
//!                 KeyCode::Down => {
//!                     self.counter = self.counter.saturating_sub(1);
//!                 }
//!                 _ => {}
//!             }
//!
//!             navigator.redraw(); // Add this line to trigger a re-draw after handling the event.
//!         }
//!     }
//! }
//! ```
//!
//! Now our application will respond to the up and down arrow keys to increment and decrement
//! the counter displayed on the screen. [`Screen::on_event`] gets called whenever a terminal event
//! is sent, and by calling `navigator.redraw()` we trigger a redraw with our updated screen
//! state. That's why you'll see the screen updating its numbers when you press the arrow keys.
//! 
//! [`Screen::draw()`] is only called when a redraw is needed, so it won't be called on every event
//! unless you explicitly request it with `navigator.redraw()`. This helps optimize performance by
//! avoiding unnecessary redraws.
//!
//! ## Running an [`App`]
//!
//! Finally, let's put everything together in our `main.rs`. We haven't seen it live yet, after all.
//!
//! Going back to our `main.rs`, let's get our application running. We'll create an instance of the
//! [`App`] with our `AppScreens` enum and call the [`App::run()`] method to start the application.
//!
//! ```ignore
//! use ratapp::App;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut app = App::new();
//!
//!     app.run::<AppScreens>().await.unwrap();
//! }
//! ```
//!
//! `cargo run` it, and wala! You have a basic asynchronous application running with `ratapp` and
//! `ratatui`. Congrats!
//!
//! Okay but, where's our fun async code? Where's our multi-screen stuff? Up to now, it's
//! practically a normal synchronous app but with `await` and `ratapp` code, nothing that
//! [`ratatui`] couldn't do on its own.
//!
//! ## A Second Screen
//!
//! Let's write a second screen.
//!
//! The first screen is quite simple, just a counter and some text. We want to navigate and to use
//! a more complex component. What about drawing a list? Let's use [`ratatui::widgets::ListState`]
//! for it to add some fun to it.
//!
//! Back to our folder structure, let's create a new module for our second screen.
//!
//! ```txt
//! src/
//!   main.rs
//!   screens/
//!     mod.rs
//!     home.rs
//!     list.rs
//! ```
//!
//! Perfect. Now, let's go to our `screens/mod.rs` and update some parts to add our second screen.
//!
//! ```ignore
//! mod home;
//! mod list;  // Add this!
//!
//! use ratapp::Screens;
//!
//! #[derive(Screens)]
//! pub enum AppScreens {
//!     Home(home::HomeScreen),
//!     List(list::ListScreen),  // And this!
//! }
//!
//! impl Default for AppScreens {
//!     fn default() -> Self {
//!         AppScreens::Home(home::HomeScreen::default())
//!     }
//! }
//! ```
//!
//! That's all we need to do here. Perfect. Now, same as with our home screen, let's create a new
//! screen in our new `list` module.
//!
//! ```ignore
//! use ratapp::{Navigator, Screen};
//! use ratatui::{Frame, crossterm::event::Event};
//!
//! use crate::screens::ScreenID;
//!
//! #[derive(Default)]
//! struct ListScreen;
//!
//! impl Screen<ScreenID> for ListScreen {
//!     fn draw(&mut self, frame: &mut Frame) {
//!         // Drawing logic will go here.
//!     }
//!
//!     async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
//!         // Terminal-event-handling logic will go here.
//!     }
//! }
//! ```
//!
//! Perfect. This screen doesn't do anything yet, but there's no point in adding any cool features
//! to a screen you can't access, so let's add some code to navigate to it. Back to our
//! `HomeScreen` at `src/screens/home.rs`, let's see how to navigate to our new `ListScreen`.
//!
//! ## The [`Navigator`]
//!
//! As you may have guessed, [`Navigator`] lets you navigate between screens. It also triggers
//! re-re-draws. Its API is quite simple, and it'll look familiar if you've done frontend web
//! development before.
//!
//! ```ignore
//! navigator.push(ScreenID::Home);
//!
//! navigator.redraw();
//!
//! navigator.exit();
//! ```
//!
//! Simple, yet enough for most use cases. Our first method here, and the one we'll be using in a
//! minute, is [`Navigator::push()`]. It takes a `ScreenID` as its only argument and it allows for
//! navigating to any screen in your app.
//!
//! Applications have a history stack, which works like a list. When you call [`Navigator::push()`]
//! it adds the new screen to the top of the stack. You can then use [`Navigator::back()`] to go
//! back to the previous screen. Screen state is kept in that stack, so [`Navigator::back()`] will
//! restore the screen state right where you left off.
//!
//! The second method listed above is [`Navigator::redraw()`] which causes `ratapp` to redraw the
//! current screen on your terminal. You can also call it on demand from a background task, for
//! example, to dynamically update the screen based on asynchronous state updates.
//!
//! The third method listed above is [`Navigator::exit()`], which exits the application. Calling it
//! will clean up everything and exit the application.
//!
//! For the full list of [`Navigator`] methods, check its documentation.
//!
//! ## Going to Another Screen
//!
//! Now that we know how to use the [`Navigator`], let's go back to our project and make use of it!
//! Back in our `HomeScreen`, let's update the `on_event` to make it navigate to the `ListScreen`,
//! and add an exit option in the process.
//!
//! ```
//! # use ratapp::{App, Navigator, Screen};
//! use ratatui::{Frame, crossterm::event::{Event, KeyCode}, widgets::Paragraph, text::Line};
//!
//! # enum ScreenID { List }
//! # #[derive(Default)]
//! # struct HomeScreen { counter: u32 }
//! #
//! impl Screen<ScreenID> for HomeScreen {
//!     fn draw(&mut self, frame: &mut Frame) {
//!         let text = Paragraph::new(
//!             vec![
//!                 Line::from("Hello ratapp!"),
//!                 Line::from(""),
//!                 Line::from("This is the home screen. Welcome!"),
//!                 Line::from(""),
//!                 Line::from(format!("Counter: {}", self.counter)),
//!                 Line::from(""),
//!                 Line::from("Use the arrows up and down to update the counter."),
//!                 Line::from("Press enter to go to the list screen."),  // Add this!
//!                 Line::from("Press Q to exit."),                       // Add this!
//!             ]
//!         );
//!
//!        frame.render_widget(text, frame.area());
//!     }
//!
//!     async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
//!         if let Event::Key(key_event) = event {
//!              match key_event.code {
//!                 KeyCode::Up => {
//!                     self.counter = self.counter.saturating_add(1);
//!                 }
//!                 KeyCode::Down => {
//!                     self.counter = self.counter.saturating_sub(1);
//!                 }
//!                 KeyCode::Enter => {                  // Add this!
//!                     navigator.push(ScreenID::List);  // Add this!
//!                 }                                    // Add this!
//!                 KeyCode::Char('q') => {              // Add this!
//!                     navigator.exit();                // Add this!
//!                 }                                    // Add this!
//!                 _ => {}
//!             }
//!
//!             navigator.redraw();
//!         }
//!     }
//! }
//! ```
//!
//! Fantastic! Now when you do `cargo run`, you'll have your application appear in the
//! `HomeScreen`, as before, but when you press Q the application will exit and when you press
//! Enter the screen will go blank. That's exactly what is supposed to happen.
//!
//! ## More State
//!
//! Now, let's add our beautiful [`ratatui::widgets::ListState`]! This will allow us to keep track
//! of the currently-selected item and update our UI accordingly in real time.
//!
//! First of all, let's make our `ListScreen` be able to exit and go back home. Let's keep the keys
//! the same as they are on our `HomeScreen` to keep it consistent: Enter to navigate and Q to
//! exit.
//!
//! ```
//! # use ratapp::{Navigator, Screen};
//! # use ratatui::{Frame, crossterm::event::{Event, KeyCode}};
//! #
//! # #[derive(Default)]
//! # struct ListScreen;
//! #
//! # enum ScreenID { Home }
//! #
//! impl Screen<ScreenID> for ListScreen {
//!     fn draw(&mut self, frame: &mut Frame) {
//!         // Drawing logic will go here.
//!     }
//!
//!     async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
//!         if let Event::Key(key_event) = event {  // Add this!
//!             match key_event.code {              // Add this!
//!                 KeyCode::Enter => {             // Add this!
//!                     navigator.back();           // Add this!
//!                 }                               // Add this!
//!                 KeyCode::Char('q') => {         // Add this!
//!                     navigator.exit();           // Add this!
//!                 }                               // Add this!
//!                 _ => {}                         // Add this!
//!             }                                   // Add this!
//!                                                 // Add this!
//!             navigator.redraw();                 // Add this!
//!         }                                       // Add this!
//!     }
//! }
//! ```
//!
//! Great! Now, we're no longer locked in that screen once we navigate to it. Let's add our
//! [`List`](ratatui::widgets::List).
//!
//! Let's add a field to our `ListScreen` called `state` and make it a
//! [`ListState`](ratatui::widgets::ListState).
//!
//! ```
//! use ratatui::widgets::ListState;
//!
//! #[derive(Default)]
//! pub struct ListScreen {
//!     state: ListState
//! }
//! ```
//!
//! Great. Now, let's draw a list on our screen. We'll use some text and layout too to make it a
//! bit more user-friendly, since without guides our user wouldn't know how to use our app.
//!
//! ```
//! # use ratapp::{Navigator, Screen};
//! # use ratatui::{
//! #     Frame,
//! #     crossterm::event::Event,
//! #     widgets::{List, ListState, ListItem, Paragraph},
//! #     text::Line,
//! #     layout::{Layout, Constraint}
//! # };
//! #
//! # #[derive(Default)]
//! # struct ListScreen { state: ListState }
//! #
//! # enum ScreenID {}
//! #
//! impl Screen<ScreenID> for ListScreen {
//!     fn draw(&mut self, frame: &mut Frame) {
//!         let layout = Layout::vertical([
//!             Constraint::Length(3),
//!             Constraint::Length(1), // Gap
//!             Constraint::Fill(1),
//!         ]);
//!   
//!         let [list_area, _, text_area] = layout.areas(frame.area());
//!   
//!         let list = List::new(vec![
//!             ListItem::new("1"),
//!             ListItem::new("2"),
//!             ListItem::new("3"),
//!         ])
//!         .highlight_symbol("> ");
//!   
//!         let text = Paragraph::new(vec![
//!             Line::from("Use the arrows up and down to change the selected item."),
//!             Line::from(""),
//!             Line::from("Press enter to go back home."),
//!             Line::from(""),
//!             Line::from("Press Q to exit."),
//!         ]);
//!   
//!         frame.render_stateful_widget(list, list_area, &mut self.state);
//!         frame.render_widget(text, text_area);
//!     }
//!
//!     async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
//!         // Our previous code...
//!     }
//! }
//! ```
//!
//! As you can see, anything you could draw on a bare [`ratatui`] app can be drawn here. Now that
//! we have our pretty list drawn on the screen, let's make the arrows change the selected item!
//!
//! ```
//! # use ratapp::{Navigator, Screen};
//! # use ratatui::{Frame, crossterm::event::{Event, KeyCode}, widgets::ListState};
//! #
//! # #[derive(Default)]
//! # struct ListScreen { state: ListState }
//! #
//! # enum ScreenID { Home }
//! #
//! impl Screen<ScreenID> for ListScreen {
//! #   fn draw(&mut self, frame: &mut Frame) {}
//! #
//!     async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>) {
//!         if let Event::Key(key_event) = event {
//!             match key_event.code {
//!                 KeyCode::Up => {                   // Add this!
//!                     self.state.select_previous();  // Add this!
//!                 }                                  // Add this!
//!                 KeyCode::Down => {                 // Add this!
//!                     self.state.select_next();      // Add this!
//!                 }                                  // Add this!
//!                 KeyCode::PageUp => {               // Add this!
//!                     self.state.select_first();     // Add this!
//!                 }                                  // Add this!
//!                 KeyCode::PageDown => {             // Add this!
//!                     self.state.select_last();      // Add this!
//!                 }                                  // Add this!
//!                 KeyCode::Enter => {
//!                     navigator.push(ScreenID::Home);
//!                 }
//!                 KeyCode::Char('q') => {
//!                     navigator.exit();
//!                 }
//!                 _ => {}
//!             }
//!
//!             navigator.redraw();
//!         }
//!     }
//! }
//! ```
//!
//! `cargo run` and... It works! You'll notice, though, that the list's selected item indicator
//! doesn't appear until after you press either arrow, and that snap doesn't look too nice. Let's
//! fix that.
//!
//! ## Final Polish
//!
//! Instead of deriving [`Default`], we'll implement [`Default`] on the `ListScreen` type
//! ourselves.
//!
//! ```
//! use ratatui::widgets::ListState;
//!
//! struct ListScreen {
//!     state: ListState,
//! }
//!
//! impl Default for ListScreen {
//!     fn default() -> Self {
//!         ListScreen {
//!             state: ListState::default().with_selected(Some(0)),
//!         }
//!     }
//! }
//! ```
//!
//! That should be it. `cargo run` it again and you'll see now the first item is selected by
//! default. No more snaps.
//!
//! Congratulations! You finished the Quick Start tutorial, now you have a little app you can work
//! on to make it cool and yours or just jump straight into what you had in mind. Good luck!
//!
//! > The final code of this tutorial can be found under `examples/tutorial.rs` in our [GitHub
//! > repository](https://github.com/Nekidev/ratapp). Check it out if you encounter any issues!
//!
//! # Advanced Usage
//!
//! This part of the documentation covers more advanced usage of `ratapp`, including how to manage
//! global application state across screens using the [`ScreenWithState`] trait, how to dynamically
//! trigger re-draws, and more. It's not a step-by-step tutorial like the Quick Start, but it's
//! still meant to be easy to follow.
//!
//! ## Global Application State
//!
//! Sometimes, you may want to share some state across multiple screens in your application. For
//! example, you might have user preferences, a theme setting, or any other data that should be
//! accessible from different parts of your app.
//!
//! To achieve this, `ratapp`'s [`App`] struct can be initialized with [`App::with_state()`] to
//! hold a global application state. This state can then be accessed and modified by screens that
//! implement the [`ScreenWithState`] trait (instead of the [`Screen`] trait).
//!
//! For example:
//!
//! ```
//! use ratapp::{App, Navigator, ScreenWithState, Screens};
//! use ratatui::{Frame, crossterm::event::Event};
//!
//! enum Theme {
//!     Light,
//!     Dark
//! }
//!
//! struct State {
//!     theme: Theme
//! }
//!
//! #[derive(Screens)]
//! pub enum MyScreens {
//!    Home(HomeScreen),
//! }
//!
//! impl Default for MyScreens {
//!     fn default() -> Self {
//!         MyScreens::Home(HomeScreen::default())
//!     }
//! }
//!
//! #[derive(Default)]
//! struct HomeScreen;
//!
//! impl ScreenWithState<ScreenID, State> for HomeScreen {
//!     fn draw(&mut self, frame: &mut Frame, state: &State) {
//!         // Use state.theme to determine colors, etc.
//!     }
//!
//!     async fn on_event(&mut self, event: Event, navigator: Navigator<ScreenID>, state: &mut State) {
//!         // Modify state.theme based on user input, etc.
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut app = App::with_state(State { theme: Theme::Light });
//! }
//! ```
//!
//! Note that [`Screen`] and [`ScreenWithState`] can both be combined in a single app. Use the one
//! that works best for each screen.
//!
//! ## On-demand Re-drawing
//!
//! `ratapp` provides the [`Navigator::redraw()`] method to trigger a re-draw of the current
//! screen. Since [`Navigator`] can be cloned and sent across threads, this means you can implement
//! background tasks that update the UI asynchronously.
//!
//! For example, if you want to do a tick-based animation, like a spinner, you could do something
//! like:
//!
//! ```
//! use ratapp::{App, Navigator, Screen, Screens};
//! use ratatui::{
//!     Frame,
//!     crossterm::event::{Event, KeyCode},
//!     text::Text,
//! };
//! use std::time::Duration;
//! 
//! fn get_tick(tick: usize) -> char {
//!     match tick % 4 {
//!         0 => '-',
//!         1 => '\\',
//!         2 => '|',
//!         3 => '/',
//!         _ => unreachable!(),
//!     }
//! }
//! 
//! #[derive(Default)]
//! struct TickBasedScreen {
//!     tick: usize,
//! }
//! 
//! impl Screen<ScreenID> for TickBasedScreen {
//!     fn draw(&mut self, frame: &mut Frame) {
//!         let text = Text::from(format!(
//!             "{} Dummy loading... (press Q to exit)",
//!             get_tick(self.tick)
//!         ));
//! 
//!         frame.render_widget(text, frame.area());
//!     }
//! 
//!     async fn task(&mut self, navigator: Navigator<ScreenID>) {
//!         tokio::time::sleep(Duration::from_millis(200)).await;
//!         self.tick = self.tick.wrapping_add(1);
//!         navigator.redraw();
//!     }
//! }
//! ```
//!
//! That screen would update itself every 200 milliseconds and add 1 to the tick state, effectively
//! animating the spinner.
//! 
//! As a side note, given that `task()` gets cancelled on events, if you hold down a key pressed,
//! the spinner will stop updating until the key is released (because the task call gets cancelled
//! before it can update).
//!
//! ## Screen Hooks
//!
//! Screens have a few different hooks you can override to run code at specific points in their
//! lifecycle. The most important ones are:
//!
//! - `on_enter`: Called when the screen is entered.
//! - `on_exit`: Called when the screen is exited.
//! - `on_pause`: Called when the screen is paused (another screen is pushed on top).
//! - `on_resume`: Called when the screen is resumed (the top screen is popped off).
//!
//! These hooks are asynchronous. However, they run sequentially, so make sure to avoid long
//! operations that could block the UI.
//!
//! ## [`State`]
//!
//! Sometimes, you want to update the screen state from a background task, like in the tick-based
//! example above. `ratapp` provides the [`State`] type for this purpose.
//!
//! It's a wrapper to [`Arc<Mutex<T>>`](std::sync::Arc) with a nice name, so you should not keep
//! hold of [`StateHandle`]s across `await` points to avoid deadlocks. Instead, clone the [`State`]
//! and get a new handle when needed.
//!
//! # Contributing
//!
//! `ratapp` is pretty new, so some things may be undocumented or missing. If you find any of that,
//! feel free to open an issue or PR in our [GitHub repository](https://github.com/Nekidev/ratapp).
//! All contributions are welcome!

mod app;
mod navigation;
mod screen;
mod state;

pub use app::App;
pub use navigation::Navigator;
pub use screen::{Screen, ScreenState, ScreenWithState};
pub use state::{State, StateHandle};

pub use ratapp_macros::Screens;
