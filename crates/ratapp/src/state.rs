use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use std::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct State<T>(Arc<Mutex<T>>);

impl<T> State<T> {
    pub fn new(state: T) -> Self {
        State(Arc::new(Mutex::new(state)))
    }

    pub fn get(&self) -> StateHandle<'_, T> {
        StateHandle(self.0.lock().expect("Failed to lock the application state mutex"))
    }
}

pub struct StateHandle<'a, T>(MutexGuard<'a, T>);

impl<'a, T> Deref for StateHandle<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for StateHandle<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Default for State<T>
where
    T: Default,
{
    fn default() -> Self {
        State::new(T::default())
    }
}
