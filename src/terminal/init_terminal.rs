use crate::terminal::terminal::run_terminal;

pub fn init_terminal() -> Result<(), color_eyre::eyre::Error> {
    let mut terminal = ratatui::init();
    // let frame = terminal.get_frame();
    // ratatui::restore();
    run_terminal(&mut terminal)
}
