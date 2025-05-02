use ratatui::{
    prelude::*,
    Terminal,
    layout::{Constraint, Direction, Layout, Rect,},
};

mod terminal;
mod status;

use terminal::TerminalPanel;
use status::StatusPanel;
use super::state::GameState;

pub struct UiManager {
    terminal_panel: TerminalPanel,
    status_panel: StatusPanel,
}

impl UiManager {
    pub fn new() -> Self {
        Self {
            terminal_panel: TerminalPanel::new(),
            status_panel: StatusPanel::new(),
        }
    }

    pub fn draw<B: Backend>(&mut self, state: &GameState, terminal: &mut Terminal<B>) {
        // Update state of panels
        self.terminal_panel.update(state);
        self.status_panel.update(state);

        // Draw everything
        let _ = terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut()));
    }
    
}

impl Widget for &UiManager {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(area);

        self.terminal_panel.render(chunks[0], buf);
        self.status_panel.render(chunks[1], buf);
    }
}