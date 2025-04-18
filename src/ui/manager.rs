use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::state::GameState;
use super::terminal::TerminalPanel;
use crate::input::EventHandler;

pub struct UiManager {
    terminal_panel: TerminalPanel,
}

impl UiManager {
    pub fn new() -> Self {
        Self {
            terminal_panel: TerminalPanel::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame, state: &GameState, event_handler: &EventHandler) {
        // Render terminal panel
        self.terminal_panel.render(frame, frame.area(), state, &event_handler);
    }
}