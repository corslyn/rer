pub mod api;
pub mod app;
pub mod siri;
mod ui;
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    let api_key = std::env::var("API_KEY")?;
    let gare = "45102"; // Chatelet les Halles
    let response = api::request(gare, &api_key)?;
    let siri = api::parse_siri(&response)?;

    let mut terminal = setup_terminal()?;
    let mut app = app::App::new(gare, api_key, siri);
    let mut input_focused = true;

    loop {
        terminal.draw(|frame| ui::ui(frame, &app, input_focused))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Enter => app.refresh(),
                KeyCode::Backspace if input_focused => {
                    app.input.pop();
                }
                KeyCode::Char(character) if input_focused => {
                    app.input.push(character)
                }
                KeyCode::Tab => input_focused = !input_focused,
                _ => {}
            }
        }
    }

    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
