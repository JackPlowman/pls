use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs;
use std::path::PathBuf;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

struct AppState {
    current_dir: PathBuf,
    left_selected: Option<usize>,
    right_selected: Option<usize>,
}

impl AppState {
    fn new() -> Self {
        Self {
            current_dir: PathBuf::from("."),
            left_selected: Some(0),
            right_selected: None,
        }
    }
}

fn get_directory_contents(path: &PathBuf) -> Vec<(String, bool)> {
    fs::read_dir(path)
        .unwrap_or_else(|_| fs::read_dir(".").unwrap())
        .filter_map(Result::ok)
        .map(|entry| {
            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            let name = entry.file_name().into_string().unwrap_or_default();
            (name, is_dir)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::new();
    let result = run_app(&mut terminal, &mut state);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut AppState) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

            // Left pane
            let contents = get_directory_contents(&state.current_dir);
            let items: Vec<ListItem> = contents
                .iter()
                .map(|(name, is_dir)| {
                    let prefix = if *is_dir { "📁 " } else { "📄 " };
                    ListItem::new(format!("{}{}", prefix, name))
                })
                .collect();

            let left_list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Current Directory"),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            // Right pane
            let right_contents = if let Some(idx) = state.left_selected {
                if let Some((name, true)) = contents.get(idx) {
                    let mut child_path = state.current_dir.clone();
                    child_path.push(name);
                    get_directory_contents(&child_path)
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            let right_items: Vec<ListItem> = right_contents
                .iter()
                .map(|(name, is_dir)| {
                    let prefix = if *is_dir { "📁 " } else { "📄 " };
                    ListItem::new(format!("{}{}", prefix, name))
                })
                .collect();

            let right_list = List::new(right_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Selected Directory"),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let mut left_state = ListState::default();
            left_state.select(state.left_selected);
            f.render_stateful_widget(left_list, chunks[0], &mut left_state);

            let mut right_state = ListState::default();
            right_state.select(state.right_selected);
            f.render_stateful_widget(right_list, chunks[1], &mut right_state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {
                    let contents = get_directory_contents(&state.current_dir);
                    if let Some(selected) = state.left_selected {
                        state.left_selected = Some((selected + 1) % contents.len());
                        state.right_selected = None;
                    }
                }
                KeyCode::Up => {
                    let contents = get_directory_contents(&state.current_dir);
                    if let Some(selected) = state.left_selected {
                        state.left_selected =
                            Some(selected.checked_sub(1).unwrap_or(contents.len() - 1));
                        state.right_selected = None;
                    }
                }
                KeyCode::Enter => {
                    let contents = get_directory_contents(&state.current_dir);
                    if let Some(selected) = state.left_selected {
                        if let Some((name, true)) = contents.get(selected) {
                            state.current_dir.push(name);
                            state.left_selected = Some(0);
                            state.right_selected = None;
                        }
                    }
                }
                KeyCode::Backspace => {
                    if state.current_dir.pop() {
                        state.left_selected = Some(0);
                        state.right_selected = None;
                    }
                }
                _ => {}
            }
        }
    }
}
