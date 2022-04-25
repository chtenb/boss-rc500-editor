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
use crate::writer;

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

enum Clipboard {
    Empty,
    CopiedMenu(model::Menu),
    CopiedMemory(model::Memory),
}

impl Default for Clipboard {
    fn default() -> Clipboard {
        Clipboard::Empty
    }
}

#[derive(Default)]
struct UiState {
    memory: MemoryIndex,
    menu: MenuIndex,
    setting: SettingIndex,
    focus: Focus,
    clipboard: Clipboard,
    message: Option<String>,

    memory_state: ListState,
    menu_state: ListState,
    setting_state: ListState,
}

fn clear_message(ui_state: &mut UiState) {
    ui_state.message = None
}

fn post_message(ui_state: &mut UiState, msg: &str) {
    ui_state.message = Some(msg.to_string());
}

fn nr_memories(config: &model::Config) -> usize {
    config.memories.len()
}

fn nr_menus(config: &model::Config) -> usize {
    config.memories[0].menus.len()
}

fn get_selected_memory<'a>(config: &'a model::Config, ui_state: &UiState) -> &'a model::Memory {
    &config.memories[ui_state.memory.0.get(nr_memories(config))]
}

fn get_selected_memory_mut<'a>(config: &'a mut model::Config, ui_state: &UiState) -> &'a mut model::Memory {
    let nr_memories = config.memories.len();
    &mut config.memories[ui_state.memory.0.get(nr_memories)]
}

fn get_selected_menu<'a>(config: &'a model::Config, ui_state: &UiState) -> &'a model::Menu {
    let selected_memory = get_selected_memory(config, ui_state);
    &selected_memory.menus[ui_state.menu.0.get(nr_menus(config))]
}

fn get_selected_menu_mut<'a>(config: &'a mut model::Config, ui_state: &UiState) -> &'a mut model::Menu {
    let selected_memory = get_selected_memory_mut(config, ui_state);
    let nr_menus = selected_memory.menus.len();
    &mut selected_memory.menus[ui_state.menu.0.get(nr_menus)]
}

fn get_selected_setting<'a>(menu: &'a model::UntypedMenu, ui_state: &UiState) -> &'a model::UntypedKeyValue {
    &menu.settings[ui_state.setting.0.get(menu.settings.len())]
}

fn get_selected_setting_mut<'a>(
    menu: &'a mut model::UntypedMenu,
    ui_state: &UiState,
) -> &'a mut model::UntypedKeyValue {
    let nr_settings = menu.settings.len();
    &mut menu.settings[ui_state.setting.0.get(nr_settings)]
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

fn save(config: &mut model::Config, ui_state: &mut UiState) -> Result<(), ()> {
    writer::write(&config.filename, config)
        .map_err(|e| post_message(ui_state, &format!("Error saving to file: {:?}", e)))
}

