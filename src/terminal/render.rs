use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, Widget},
};

use crate::terminal::models::AppState;

pub fn render(frame: &mut Frame, state: &mut AppState) -> () {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);
    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::DarkGray)
        .render(border_area, frame.buffer_mut());
    let lists = List::new(state.dirs.iter().map(|x| {
        ListItem::style(
            ListItem::from(x.file_name().unwrap().to_string_lossy()),
            Color::DarkGray,
        )
    }))
    .highlight_symbol(">")
    .highlight_style(Style::default().fg(Color::Green));
    frame.render_stateful_widget(lists, inner_area, &mut state.lists);
    frame.render_widget(Block::bordered().title("「 ✦ dir_nav 🦀 ✦ 」"), inner_area);
}
