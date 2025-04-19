
use ratatui::{
    prelude::*,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph, StatefulWidget},
};
use crate::input::EventHandler;
use crate::state::GameState;
//use crate::input::EventHandler;

pub struct TerminalPanel {
    // Terminal-specific state
    command_output: String,
    current_input: String,
}

impl TerminalPanel {
    pub fn new() -> Self {
        Self {
            command_output: String::new(),
            current_input : String::new(),
        }
    }

    pub fn update(&mut self, state: &GameState, event_handler: &EventHandler) {
        // Gather necessary state information into self
        self.command_output = state.command_output.to_string();
        self.current_input = event_handler.current_input().to_string();
    }
}

impl Widget for &TerminalPanel {
    fn render(self, area: Rect, buf: &mut Buffer){
        // Terminal border
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
        let output_text = Text::from(self.command_output.clone());
        Paragraph::new(output_text)
            .style(Style::default().fg(Color::Green))
            .block(block)
            .render( terminal_chunks[0], buf);
        
        // Render input
        let input_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Green));
        
        let input_text = Span::from(format!(" >{} ", self.current_input))
            .style(Style::default().fg(Color::Green));

        Paragraph::new(input_text)
                .block(input_block)
                .render(terminal_chunks[1], buf);

    }
}