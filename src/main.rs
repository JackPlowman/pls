use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Create a list of folder names
            let folders = fs::read_dir(".")
                .unwrap()
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    entry
                        .file_type()
                        .ok()
                        .filter(|ft| ft.is_dir())
                        .map(|_| entry)
                })
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect::<Vec<_>>();

            let items: Vec<ListItem> = folders.iter().map(|i| ListItem::new(i.as_ref())).collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Folders"))
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            f.render_widget(list, chunks[0]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
