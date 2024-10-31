use std::{
    ops::{Deref, DerefMut},
    sync::{LazyLock, Mutex},
};

use savefile::{load_file, save_file};
use savefile_derive::Savefile;

use crate::config::CONFIG;

#[derive(Default, Debug, Savefile)]
pub struct State {
    pub current_factorio_version: String,
    pub latest_factorio_version: String,
    pub last_chat_message: String,
    pub latest_fff: String,
}

pub static STATE: LazyLock<Mutex<PersistedState<State>>> =
    LazyLock::new(|| Mutex::new(State::new()));

impl State {
    pub fn new() -> PersistedState<Self> {
        let state = load_file(&CONFIG.state_file_path, 0).unwrap_or(Self::default());

        PersistedState::new(state, |state| {
            save_file(&CONFIG.state_file_path, 0, state).unwrap()
        })
    }
}

pub struct PersistedState<T> {
    inner_value: T,
    save_fn: fn(&T),
}

impl<T> PersistedState<T> {
    pub fn new(inner_value: T, save_fn: fn(&T)) -> Self {
        Self {
            inner_value,
            save_fn,
        }
    }

    pub fn write(&mut self) -> ModifyGuard<T> {
        ModifyGuard {
            inner_value: &mut self.inner_value,
            save_fn: self.save_fn,
        }
    }
}

impl<T> Deref for PersistedState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner_value
    }
}

pub struct ModifyGuard<'a, T> {
    inner_value: &'a mut T,
    save_fn: fn(&T),
}

impl<T> Deref for ModifyGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner_value
    }
}

impl<T> DerefMut for ModifyGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner_value
    }
}

impl<T> Drop for ModifyGuard<'_, T> {
    fn drop(&mut self) {
        (self.save_fn)(self.inner_value);
    }
}
