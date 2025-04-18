use crossterm::event;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::state::GameState;
use crate::input::EventHandler;

pub struct TerminalPanel {
    // Terminal-specific state
}

impl TerminalPanel {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn render(&self, frame: &mut Frame, area: Rect, state: &GameState, event_handler: &EventHandler) {
        // Create terminal border
        let block = Block::default()
            .title(" TERMINAL ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));
        
        // Split into output area and input line
        let terminal_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(block.inner(area));
            
        // Render terminal output
        let output_text = Text::from(state.command_output.clone());
        let output = Paragraph::new(output_text)
            .style(Style::default().fg(Color::Green));
        frame.render_widget(output, terminal_chunks[0]);
        
        // Render command input line
        let input_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Green));
        
        // Get input text from event handler
        let input_text = Span::from(format!(" >_ {}", event_handler.current_input()))
            .style(Style::default().fg(Color::Green));
        let input = Paragraph::new(input_text);
        frame.render_widget(input_block, terminal_chunks[1]);
        frame.render_widget(input, terminal_chunks[1]);
        
        // Render outer block
        frame.render_widget(block, area);
    }
}