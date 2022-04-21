use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::model;

enum ActivePane {
    Memory,
    Menu,
    Setting,
}

/// App holds the state of the application
struct App {
    input: model::Config,
    ui_state: UiState,
}

struct UiState {
    memory: u16,
    menu: u16,
    setting: u16,
    active_pane: ActivePane,
}

impl Default for UiState {
    fn default() -> UiState {
        UiState {
            memory: 0,
            menu: 0,
            setting: 0,
            active_pane: ActivePane::Memory,
        }
    }
}

pub fn init(config: model::Config) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let ui_state = UiState::default();
    let app = App {
        input: config,
        ui_state: ui_state,
    };
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.ui_state.active_pane {
                ActivePane::Memory => match key.code {
                    KeyCode::Up => {
                        if app.ui_state.memory > 0 {
                            app.ui_state.memory = app.ui_state.memory - 1
                        }
                    }
                    KeyCode::Down => {
                        if (app.ui_state.memory as usize) < app.input.memories.len() - 1 {
                            app.ui_state.memory = app.ui_state.memory + 1
                        }
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
        .split(f.size());

    let (msg, style) = (
        vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let messages: Vec<ListItem> = app
        .input
        .memories
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let style = if (i == app.ui_state.memory as usize) {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            let content = vec![Spans::from(Span::raw(format!("MEMORY {}", m.id)))];
            ListItem::new(content)
                .style(style)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[1]);
}
