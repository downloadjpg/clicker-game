use super::GameState;




pub fn process(cmd: &str, state: &mut GameState) {
    let cmd_parts: Vec<&str> = cmd.trim().split_whitespace().collect();
    if cmd_parts.is_empty() {
        return;
    }
    
    // Process command
    match cmd_parts[0] {
        "help" => cmd_help(state),
        "hack" => cmd_hack(state),
        "buy" if cmd_parts.len() > 1 => cmd_buy(cmd_parts[1], state),
        "status" => cmd_status(state),
        "echo" => cmd_echo(&cmd_parts[1..], state),
        "shop" => cmd_shop(state),
        _ => state.terminal.command_output = format!("UNKNOWN COMMAND: '{}'", cmd),
    }
}

fn cmd_help(state: &mut GameState) {
    state.terminal.command_output = "AVAILABLE COMMANDS:\n\
                            hack - Earn credits\n\
                            buy <item> - Purchase upgrades\n\
                            status - Show current status\n\
                            shop - Show available items\n\
                            echo <message> - Echo a message\n\
                            clear - Clear terminal\n\
                            help - Show this help".to_string();
}

fn cmd_hack(state: &mut GameState) {
    let hack_value : f64 = 1.0 * state.resources.hack_multiplier;
    state.resources.credits += hack_value;
    state.terminal.command_output = format!("HACK SUCCESSFUL. +{} CREDITS", hack_value);
}

fn cmd_status(state: &mut GameState) {
    state.terminal.command_output = format!("CREDITS: {}\n\
                            CREDITS PER SECOND: {}\n",
                            state.resources.credits as i64,
                            state.resources.credits_per_second as i64);
}

fn cmd_buy(name: &str, state: &mut GameState) {
    match name {
        "port-scan" => {
            if state.resources.credits < 10.0 {
                state.terminal.command_output = "NOT ENOUGH CREDITS".to_string();
                return;
            }
            state.resources.credits -= 10.0;
            state.resources.credits_per_second += 1.0;
            state.terminal.command_output = format!("PURCHASED 1 PORT-SCAN UPGRADE. CREDITS PER SECOND: {}",
                                            state.resources.credits_per_second);
        }

        "upgrade" => {
            if state.resources.credits < 50.0 {
                state.terminal.command_output = "NOT ENOUGH CREDITS".to_string();
                return;
            }
            state.resources.credits -= 50.0;
            state.resources.hack_multiplier += 1.0;
            state.terminal.command_output = format!("PURCHASED 1 UPGRADE. HACK MULTIPLIER: {}",
                                            state.resources.hack_multiplier);
        }
        _ => {
            state.terminal.command_output = format!("UNKNOWN ITEM: '{}'", name);
        }
    }
}

fn cmd_shop(state: &mut GameState) {
    state.terminal.command_output = "SHOP:\n\
                            port-scan - 10 credits\n\
                            upgrade - 50 credits\n".to_string();
}

fn cmd_echo(args: &[&str], state: &mut GameState) {
    if args.is_empty() {
        state.terminal.command_output = "USAGE: echo <message>".to_string();
    } else {
        state.terminal.command_output = args.join(" ");
    }
}