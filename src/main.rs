use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::ExecutableCommand,
    crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    panic,
};

mod game;
mod state;
mod ui;
mod input;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    setup_terminal()?;
    // Initialize terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    // Clear the screen
    crossterm::execute!(std::io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
    enable_raw_mode()?;
    // Set the terminal size
    // Run tha game!
    let result = game::run(&mut terminal);
    // Restore terminal on exit
    restore_terminal()?;
    result
}


fn setup_terminal() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    
    // Set up panic hook to restore terminal on panic
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Try to restore terminal
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
        
        // Call the original hook
        original_hook(panic_info);
    }));
    
    Ok(())
}

fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}