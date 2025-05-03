use crossterm::event::KeyEvent;
use crate::game::{EventBus, Event};
pub struct GameTerminal {
    pub command_output: String,
    pub current_input: String,
    pub command_history: Vec<String>,
    history_index: usize,
    //temp_input: String, // Temporary input for command history navigation... 
}

impl GameTerminal {
    pub fn new() -> Self {
        Self {
            command_output: "SYSTEM INITIALIZED. TYPE 'help' FOR COMMANDS.".to_string(),
            current_input: String::new(),
            command_history: Vec::new(),
            history_index: 0, // Index for command history. 0 points to the oldest command.
            //temp_input: String::new(), // Temporary input for command history navigation
        }
    }

    pub fn process_input(&mut self, input: KeyEvent, event_bus: &mut EventBus) {
        use crossterm::event::KeyCode::*;

        match input.code {
            Enter => {
                // Process the command
                self.command_history.insert(0, self.current_input.clone());
                // Call command processor here
                event_bus.push(Event::Command(self.current_input.clone()));
                self.current_input.clear();
            }
            Backspace => {
                // Remove the last character from the current input
                self.current_input.pop();
            }
            Up => {
                // Move to the previous command in history
                if let Some(last_command) = self.command_history.first() {
                    self.current_input = last_command.clone();
                    self.history_index += 1;
                }
            }
            Char(c) => {
                // Add the character to the current input
                self.current_input.push(c);
            }
            _ => {}
        }
    }
}