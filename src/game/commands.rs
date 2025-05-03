use super::GameState;

use crossterm::style::{Color, Stylize};



pub fn process(cmd: &str, state: &mut GameState) {
    let cmd_parts: Vec<&str> = cmd.trim().split_whitespace().collect();
    if cmd_parts.is_empty() {
        return;
    }
    
    // Process command
    match cmd_parts[0] {
        "help" => cmd_help(state),
        "buy" if cmd_parts.len() > 1 => cmd_buy(cmd_parts[1], state),
        "status" => cmd_status(state),
        "echo" => cmd_echo(&cmd_parts[1..], state),
        "shop" => cmd_shop(state),
        "port-scan" => cmd_port_scan(state),
        "hack" if cmd_parts.len() > 1 => cmd_hack(state, &cmd_parts[1..]),
        _ => state.terminal.command_output = format!("UNKNOWN COMMAND: '{}'", cmd_parts[0]),
    }
}

fn cmd_help(state: &mut GameState) {
    state.terminal.command_output = "AVAILABLE COMMANDS:\n\
                            hack - Hack an address\n\
                            buy <item> - Purchase upgrades\n\
                            status - Show current status\n\
                            shop - Show available items\n\
                            echo <message> - Echo a message\n\
                            clear - Clear terminal\n\
                            help - Show this help".to_string();
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

fn cmd_port_scan(state: &mut GameState) {
    // Get the list of shells from the network
    let shells = state.network.get_shells();
    if shells.is_empty() {
        state.terminal.command_output = "NO SHELLS FOUND".to_string();
        return;
    }

    // Build the output with colored text
    let mut output = String::from("AVAILABLE SHELLS:\n");
    for shell in shells {
        let color = if shell.is_hacked { Color::Green } else { Color::Red };
        output.push_str(&format!(
            "{} - {} CREDITS\n",
            shell.address.to_string().with(color),
            shell.credit_reward.to_string().with(Color::Yellow)
        ));
    }

    state.terminal.command_output = output;
}

fn cmd_hack(state: &mut GameState, args: &[&str]) {
   if args.is_empty() {
        state.terminal.command_output = "USAGE: ssh <address>".to_string();
        return;
    }

    let address = args[0];
    if let Some(shell) = state.network.get_mut_shell(address) {
        if shell.is_hacked {
            state.terminal.command_output = format!("ALREADY HACKED: {}", address);
        } else {
            shell.hack(&mut state.resources.credits);
            state.terminal.command_output = format!("HACKED: {} - {} CREDITS", address, shell.credit_reward);
        }
    } else {
        state.terminal.command_output = format!("UNKNOWN ADDRESS: {}", address);
    } 
}