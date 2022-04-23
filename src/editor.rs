use crossterm::event::KeyEvent;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::cmp::min;
use std::{error::Error, io};
use tui::layout::Rect;
use tui::widgets::ListState;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::arith;
use crate::model;

#[derive(Debug, PartialEq, Eq)]
enum Focus {
    Memory,
    Menu,
    Setting,
    Edit,
}
impl Default for Focus {
    fn default() -> Focus {
        Focus::Memory
    }
}

#[derive(Default)]
struct BoundedIndex {
    index: usize,
}

impl BoundedIndex {
    fn get(&self, upper_bound: usize) -> usize {
        min(self.index, upper_bound - 1)
    }
    fn dec(&mut self, upper_bound: usize) {
        self.index = arith::modulo(self.get(upper_bound) as i32 - 1, upper_bound as i32) as usize;
    }
    fn inc(&mut self, upper_bound: usize) {
        self.index = arith::modulo(self.get(upper_bound) as i32 + 1, upper_bound as i32) as usize;
    }
}

#[derive(Default)]
struct MemoryIndex(BoundedIndex);
#[derive(Default)]
struct MenuIndex(BoundedIndex);
#[derive(Default)]
struct SettingIndex(BoundedIndex);

#[derive(Default)]
struct UiState {
    memory: MemoryIndex,
    menu: MenuIndex,
    setting: SettingIndex,
    focus: Focus,

    memory_state: ListState,
    menu_state: ListState,
    setting_state: ListState,
}

fn nr_memories(config: &model::Config) -> usize {
    config.memories.len()
}

fn nr_menus(config: &model::Config) -> usize {
    config.memories[0].menus.len()
}

fn nr_settings(config: &model::Config, ui_state: &UiState) -> usize {
    get_selected_menu(config, ui_state).settings.len()
}

fn get_selected_memory<'a>(config: &'a model::Config, ui_state: &UiState) -> &'a model::Memory {
    &config.memories[ui_state.memory.0.get(nr_memories(config))]
}

fn get_selected_menu<'a>(config: &'a model::Config, ui_state: &UiState) -> &'a model::UntypedMenu {
    let selected_memory = get_selected_memory(config, ui_state);
    &selected_memory.menus[ui_state.menu.0.get(nr_menus(config))]
}

fn get_selected_setting_mut<'a>(config: &'a mut model::Config, ui_state: &UiState) -> &'a mut model::UntypedKeyValue {
    let nr_memories = nr_memories(config);
    let nr_menus = nr_menus(config);
    let selected_memory = &mut config.memories[ui_state.memory.0.get(nr_memories)];
    let selected_menu = &mut selected_memory.menus[ui_state.menu.0.get(nr_menus)];
    &mut selected_menu.settings[ui_state.setting.0.get(nr_menus)]
}

pub fn init(config: &mut model::Config) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut ui_state = UiState::default();
    let res = run_app(&mut terminal, config, &mut ui_state);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    config: &mut model::Config,
    ui_state: &mut UiState,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, config, ui_state))?;

        if let Event::Key(key) = event::read()? {
            match handle_input(config, ui_state, key) {
                Ok(()) => continue,
                Err(()) => return Ok(()),
            };
        }
    }
}

fn handle_input(config: &mut model::Config, ui_state: &mut UiState, key: KeyEvent) -> Result<(), ()> {
    match ui_state.focus {
        Focus::Memory => match key.code {
            KeyCode::Up => ui_state.memory.0.dec(nr_memories(config)),
            KeyCode::Down => ui_state.memory.0.inc(nr_memories(config)),
            KeyCode::Right => ui_state.focus = Focus::Menu,
            KeyCode::Char('q') => return Err(()),
            _ => {}
        },
        Focus::Menu => match key.code {
            KeyCode::Up => ui_state.menu.0.dec(nr_menus(config)),
            KeyCode::Down => ui_state.menu.0.inc(nr_menus(config)),
            KeyCode::Left => ui_state.focus = Focus::Memory,
            KeyCode::Right => ui_state.focus = Focus::Setting,
            KeyCode::Char('q') => return Err(()),
            _ => {}
        },
        Focus::Setting => match key.code {
            KeyCode::Up => ui_state.setting.0.dec(nr_settings(config, ui_state)),
            KeyCode::Down => ui_state.setting.0.inc(nr_settings(config, ui_state)),
            KeyCode::Left => ui_state.focus = Focus::Menu,
            KeyCode::Enter => ui_state.focus = Focus::Edit,
            KeyCode::Char('q') => return Err(()),
            _ => {}
        },
        Focus::Edit => match key.code {
            KeyCode::Up => {
                let setting = get_selected_setting_mut(config, ui_state);
                setting.value += 1;
            }
            KeyCode::Down => {
                let setting = get_selected_setting_mut(config, ui_state);
                setting.value -= 1;
            }
            KeyCode::Enter | KeyCode::Esc => ui_state.focus = Focus::Setting,
            _ => {}
        },
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, config: &model::Config, ui_state: &mut UiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(5)].as_ref())
        .split(f.size());

    render_help(f, chunks[0], ui_state);
    render_description(f, chunks[2], config, ui_state);

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

        render_memories(f, chunks[0], config, ui_state);
        render_menus(f, chunks[1], config, ui_state);
        render_settings(f, chunks[2], config, ui_state);
    }
}

