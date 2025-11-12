mod docker;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use ui::{render, App};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let mut app = App::new()?;
    
    app.refresh_containers().await?;
    
    let result = run_app(&mut terminal, &mut app).await;
    
    restore_terminal(&mut terminal)?;
    
    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
    
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    let mut last_refresh = std::time::Instant::now();
    let refresh_interval = std::time::Duration::from_millis(500);
    let mut last_status_clear = std::time::Instant::now();
    
    loop {
        terminal.draw(|f| render(f, app))?;
        
        if app.status_message.is_some() && last_status_clear.elapsed() > std::time::Duration::from_secs(3) {
            app.clear_status();
        }
        
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if app.show_delete_confirm {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            app.confirm_delete().await?;
                            last_refresh = std::time::Instant::now();
                            last_status_clear = std::time::Instant::now();
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            app.cancel_delete();
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Down | KeyCode::Char('j') => app.next_container(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous_container(),
                        KeyCode::Char('s') => {
                            app.start_selected().await?;
                            app.refresh_containers().await?;
                            last_refresh = std::time::Instant::now();
                            last_status_clear = std::time::Instant::now();
                        }
                        KeyCode::Char('x') => {
                            app.stop_selected().await?;
                            app.refresh_containers().await?;
                            last_refresh = std::time::Instant::now();
                            last_status_clear = std::time::Instant::now();
                        }
                        KeyCode::Char('r') => {
                            app.restart_selected().await?;
                            app.refresh_containers().await?;
                            last_refresh = std::time::Instant::now();
                            last_status_clear = std::time::Instant::now();
                        }
                        KeyCode::Char('d') => {
                            app.request_delete_confirm();
                        }
                        KeyCode::Char('R') => {
                            app.refresh_containers().await?;
                            last_refresh = std::time::Instant::now();
                        }
                        _ => {}
                    }
                }
            }
        }
        
        if last_refresh.elapsed() >= refresh_interval {
            app.refresh_containers().await?;
            last_refresh = std::time::Instant::now();
        }
        
        if app.should_quit {
            break;
        }
    }
    
    Ok(())
}