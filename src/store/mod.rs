use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
#[store(storage = "local")]
pub struct State {
    pub sidebar: Sidebar,
}

impl State {
    pub fn is_sidebar_open(&self) -> bool {
        self.sidebar.is_open
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar.is_open = !self.sidebar.is_open;
    }

    pub fn open_sidebar(&mut self) {
        self.sidebar.is_open = true;
    }

    pub fn close_sidebar(&mut self) {
        self.sidebar.is_open = false;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sidebar {
    pub is_open: bool,
}