fn render_help<B: Backend>(f: &mut Frame<B>, rect: Rect, ui_state: &mut UiState) {
    let (msg, style) = if ui_state.focus == Focus::Edit {
        (
            vec![Span::styled("Editing value", Style::default().fg(Color::Magenta))],
            Style::default(),
        )
    } else {
        (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().fg(Color::Red)),
                Span::raw(" to exit"),
            ],
            Style::default(),
        )
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, rect);
}

fn render_description<B: Backend>(f: &mut Frame<B>, rect: Rect, config: &model::Config, ui_state: &mut UiState) {
    let (description, style) = (
        vec![Span::styled("Description here", Style::default())],
        Style::default(),
    );
    let mut text = Text::from(Spans::from(description));
    text.patch_style(style);
    let msg = Paragraph::new(text).block(Block::default().title("DESCRIPTION").borders(Borders::ALL));
    f.render_widget(msg, rect);
}

fn render_memories<B: Backend>(f: &mut Frame<B>, rect: Rect, config: &model::Config, ui_state: &mut UiState) {
    let items_style = Style::default();
    let memories: Vec<ListItem> = config
        .memories
        .iter()
        .map(|m| {
            let content = vec![Spans::from(Span::raw(format!("MEMORY {}", m.id)))];
            ListItem::new(content).style(items_style)
        })
        .collect();
    ui_state
        .memory_state
        .select(Some(ui_state.memory.0.get(nr_memories(config))));
    let selected_style = items_style.add_modifier(Modifier::REVERSED);
    let memories = List::new(memories)
        .block(Block::default().borders(Borders::ALL).title("MEMORY SLOTS"))
        .highlight_style(selected_style);
    f.render_stateful_widget(memories, rect, &mut ui_state.memory_state);
}

fn render_menus<B: Backend>(f: &mut Frame<B>, rect: Rect, config: &model::Config, ui_state: &mut UiState) {
    let items_style = match ui_state.focus {
        Focus::Memory => Style::default().add_modifier(Modifier::DIM).fg(Color::DarkGray),
        _ => Style::default(),
    };
    let selected_memory = get_selected_memory(config, ui_state);
    let menus: Vec<ListItem> = selected_memory
        .menus
        .iter()
        .map(|m| {
            let content = vec![Spans::from(Span::raw(format!("{}", m.name)))];
            ListItem::new(content).style(items_style)
        })
        .collect();
    ui_state.menu_state.select(Some(ui_state.menu.0.get(nr_menus(config))));
    let selected_style = items_style.add_modifier(Modifier::REVERSED);
    let menus = List::new(menus)
        .block(Block::default().borders(Borders::ALL).title("MENUS"))
        .highlight_style(selected_style);
    f.render_stateful_widget(menus, rect, &mut ui_state.menu_state);
}

fn render_settings<B: Backend>(f: &mut Frame<B>, rect: Rect, config: &model::Config, ui_state: &mut UiState) {
    let items_style = match ui_state.focus {
        Focus::Memory | Focus::Menu => Style::default().add_modifier(Modifier::DIM).fg(Color::DarkGray),
        Focus::Setting | Focus::Edit => Style::default(),
    };
    let selected_menu = get_selected_menu(config, ui_state);
    let settings: Vec<ListItem> = selected_menu
        .settings
        .iter()
        .map(|m| {
            let content = vec![Spans::from(Span::raw(format!("{} = {}", m.key, m.value)))];
            ListItem::new(content).style(items_style)
        })
        .collect();
    ui_state
        .setting_state
        .select(Some(ui_state.setting.0.get(nr_settings(config, ui_state))));
    let selected_style = if ui_state.focus == Focus::Edit {
        items_style.fg(Color::Red)
    } else {
        items_style.add_modifier(Modifier::REVERSED)
    };
    let settings = List::new(settings)
        .block(Block::default().borders(Borders::ALL).title("SETTINGS"))
        .highlight_style(selected_style);
    f.render_stateful_widget(settings, rect, &mut ui_state.setting_state);
}
