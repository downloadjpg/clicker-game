use crate::state::GameState;

pub struct CommandProcessor {
    // Command-specific state
}

impl CommandProcessor {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn process_command(&self, cmd: &str, state: &mut GameState) {
        let cmd_parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if cmd_parts.is_empty() {
            return;
        }
        
        // Add command to history
        state.command_history.push(cmd.to_string());
        
        // Process command
        match cmd_parts[0] {
            "help" => self.cmd_help(state),
            "hack" => self.cmd_hack(state),
            "buy" if cmd_parts.len() > 1 => self.cmd_buy(cmd_parts[1], state),
            "status" => self.cmd_status(state),
            //"clear" => self.cmd_clear(state),
            _ => state.command_output = format!("UNKNOWN COMMAND: '{}'", cmd),
        }
    }
    
    fn cmd_help(&self, state: &mut GameState) {
        state.command_output = "AVAILABLE COMMANDS:\n\
                               hack - Earn credits\n\
                               buy <item> - Purchase upgrades\n\
                               status - Show current status\n\
                               clear - Clear terminal\n\
                               help - Show this help".to_string();
    }
    
    fn cmd_hack(&self, state: &mut GameState) {
        let hack_value : f64 = 1.0; // Base value
        state.resources.credits += hack_value;
        state.command_output = format!("HACK SUCCESSFUL. +{} CREDITS", hack_value);
    }

    fn cmd_status(&self, state: &mut GameState) {
        state.command_output = format!("CREDITS: {}\n\
                               CREDITS PER SECOND: {}\n",
                               state.resources.credits as i64,
                               state.resources.credits_per_second as i64);
    }
    
    fn cmd_buy(&self, name: &str, state: &mut GameState) {
        match name {
            "port-scan" => {
                state.resources.credits_per_second += 1.0;
                state.command_output = format!("PURCHASED 1 PORT-SCAN UPGRADE. CREDITS PER SECOND: {}",
                                               state.resources.credits_per_second);
            }
            _ => {
                state.command_output = format!("UNKNOWN ITEM: '{}'", name);
            }
        }
    }
}