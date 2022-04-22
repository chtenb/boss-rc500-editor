use crossterm::event::KeyEvent;
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

use crate::model;

#[derive(Debug, PartialEq, Eq)]
enum Focus {
    Memory,
    Menu,
    Setting,
    Edit,
}

/// App holds the state of the application
struct App {
    config: model::Config,
    ui_state: UiState,
}

struct UiState {
    memory: usize,
    menu: usize,
    setting: usize,
    focus: Focus,
}

impl Default for UiState {
    fn default() -> UiState {
        UiState {
            memory: 0,
            menu: 0,
            setting: 0,
            focus: Focus::Memory,
        }
    }
}

fn nr_memories(app: &App) -> usize {
    app.config.memories.len()
}

fn nr_menus(app: &App) -> usize {
    app.config.memories[0].menus.len()
}

fn nr_settings(app: &App) -> usize {
    app.config.memories[0].menus[app.ui_state.menu]
        .settings
        .len()
}

fn get_selected_memory(app: &App) -> &model::Memory {
    &app.config.memories[app.ui_state.memory]
}

fn get_selected_menu(app: &App) -> &model::UntypedMenu {
    let selected_memory = get_selected_memory(app);
    &selected_memory.menus[app.ui_state.menu]
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
            match handle_input(&mut app, key) {
                Ok(()) => continue,
                Err(()) => return Ok(()),
            };
        }
    }
}

fn modulo(n: i32, m: i32) -> i32 {
    i32::rem_euclid(n as i32, m as i32) as i32
}

fn dec_modulo(n: usize, m: usize) -> usize {
    modulo(n as i32 - 1, m as i32) as usize
}

fn inc_modulo(n: usize, m: usize) -> usize {
    modulo(n as i32 + 1, m as i32) as usize
}

fn handle_input(app: &mut App, key: KeyEvent) -> Result<(), ()> {
    match app.ui_state.focus {
        Focus::Memory => match key.code {
            KeyCode::Up => app.ui_state.memory = dec_modulo(app.ui_state.memory, nr_memories(&app)),
            KeyCode::Down => {
                app.ui_state.memory = inc_modulo(app.ui_state.memory, nr_memories(&app))
            }
            KeyCode::Right => app.ui_state.focus = Focus::Menu,
            KeyCode::Char('q') => return Err(()),
            _ => {}
        },
        Focus::Menu => match key.code {
            KeyCode::Up => app.ui_state.menu = dec_modulo(app.ui_state.menu, nr_menus(&app)),
            KeyCode::Down => app.ui_state.menu = inc_modulo(app.ui_state.menu, nr_menus(&app)),
            KeyCode::Left => app.ui_state.focus = Focus::Memory,
            KeyCode::Right => app.ui_state.focus = Focus::Setting,
            KeyCode::Char('q') => return Err(()),
            _ => {}
        },
        Focus::Setting => match key.code {
            KeyCode::Up => {
                app.ui_state.setting = dec_modulo(app.ui_state.setting, nr_settings(&app))
            }
            KeyCode::Down => {
                app.ui_state.setting = inc_modulo(app.ui_state.setting, nr_settings(&app))
            }
            KeyCode::Left => app.ui_state.focus = Focus::Menu,
            KeyCode::Enter => app.ui_state.focus = Focus::Edit,
            KeyCode::Char('q') => return Err(()),
            _ => {}
        },
        Focus::Edit => match key.code {
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Enter | KeyCode::Esc => app.ui_state.focus = Focus::Setting,
            _ => {}
        },
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
        .split(f.size());

    let (msg, style) = if app.ui_state.focus == Focus::Edit {
        (
            vec![Span::styled(
                "Editing value",
                Style::default().fg(Color::Magenta),
            )],
            Style::default()
        )
    } else {
        (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().fg(Color::Red)),
                Span::raw(" to exit"),
            ],
            Style::default()
        )
    };
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
                let style = if i == app.ui_state.memory {
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

        let menus_style = match app.ui_state.focus {
            Focus::Memory => Style::default().add_modifier(Modifier::DIM).fg(Color::DarkGray),
            _ => Style::default(),
        };
        let selected_memory = get_selected_memory(app);
        let menus: Vec<ListItem> = selected_memory
            .menus
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let style = if i == app.ui_state.menu {
                    menus_style.add_modifier(Modifier::REVERSED)
                } else {
                    menus_style
                };
                let content = vec![Spans::from(Span::raw(format!("{}", m.name)))];
                ListItem::new(content).style(style)
            })
            .collect();
        let menus = List::new(menus).block(Block::default().borders(Borders::ALL).title("MENUS"));
        f.render_widget(menus, chunks[1]);

        let settings_style = match app.ui_state.focus {
            Focus::Memory | Focus::Menu => Style::default().add_modifier(Modifier::DIM).fg(Color::DarkGray),
            Focus::Setting | Focus::Edit => Style::default(),
        };
        let selected_menu = get_selected_menu(app);
        let settings: Vec<ListItem> = selected_menu
            .settings
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let style = if i == app.ui_state.setting {
                    if app.ui_state.focus == Focus::Edit {
                        settings_style.fg(Color::Red)
                    } else if app.ui_state.focus == Focus::Setting {
                        settings_style.add_modifier(Modifier::REVERSED)
                    } else {
                        settings_style
                    }
                } else {
                    settings_style
                };
                let content = vec![Spans::from(Span::raw(format!("{} = {}", m.key, m.value)))];
                ListItem::new(content).style(style)
            })
            .collect();
        let settings =
            List::new(settings).block(Block::default().borders(Borders::ALL).title("SETTINGS"));
        f.render_widget(settings, chunks[2]);
    }
}
