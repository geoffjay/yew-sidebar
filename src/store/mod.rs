use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Store)]
pub struct State {
    pub sidebar: Sidebar,
}

impl State {
    pub fn is_sidebar_open(&self) -> bool {
        log::info!("is_sidebar_open: {:?}", self.sidebar.is_open);
        self.sidebar.is_open
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar.is_open = !self.sidebar.is_open;
    }

    pub fn open_sidebar(&mut self) {
        log::info!("open_sidebar");
        self.sidebar.is_open = true;
    }

    pub fn close_sidebar(&mut self) {
        self.sidebar.is_open = false;
    }

    pub fn set_sidebar_title(&mut self, title: String) {
        self.sidebar.title = title;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sidebar {
    pub is_open: bool,
    pub title: String,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            is_open: false,
            title: "Sidebar".to_string(),
        }
    }
}
