use std::time::Instant;
use ratatui::{
    prelude::*,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use crate::game::GameState;

const CURSOR_BLINK_RATE: f32 = 2.0; // Cursor blink rate in Hz

pub struct TerminalPanel {
    // cursor_pos: usize, // position of the cursor in the input line
    cursor_visible: bool,   // whether or not the cursor is visible
    last_cursor_toggle: Instant, // tracks the last time the cursor toggled
    command_output: String,
    current_input: String,
}

impl TerminalPanel {
    pub fn new() -> Self {
        Self {
            cursor_visible: false,
            // cursor_pos: 0,
            last_cursor_toggle: Instant::now(),
            command_output: String::new(),
            current_input: String::new(),
        }
    }

    pub fn update(&mut self, game_state: &GameState) {
        // Check if it's time to toggle the cursor visibility
        let elapsed = self.last_cursor_toggle.elapsed().as_secs_f32();
        if elapsed >= 1.0 / CURSOR_BLINK_RATE {
            self.cursor_visible = !self.cursor_visible;
            self.last_cursor_toggle = Instant::now(); // Reset the toggle timer
        }
    
        // Update command output and current input
        self.command_output = game_state.terminal.command_output.clone();
        self.current_input = game_state.terminal.current_input.clone();
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
            

        // Render terminal output with text wrapping
        let output_text = Text::from(self.command_output.clone());
        Paragraph::new(output_text)
            .style(Style::default().fg(Color::Green))
            .block(block.clone())
            .wrap(ratatui::widgets::Wrap { trim: true })
            .render(terminal_chunks[0], buf);
        
        // Render input
        let input_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Green));
        
        // Create the input text with cursor.
        let input_text = if self.cursor_visible {
            Span::from(format!(" >{}_ ", self.current_input))
        } else {
            Span::from(format!(" >{}  ", self.current_input))
        }
        .style(Style::default().fg(Color::Green));

        Paragraph::new(input_text)
                .block(input_block)
                .render(terminal_chunks[1], buf);

    }
}