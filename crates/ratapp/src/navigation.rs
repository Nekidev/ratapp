//! Navigation between screens, exiting the app, and requesting rerenders.
//!
//! It exposes the [`Navigator`] struct, which is passed to each screen to allow them to
//! navigate between each other, request rerenders, or exit the application.
//!
//! Check out the documentation of the [`Navigator`] for more information.

use tokio::sync::mpsc;

/// Allows screens to navigate between each other, request rerenders, or exit the application.
///
/// The API has a few methods to perform navigation actions:
/// - [`Navigator::push()`]: Pushes a new screen onto the navigation stack.
/// - [`Navigator::replace()`]: Replaces the current screen with a new one.
/// - [`Navigator::back()`]: Pops the current screen off the navigation stack, returning to the
///   previous screen.
/// - [`Navigator::clear()`]: Clears the entire navigation stack, leaving only the current screen.
/// - [`Navigator::restart()`]: Restarts the application, clearing the navigation stack and
///   returning to the initial screen.
/// - [`Navigator::exit()`]: Exits the application.
/// - [`Navigator::rerender()`]: Requests a rerender of the current screen.
///
/// [`Navigator`]s are clonable and sendable, so you can
#[derive(Clone)]
pub struct Navigator<ID> {
    pub(crate) channel: mpsc::UnboundedSender<Action<ID>>,
}

impl<ID> Navigator<ID> {
    pub(crate) fn new(channel: mpsc::UnboundedSender<Action<ID>>) -> Self {
        Navigator { channel }
    }

    pub fn push(&self, id: ID) {
        self.channel
            .send(Action::Push(id))
            .expect("The Navigator actions channel was dropped! This is a ratapp bug.");
    }

    pub fn replace(&self, id: ID) {
        self.channel
            .send(Action::Replace(id))
            .expect("The Navigator actions channel was dropped! This is a ratapp bug.");
    }

    pub fn back(&self) {
        self.channel
            .send(Action::Back)
            .expect("The Navigator actions channel was dropped! This is a ratapp bug.");
    }

    pub fn clear(&self) {
        self.channel
            .send(Action::Clear)
            .expect("The Navigator actions channel was dropped! This is a ratapp bug.");
    }

    pub fn restart(&self) {
        self.channel
            .send(Action::Restart)
            .expect("The Navigator actions channel was dropped! This is a ratapp bug.");
    }

    pub fn exit(&self) {
        self.channel
            .send(Action::Exit)
            .expect("The Navigator actions channel was dropped! This is a ratapp bug.");
    }

    pub fn rerender(&self) {
        self.channel
            .send(Action::Rerender)
            .expect("The Navigator actions channel was dropped! This is a ratapp bug.");
    }
}

/// Actions that can be performed by the [`Navigator`].
///
/// These actions are sent to the main application loop to be processed.
pub(crate) enum Action<ID> {
    Push(ID),
    Replace(ID),
    Back,
    Clear,
    Restart,
    Exit,
    Rerender,
}
