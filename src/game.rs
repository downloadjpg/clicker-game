use ratatui::Terminal;

use crate::state::GameState;
use crate::ui::UiManager;
use crate::input::{CommandProcessor, EventHandler};
use std::time::{Duration, Instant};

pub fn run<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize game state
    let mut game_state = GameState::new();
    let ui_manager = UiManager::new();
    let command_processor = CommandProcessor::new();
    let mut event_handler = EventHandler::new();

    // Main game loop
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    loop {
        // Update UI
        terminal.draw(|frame| {
            ui_manager.render(frame, &game_state, &event_handler);
        })?;

        // Handle input events // TODO unsure!
        if let Some(event) = event_handler.next(Duration::from_millis(10))? {

            if event_handler.should_quit(&event) {
                break;
            }
            if let Some(command) = event_handler.handle_input(&event) {
                command_processor.process_command(&command, &mut game_state);
            }
        }
        
        // Update game state on tick
        if last_tick.elapsed() >= tick_rate {
            game_state.update(last_tick.elapsed());
            last_tick = Instant::now();
        }
    }
    Ok(())
}