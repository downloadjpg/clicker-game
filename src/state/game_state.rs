use std::time::Duration;

pub struct GameState {
    pub resources: Resources,
    pub has_status_panel: bool,
    pub has_upgrades_panel: bool,
    pub command_history: Vec<String>,
    pub command_output: String
}

impl GameState {
    pub fn new() -> Self {
        Self {
            resources: Resources::new(),
            has_status_panel: false,
            has_upgrades_panel: false,
            command_history: Vec::new(),
            command_output: "SYSTEM INITIALIZED. TYPE 'help' FOR COMMANDS.".to_string(),
        }
    }
    
    pub fn update(&mut self, elapsed: Duration) {
        // Update resources based on automations
        let seconds = elapsed.as_secs_f32();
        self.resources.credits += (self.resources.credits_per_second as f32 * seconds) as u64; 
    }
}

pub struct Resources {
    pub credits: u64,
    pub credits_per_second: u32
}

impl Resources {
    pub fn new() -> Self {
        Self {
            credits: 0,
            credits_per_second: 0,
        }
    }
}
