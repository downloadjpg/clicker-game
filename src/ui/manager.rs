use crossterm::terminal;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect,}, prelude::*, style::{Color, Style}, symbols::half_block, text::{Span, Text}, widgets::{Block, Borders, Paragraph, Widget}, Frame
};
use crate::state::GameState;
use super::status::StatusPanel;
use super::terminal::TerminalPanel;
use crate::input::EventHandler;
use std::time::Instant;

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

    pub fn update(&mut self, state: &GameState, event_handler: &EventHandler) {
        self.terminal_panel.update(state, event_handler);
        self.status_panel.update(state);
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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