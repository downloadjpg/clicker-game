use std::time::Duration;
use super::game_terminal::GameTerminal;


pub struct GameState {
    pub resources: Resources,
    pub terminal: GameTerminal,
    pub network: Network,
    // pub commands: HashMap<String, Box<dyn Fn(&mut GameState, Vec<String>)>>,

}
impl GameState {
    pub fn new() -> Self {
        Self {
            resources: Resources::new(),
            terminal: GameTerminal::new(),
            network: Network::new(),
            // commands: HashMap::new(),
        }
    }
    
    pub fn update(&mut self, elapsed: Duration) {
        // Update resources based on automations
        let seconds = elapsed.as_secs_f64();
        self.resources.credits += self.resources.credits_per_second as f64 * seconds; 
    }

    pub fn set_output(&mut self, output: &str) { // more of a helper than a setter. reduces path chasing.
        self.terminal.command_output = output.to_string();
    }
    
    pub fn process_command(&mut self, command: String) {
        let cmd_parts = command.split_whitespace().collect::<Vec<_>>();
        if cmd_parts.is_empty() {
            return;
        }
        let command = cmd_parts[0];
        let args = &cmd_parts[1..];
        match command {
            "hack" => self.cmd_hack(args),
            "help" => self.cmd_help(args),
            // todo
            "install" => self.cmd_install(args),
            "scan" => self.cmd_scan(args),
            "ssh" => self.cmd_ssh(args),
            _ => self.set_output("COMMAND NOT FOUND"),
        }
    }

    fn cmd_hack(&mut self, args: &[&str]) {
        if args.len() == 0 {
            self.set_output("USAGE: hack [address]");
            return;
        }
        let address = args[0];

        let hack_result = if let Some(node) = self.network.get_mut_node(address) {
            if node.is_hacked {
                "NODE ALREADY HACKED"
            }
            else {
                node.is_hacked = true;
                self.resources.credits += node.credit_reward;
                &format!("HACKED {} FOR {} CREDITS", address, node.credit_reward)
            }
        } else {
            "NODE NOT FOUND"
        };

        self.set_output(hack_result);
    }

    fn cmd_help(&mut self, _args: &[&str]) {
        self.set_output(
            "Available commands:
            ssh [target]      - Connect to controlled node
            hack [target]     - Attempt to gain control of target node
            install [program] - Install software on current node
            scan              - Search for available nodes
            help             - Show this help message
        ");
    }

    fn cmd_ssh(&mut self, args: &[&str]) {
        if args.len() == 0 {
            self.set_output("USAGE: ssh [address]");
            return;
        }
        let address = args[0];

        if  self.network.is_visible(address) {
            self.network.set_current_address(address);
            self.set_output(&format!("Connected to {}", address));
        } else {
            self.set_output("NODE NOT FOUND");
        }
    }

    fn cmd_scan(&mut self, _args: &[&str]) {
        // Get current node and its connections. Add them to the visible nodes.
    }
    fn cmd_install(&mut self, args: &[&str]) {
        todo!()
    }
    
    
}

pub struct Resources {
    pub credits: f64,
    pub credits_per_second: f64,
    pub compute_power: u64,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            credits: 0.0,
            credits_per_second: 0.0,
            compute_power: 1,
        }
    }
}





// The player can connect to different shells on their network
// Each shell has a different address and can be hacked for different amounts of credits
use std::collections::HashMap;

// How should we represent the shells? IP address isn't an actual address, just a string. We don't need to do any networking. It's all a game!
pub struct Node {
    pub address: String,
    pub credit_reward: f64,
    pub is_hacked: bool,
    pub program_capacity: usize,
    pub programs: Vec<Program>,
    pub connected_nodes: Vec<String>,
}

impl Node {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            credit_reward: 0.0,
            is_hacked: false,
            program_capacity: 1,
            programs: vec![],
            connected_nodes: vec![],
        }
    }
    // Root node, the user!
    pub fn root() -> Self {
        Self::new("NEXUS-CORE")
        .with_connections(vec!["RAN-A13".to_string()])
    }
    pub fn with_connections(mut self, connections: Vec<String>) -> Self {
        self.connected_nodes = connections;
        self
    }
    pub fn with_credits(mut self, credits: f64) -> Self {
        self.credit_reward = credits;
        self
    }
    pub fn with_program_capacity(mut self, capacity: usize) -> Self {
        self.program_capacity = capacity;
        self
    }
    
}

enum Program {
    // Programs that can be installed on the nodes
    // These are just placeholders for now
    Miner,
    GPU,
}

// The gamestate network is a collection of shells and their addresses
pub struct Network {
    current_address: String,
    visible_nodes: Vec<String>,
    nodes: HashMap<String, Node>,
}


impl Network {
    pub fn new() -> Self {
        Self::from_node_list(vec![
            Node::root(),
            Node::new("RAN-A13")
                .with_credits(10.0)
                .with_program_capacity(2)
                .with_connections(vec![
                    "RAN-A45".to_string(),
                    "RAN-K56".to_string()
                ]),
            Node::new("RAN-A45")
                .with_credits(5.0)
                .with_program_capacity(1),
            Node::new("RAN-K56")
                .with_credits(20.0)
                .with_program_capacity(3)
        ])
    }

    fn from_node_list(list: Vec<Node>) -> Self {
        let mut nodes = HashMap::new();
        for node in list {
            nodes.insert(node.address.clone(), node);
        }
        Self {
            current_address: "NEXUS-CORE".to_string(),
            visible_nodes: vec![],
            nodes
        }
    }

    pub fn get_mut_node(&mut self, address: &str) -> Option<&mut Node> {
        self.nodes.get_mut(address)
    }
    pub fn set_current_address(&mut self, address: &str) {
        self.current_address = address.to_string();
    }
    pub fn is_visible(&self, address: &str) -> bool {
        self.nodes.contains_key(address)
    }

}