pub mod terminal;
use crate::terminal::init_terminal::init_terminal;

fn main() -> Result<(), color_eyre::eyre::Error> {
    init_terminal()
}
