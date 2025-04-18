use std::{io, time::Duration};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub struct EventHandler {
    // Current input being typed
    current_input: String,
    // Position of cursor in current input
    cursor_position: usize,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            cursor_position: 0,
        }
    }

    /// Polls for events with a timeout
    pub fn next(&self, timeout: Duration) -> io::Result<Option<Event>> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    /// Handles input events, returns command string when Enter is pressed
    pub fn handle_input(&mut self, event: &Event) -> Option<String> {
        if let Event::Key(key) = event {
            match key.code {
                // Execute command on Enter
                KeyCode::Enter => {
                    let command = self.current_input.clone();
                    self.current_input.clear();
                    self.cursor_position = 0;
                    if !command.is_empty() {
                        return Some(command);
                    }
                }
                
                // Handle backspace
                KeyCode::Backspace => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        self.current_input.remove(self.cursor_position);
                    }
                }
                
                // Handle delete
                KeyCode::Delete => {
                    if self.cursor_position < self.current_input.len() {
                        self.current_input.remove(self.cursor_position);
                    }
                }
                
                // Handle arrow keys for cursor movement
                KeyCode::Left => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.cursor_position < self.current_input.len() {
                        self.cursor_position += 1;
                    }
                }
                
                // Handle printable characters
                KeyCode::Char(c) => {
                    // Ctrl+C or Ctrl+D can be handled separately for exit
                    if key.modifiers.contains(KeyModifiers::CONTROL) && (c == 'c' || c == 'd') {
                        // Do nothing here, will be handled in should_quit
                    } else {
                        // Insert character at cursor position
                        self.current_input.insert(self.cursor_position, c);
                        self.cursor_position += 1;
                    }
                }
                
                // Other keys like Home, End, etc.
                KeyCode::Home => self.cursor_position = 0,
                KeyCode::End => self.cursor_position = self.current_input.len(),
                
                // Ignore other keys
                _ => {}
            }
        }
        None
    }
    
    /// Check if the application should exit
    pub fn should_quit(&self, event: &Event) -> bool {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
            // Exit on Ctrl+C, Ctrl+D, or Esc
            (*code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL)) ||
            (*code == KeyCode::Char('d') && modifiers.contains(KeyModifiers::CONTROL)) ||
            *code == KeyCode::Esc
        } else {
            false
        }
    }
    
    /// Get the current input string for display
    pub fn current_input(&self) -> &str {
        &self.current_input
    }
    
    /// Get cursor position for rendering
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }
}