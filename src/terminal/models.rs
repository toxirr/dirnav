use ratatui::widgets::ListState;
use std::path::PathBuf;

#[derive(Default)]
pub struct AppState {
    pub dirs: Vec<PathBuf>,
    pub lists: ListState,
}
