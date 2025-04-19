
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::state::GameState;

pub struct StatusPanel {
    // Status-specific state
    // Shouldn't need anything if the game state is passed in,
    // maybe store active components here?
}

impl StatusPanel {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn render(&self, frame: &mut Frame, area: Rect, state: &GameState) {
        // Create terminal border
        
        // Get relevant status information from the game state
        let text = format!("
            Credits: {}\n\
            CPS: {}\n\
            ",
            state.resources.credits,
            state.resources.credits_per_second,
        );

        let block = Block::default()
            .title(" STATUS ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));
        
        // Display status information
        let status_text = Text::from(text);
        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Green));
        frame.render_widget(status, area);
        
        // Render outer block
        frame.render_widget(block, area);
    }
}