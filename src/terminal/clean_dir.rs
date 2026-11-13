use crate::terminal::models::AppState;

pub fn clean_dirs(app_state: &mut AppState) {
    app_state.dirs = vec![];
}
