
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
    style::{Color, Style},
    text::Text,
};

use crate::game::GameState;

pub struct StatusPanel {
    // Status-specific state
    // Shouldn't need anything if the game state is passed in,
    // maybe store active components here?
    total_credits: u64,
    credits_per_second: u64,
}

impl StatusPanel {
    pub fn new() -> Self {
        Self {
            total_credits : 0,
            credits_per_second: 0,
        }
    }

    pub fn update(&mut self, state: &GameState) {
        // Update status information from the game state
        self.total_credits = state.resources.credits as u64;
        self.credits_per_second = state.resources.credits_per_second as u64; 
    }


}

impl Widget for &StatusPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create terminal border
        let block = Block::default()
            .title(" STATUS ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        // Get relevant status information from self
        let text = format!(
            "Credits: {}\nCPS: {}\n",
            self.total_credits,
            self.credits_per_second,
        );

        // Display status information
        let status_text = Text::from(text);
        let inner = block.inner(area);
        Paragraph::new(status_text)
            .style(Style::default().fg(Color::Green))
            .block(block)
            .render(inner, buf);
    }
}