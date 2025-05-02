use std::time::Duration;

pub struct GameState {
    pub resources: Resources,
    //pub store: Store,
    //pub upgrades: Vec<String>,
    // pub has_status_panel: bool,
    // pub has_upgrades_panel: bool,
    pub network: Network,
    pub command_history: Vec<String>,
    pub command_output: String
}

impl GameState {
    pub fn new() -> Self {
        Self {
            resources: Resources::new(),
            // has_status_panel: false,
            // has_upgrades_panel: false,
            command_history: Vec::new(),
            network: Network::new(),
            command_output: "SYSTEM INITIALIZED. TYPE 'help' FOR COMMANDS.".to_string(),
        }
    }
    
    pub fn update(&mut self, elapsed: Duration) {
        // Update resources based on automations
        let seconds = elapsed.as_secs_f64();
        self.resources.credits += self.resources.credits_per_second as f64 * seconds; 
    }
}

pub struct Resources {
    pub credits: f64,
    pub credits_per_second: f64,
    pub hack_multiplier: f64,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            credits: 0.0,
            credits_per_second: 0.0,
            hack_multiplier: 1.0,
        }
    }
}






// The player can connect to different shells on their network
// Each shell has a different address and can be hacked for different amounts of credits
use std::collections::HashMap;

// How should we represent the shells? IP address isn't an actual address, just a string. We don't need to do any networking. It's all a game!
pub struct Shell {
    pub address: String,
    pub credit_reward: f64,
    pub is_hacked: bool,
    pub cps: f64,
}

impl Shell {
    pub fn new(address: &str, credit_reward: f64) -> Self {
        Self {
            address: address.to_string(),
            credit_reward,
            is_hacked: false,
            cps: 0.0,
        }
    }

    pub fn hack(&mut self, credits: &mut f64) {
        if !self.is_hacked {
            self.is_hacked = true;
            // Add the credit reward to the player's resources
            // self.resources.credits += self.credit_reward;
            *credits += self.credit_reward;
        }
    }

}

// The gamestate network is a collection of shells and their addresses
pub struct Network {
    pub shells: HashMap<String, Shell>,
}


impl Network {
    pub fn new() -> Self {
        let mut shells = HashMap::new();
        shells.insert("derelict1".to_string(), Shell { address: "derelict1".to_string(), credit_reward: 10.0 });
        shells.insert("derelict2".to_string(), Shell { address: "derelict2".to_string(), credit_reward: 20.0 });
        shells.insert("derelict3".to_string(), Shell { address: "derelict3".to_string(), credit_reward: 30.0 });
    }
}

