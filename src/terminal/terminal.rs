use std::{env, process::Command};

use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};

use crate::terminal::{
    clean_dir::clean_dirs, generate_dir::generate_dir, models::AppState, render::render,
};

pub fn run_terminal(terminal: &mut DefaultTerminal) -> Result<(), color_eyre::eyre::Error> {
    let path = env::current_dir()?;
    let mut app_state = AppState::default();
    let mut is_dotfiles_visible = false;
    let mut show_files = true;
    generate_dir(&path, &mut app_state, false, show_files)?;
    loop {
        terminal.draw(|f| render(f, &mut app_state))?;
        if let Event::Key(k) = event::read()? {
            match k.code {
                event::KeyCode::Esc => {
                    let path = env::current_dir()?;
                    env::set_current_dir(&path)?;
                    break;
                }
                event::KeyCode::Char(char) => match char {
                    'j' => {
                        app_state.lists.select_next();
                    }
                    'k' => {
                        app_state.lists.select_previous();
                    }
                    'b' => {
                        env::set_current_dir("../")?;
                        let path = env::current_dir()?;
                        clean_dirs(&mut app_state);
                        generate_dir(&path, &mut app_state, is_dotfiles_visible, show_files);
                    }
                    'f' => {
                        if let Some(i) = app_state.lists.selected() {
                            let location = &app_state.dirs[i];
                            env::set_current_dir(location);
                            let path = env::current_dir()?;
                            clean_dirs(&mut app_state);
                            show_files = true;
                            generate_dir(&path, &mut app_state, is_dotfiles_visible, show_files);
                        }
                    }
                    'z' => {
                        Command::new("zed")
                            .arg(".")
                            .spawn()
                            .expect("zed command failed");
                    }
                    'c' => {
                        Command::new("code")
                            .arg(".")
                            .spawn()
                            .expect("vs_code command failed");
                    }
                    '.' => {
                        is_dotfiles_visible = !is_dotfiles_visible;
                        let path = env::current_dir()?;
                        generate_dir(&path, &mut app_state, is_dotfiles_visible, show_files);
                    }
                    'g' => {
                        show_files = !show_files;
                        let path = env::current_dir()?;
                        generate_dir(&path, &mut app_state, is_dotfiles_visible, show_files);
                    }
                    'p' => {
                        if let Some(i) = app_state.lists.selected() {
                            let location = &app_state.dirs[i];
                            opener::open(location)?;
                        }
                    }
                    'q' => break,
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}
