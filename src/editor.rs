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

#[derive(Debug, PartialEq, Eq)]
enum ActivePane {
    Memory,
    Menu,
    Setting,
}

/// App holds the state of the application
struct App {
    config: model::Config,
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
        config: config,
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
                        if (app.ui_state.memory as usize) < app.config.memories.len() - 1 {
                            app.ui_state.memory = app.ui_state.memory + 1
                        }
                    }
                    KeyCode::Right => {
                        app.ui_state.active_pane = ActivePane::Menu;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                ActivePane::Menu => match key.code {
                    KeyCode::Up => {
                        if app.ui_state.menu > 0 {
                            app.ui_state.menu = app.ui_state.menu - 1
                        }
                    }
                    KeyCode::Down => {
                        if (app.ui_state.menu as usize) < nr_menus(&app) - 1 {
                            app.ui_state.menu = app.ui_state.menu + 1
                        }
                    }
                    KeyCode::Left => {
                        app.ui_state.active_pane = ActivePane::Memory;
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

fn nr_menus(app: &App) -> usize {
    app.config.memories[0].menus.len()
}

fn get_selected_memory(app: &App) -> &model::Memory {
    &app.config.memories[app.ui_state.memory as usize]
}

fn get_selected_menu(app: &App) -> Option<&model::UntypedMenu> {
    if app.ui_state.active_pane == ActivePane::Memory{
        return None;
    }
    let selected_memory = get_selected_memory(app);
    Some(&selected_memory.menus[app.ui_state.menu as usize])
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
            Span::raw(" to exit"),
        ],
        Style::default().add_modifier(Modifier::BOLD),
    );
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let memories: Vec<ListItem> = app
            .config
            .memories
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let style = if i == app.ui_state.memory as usize {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                let content = vec![Spans::from(Span::raw(format!("MEMORY {}", m.id)))];
                ListItem::new(content).style(style)
            })
            .collect();
        let memories =
            List::new(memories).block(Block::default().borders(Borders::ALL).title("MEMORY SLOTS"));
        f.render_widget(memories, chunks[0]);

        let selected_memory = get_selected_memory(app);
        let menus: Vec<ListItem> = selected_memory
            .menus
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let style = if i == app.ui_state.menu as usize && app.ui_state.active_pane != ActivePane::Memory {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                let content = vec![Spans::from(Span::raw(format!("{}", m.name)))];
                ListItem::new(content).style(style)
            })
            .collect();
        let menus = List::new(menus).block(Block::default().borders(Borders::ALL).title("MENUS"));
        f.render_widget(menus, chunks[1]);

        let settings: Vec<ListItem> = match get_selected_menu(app){
            None => Vec::new(),
            Some(menu) => menu
                .settings
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let style = if i == app.ui_state.setting as usize && app.ui_state.active_pane == ActivePane::Setting{
                        Style::default().add_modifier(Modifier::REVERSED)
                    } else {
                        Style::default()
                    };
                    let content = vec![Spans::from(Span::raw(format!("{} = {}", m.key, m.value)))];
                    ListItem::new(content).style(style)
                })
                .collect()
        };
        let menus = List::new(settings).block(Block::default().borders(Borders::ALL).title("MENUS"));
        f.render_widget(menus, chunks[2]);
    }
}
