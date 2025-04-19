use ratatui::Frame;

use crate::state::GameState;
use super::status::StatusPanel;
use super::terminal::TerminalPanel;
use crate::input::EventHandler;

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

    pub fn render(&self, frame: &mut Frame, state: &GameState, event_handler: &EventHandler) {
        self.terminal_panel.render(frame, frame.area(), state, &event_handler);
        //self.status_panel.render(frame, frame.area(), state);
    }
}