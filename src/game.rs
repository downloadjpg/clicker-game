use ratatui::Terminal;
use std::time::{Duration, Instant};

mod ui;
mod state;
mod game_terminal;

use ui::UiManager;
use state::GameState;

const TICKS_PER_SECOND: u64 = 60;
const FRAMES_PER_SECOND: u64 = 30;
const TICK_RATE: Duration = Duration::from_millis(1000 / TICKS_PER_SECOND);
const FRAME_RATE: Duration = Duration::from_millis(1000 / FRAMES_PER_SECOND);


pub fn run<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize game state
    let mut game_state = GameState::new();
    let mut ui_manager = UiManager::new();
    let mut event_bus = EventBus::new();
    let mut input_handler = InputHandler::new();

    
    // Initialize when the 'last update' was for the state and render
    let mut last_tick = Instant::now();
    let mut last_frame = Instant::now();

    // Main game loop
    loop {

        // Poll for input
        input_handler.poll(&mut event_bus);

        // Process events
        while let Some(event) = event_bus.pop() {
            match event {
                Event::UserInput(key_event) => {
                    game_state.terminal.process_input(key_event, &mut event_bus);
                }
                Event::Tick(duration) => {
                    game_state.update(duration);
                }
                Event::Command(command) => {
                    game_state.process_command(command);
                }
                Event::Render => {
                    // Update UI
                    ui_manager.draw(&game_state, terminal);
                }
                Event::Quit => {
                    return Ok(()); // Exit game loop
                }
            }
        }

        
        
        // Update game state on tick
        if last_tick.elapsed() >= TICK_RATE {
            event_bus.push(Event::Tick(last_tick.elapsed()));
            last_tick = Instant::now();
        }

        if last_frame.elapsed() >= FRAME_RATE {
            event_bus.push(Event::Render);
            last_frame = Instant::now();
        }
    }
}


pub enum Event {
    UserInput(KeyEvent),
    Tick(Duration),
    Render,
    Command(String),
    Quit
}



use std::collections::VecDeque;
pub struct EventBus {
    events: VecDeque<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}

use crossterm::event::{self, KeyCode, KeyEvent,};
use crossterm::event::Event as InputEvent;

pub struct InputHandler {}

impl InputHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn poll(&mut self, event_bus: &mut EventBus) {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let InputEvent::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc => {
                        event_bus.push(Event::Quit);
                    }
                    _ =>{
                        event_bus.push(Event::UserInput(key_event));
                    }
                }
            }
        }
    }
}