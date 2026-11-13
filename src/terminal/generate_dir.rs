use crate::terminal::{clean_dir::clean_dirs, models::AppState};
use color_eyre::eyre::{Ok, Result};
use std::{
    fs::{self},
    path::PathBuf,
};

pub fn generate_dir(
    path: &PathBuf,
    app_state: &mut AppState,
    show_dotfiles: bool,
    show_files: bool,
) -> Result<()> {
    clean_dirs(app_state);
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let is_dotfile = entry.file_name().to_string_lossy().starts_with(".");

        let should_include = if show_dotfiles { true } else { !is_dotfile };

        if should_include {
            if show_files {
                app_state.dirs.push(path)
            } else if path.is_dir() {
                app_state.dirs.push(path)
            }
        }
    }
    Ok(())
}