fn handle_input(config: &mut model::Config, ui_state: &mut UiState, key: KeyEvent) -> Result<(), ()> {
    clear_message(ui_state);
    match ui_state.focus {
        Focus::Memory => match key.code {
            KeyCode::Up | KeyCode::Char('k') => ui_state.memory.0.dec(nr_memories(config)),
            KeyCode::Down | KeyCode::Char('j') => ui_state.memory.0.inc(nr_memories(config)),
            KeyCode::Right | KeyCode::Enter | KeyCode::Char('l') => ui_state.focus = Focus::Menu,
            KeyCode::Char('y') => {
                let memory = get_selected_memory(config, ui_state);
                ui_state.clipboard = Clipboard::CopiedMemory(memory.clone());
                post_message(ui_state, "Copied memory to clipboard!");
            }
            KeyCode::Char('p') => match &ui_state.clipboard {
                Clipboard::Empty => {}
                Clipboard::CopiedMenu(_) => {}
                Clipboard::CopiedMemory(copied) => {
                    let nr_memories = config.memories.len();
                    config.memories[ui_state.memory.0.get(nr_memories)] = copied.clone();
                    post_message(ui_state, "Pasted memory in clipboard to selected memory!");
                }
            },
            KeyCode::Char('!') => return Err(()),
            KeyCode::Char('q') => match save(config, ui_state) {
                Ok(_) => return Err(()),
                Err(_) => {}
            },
            KeyCode::Char('s') => {
                let _ = save(config, ui_state);
            }
            _ => {}
        },
        Focus::Menu => match key.code {
            KeyCode::Up | KeyCode::Char('k') => ui_state.menu.0.dec(nr_menus(config)),
            KeyCode::Down | KeyCode::Char('j') => ui_state.menu.0.inc(nr_menus(config)),
            KeyCode::Left | KeyCode::Char('h') => ui_state.focus = Focus::Memory,
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter => {
                let menu = get_selected_menu(config, ui_state);
                match &menu.content {
                    model::MenuContent::StringValueMenu(_) => ui_state.focus = Focus::Edit,
                    model::MenuContent::KeyValueMenu(_) => ui_state.focus = Focus::Setting,
                }
            }
            KeyCode::Char('q') => return Err(()),
            _ => {}
        },
        Focus::Setting => {
            let menu = get_selected_menu(config, ui_state);
            match &menu.content {
                model::MenuContent::StringValueMenu(_) => {
                    // This is an invalid state, so move back
                    ui_state.focus = Focus::Memory;
                }
                model::MenuContent::KeyValueMenu(menu) => match key.code {
                    KeyCode::Up | KeyCode::Char('k') => ui_state.setting.0.dec(menu.settings.len()),
                    KeyCode::Down | KeyCode::Char('j') => ui_state.setting.0.inc(menu.settings.len()),
                    KeyCode::Left | KeyCode::Char('h') => ui_state.focus = Focus::Menu,
                    KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => ui_state.focus = Focus::Edit,
                    KeyCode::Char('q') => return Err(()),
                    _ => {}
                },
            }
        }
        Focus::Edit => {
            let menu = get_selected_menu_mut(config, ui_state);
            match &mut menu.content {
                model::MenuContent::KeyValueMenu(ref mut menu) => match key.code {
                    KeyCode::Up => {
                        let setting = get_selected_setting_mut(menu, ui_state);
                        let key: &str = &setting.key;
                        let upper_bound = model::BOUNDS.get(key);
                        match upper_bound {
                            None => setting.value += 1,
                            Some(bound) => setting.value = min(*bound, setting.value + 1),
                        }
                    }
                    KeyCode::Down => {
                        let setting = get_selected_setting_mut(menu, ui_state);
                        if setting.value > 0 {
                            setting.value -= 1;
                        }
                    }
                    KeyCode::Enter | KeyCode::Esc | KeyCode::Left | KeyCode::Char('h') => {
                        ui_state.focus = Focus::Setting
                    }
                    _ => {}
                },
                model::MenuContent::StringValueMenu(ref mut menu) => match key.code {
                    KeyCode::Backspace => {
                        let mut chars = menu.value.chars();
                        chars.next_back();
                        menu.value = chars.as_str().to_string();
                    }
                    KeyCode::Enter | KeyCode::Esc | KeyCode::Left => ui_state.focus = Focus::Menu,
                    KeyCode::Char(c) => {
                        let mut chars: Vec<char> = menu.value.chars().collect();
                        chars.push(c);
                        menu.value = String::from_iter(chars.into_iter().map(|c| (c as u8) as char));
                    }
                    _ => {}
                },
            }
        }
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
                Span::styled("q", Style::default().fg(Color::Red)),
                Span::raw(" to save and exit, "),
                Span::styled("!", Style::default().fg(Color::Red)),
                Span::raw(" to exit without saving, "),
                Span::styled("y", Style::default().fg(Color::Red)),
                Span::raw(" to copy a memory, "),
                Span::styled("p", Style::default().fg(Color::Red)),
                Span::raw(" to paste a memory"),
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
    // Render message if existing, otherwise render setting description
    match &ui_state.message {
        Some(msg) => {
            let (description, style) = (vec![Span::styled(msg, Style::default())], Style::default());
            let mut text = Text::from(Spans::from(description));
            text.patch_style(style);
            let msg = Paragraph::new(text).block(Block::default().title("MESSAGE").borders(Borders::ALL));
            f.render_widget(msg, rect);
        }
        None => {
            let selected_menu = get_selected_menu(config, ui_state);
            match &selected_menu.content {
                model::MenuContent::KeyValueMenu(selected_menu) => {
                    let selected_setting = get_selected_setting(selected_menu, ui_state);
                    let (description, style) = (
                        match model::DESCRIPTIONS.get(&selected_setting.key) {
                            Some(text) => vec![Span::styled(*text, Style::default())],
                            None => vec![Span::styled("-", Style::default())],
                        },
                        Style::default(),
                    );
                    let mut text = Text::from(Spans::from(description));
                    text.patch_style(style);
                    let msg = Paragraph::new(text).block(Block::default().title("DESCRIPTION").borders(Borders::ALL));
                    f.render_widget(msg, rect);
                }
                _ => {}
            }
        }
    }
}

fn render_memories<B: Backend>(f: &mut Frame<B>, rect: Rect, config: &model::Config, ui_state: &mut UiState) {
    let items_style = Style::default();
    let memories: Vec<ListItem> = config
        .memories
        .iter()
        .map(|m| {
            let name = model::get_memory_name(m);
            let content = vec![Spans::from(Span::raw(format!("{}: {}", m.id, name)))];
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
    match &selected_menu.content {
        model::MenuContent::KeyValueMenu(selected_menu) => {
            let settings: Vec<ListItem> = selected_menu
                .settings
                .iter()
                .map(|s| {
                    let key: &str = &s.key;
                    let value_str: &str = &format!("{}", s.value);
                    let display_key: &str = model::DISPLAY_KEYS.get(key).unwrap_or(&key);
                    let display_value: &str = model::DISPLAY_VALUES
                        .get(key)
                        .and_then(|values| values.get(s.value))
                        .unwrap_or(&value_str);
                    let content = vec![Spans::from(Span::raw(format!("{} = {}", display_key, display_value)))];
                    ListItem::new(content).style(items_style)
                })
                .collect();
            ui_state
                .setting_state
                .select(Some(ui_state.setting.0.get(selected_menu.settings.len())));
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
        model::MenuContent::StringValueMenu(selected_menu) => {
            let mut value = selected_menu.value.to_string();
            let mut style = items_style;
            if ui_state.focus == Focus::Edit {
                style = Style::default().fg(Color::Red);
                value = value + "_"
            }
            let text = Text::from(Spans::from(vec![Span::styled(value, style)]));
            let msg = Paragraph::new(text).block(Block::default().title("NAME").borders(Borders::ALL));
            f.render_widget(msg, rect);
        }
    }
}
