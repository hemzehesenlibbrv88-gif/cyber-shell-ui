use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};
use chrono::Local;

#[derive(Clone, Copy, PartialEq)]
enum AppState {
    LockScreen,
    Unlocking(f32), // Animation progress 0.0 to 1.0
    Terminal,
}

struct App {
    state: AppState,
    input: String,
    output: Vec<String>,
    command_history: Vec<String>,
    history_index: usize,
    cursor_position: usize,
    last_animation_time: Instant,
}

impl App {
    fn new() -> Self {
        App {
            state: AppState::LockScreen,
            input: String::new(),
            output: vec![
                "Welcome to Cyber Shell UI v0.1.0".to_string(),
                "Type 'help' for available commands".to_string(),
                "Type 'exit' to quit".to_string(),
                "".to_string(),
            ],
            command_history: Vec::new(),
            history_index: 0,
            cursor_position: 0,
            last_animation_time: Instant::now(),
        }
    }

    fn update_animation(&mut self) {
        if let AppState::Unlocking(progress) = self.state {
            let elapsed = self.last_animation_time.elapsed();
            let new_progress = progress + (elapsed.as_secs_f32() / 0.5); // 0.5 second animation

            if new_progress >= 1.0 {
                self.state = AppState::Terminal;
            } else {
                self.state = AppState::Unlocking(new_progress);
            }
            self.last_animation_time = Instant::now();
        }
    }

    fn start_unlock(&mut self) {
        if self.state == AppState::LockScreen {
            self.state = AppState::Unlocking(0.0);
            self.last_animation_time = Instant::now();
        }
    }

    fn process_command(&mut self, cmd: &str) {
        let trimmed = cmd.trim().to_lowercase();
        
        self.output.push(format!("$ {}", cmd));
        
        match trimmed.as_str() {
            "help" => {
                self.output.push("".to_string());
                self.output.push("Available Commands:".to_string());
                self.output.push("  help          - Show this help message".to_string());
                self.output.push("  clear         - Clear the terminal".to_string());
                self.output.push("  ls            - List files (demo)".to_string());
                self.output.push("  pwd           - Print working directory".to_string());
                self.output.push("  echo <text>   - Echo text".to_string());
                self.output.push("  date          - Show current date/time".to_string());
                self.output.push("  exit          - Exit the shell".to_string());
                self.output.push("".to_string());
            },
            "clear" => {
                self.output.clear();
            },
            "ls" => {
                self.output.push("".to_string());
                self.output.push("  Documents/    Projects/     Downloads/".to_string());
                self.output.push("  Desktop/      Music/        Pictures/".to_string());
                self.output.push("".to_string());
            },
            "pwd" => {
                self.output.push("/home/user/cyber-shell-ui".to_string());
                self.output.push("".to_string());
            },
            "date" => {
                let now = Local::now();
                self.output.push(now.format("%Y-%m-%d %H:%M:%S").to_string());
                self.output.push("".to_string());
            },
            cmd if cmd.starts_with("echo ") => {
                let text = &cmd[5..];
                self.output.push(text.to_string());
                self.output.push("".to_string());
            },
            "" => {
                self.output.push("".to_string());
            },
            _ => {
                self.output.push(format!("cyber-shell-ui: command not found: '{}'", trimmed));
                self.output.push("".to_string());
            }
        }
        
        self.command_history.push(cmd.to_string());
        self.history_index = self.command_history.len();
    }

    fn handle_input(&mut self, key: KeyCode) {
        match self.state {
            AppState::LockScreen => {
                // Any key press to unlock
                self.start_unlock();
            },
            AppState::Terminal => {
                match key {
                    KeyCode::Char(c) => {
                        self.input.insert(self.cursor_position, c);
                        self.cursor_position += 1;
                    },
                    KeyCode::Backspace => {
                        if self.cursor_position > 0 {
                            self.input.remove(self.cursor_position - 1);
                            self.cursor_position -= 1;
                        }
                    },
                    KeyCode::Delete => {
                        if self.cursor_position < self.input.len() {
                            self.input.remove(self.cursor_position);
                        }
                    },
                    KeyCode::Left => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                        }
                    },
                    KeyCode::Right => {
                        if self.cursor_position < self.input.len() {
                            self.cursor_position += 1;
                        }
                    },
                    KeyCode::Home => {
                        self.cursor_position = 0;
                    },
                    KeyCode::End => {
                        self.cursor_position = self.input.len();
                    },
                    KeyCode::Enter => {
                        self.process_command(&self.input);
                        self.input.clear();
                        self.cursor_position = 0;
                    },
                    KeyCode::Up => {
                        if self.history_index > 0 {
                            self.history_index -= 1;
                            self.input = self.command_history[self.history_index].clone();
                            self.cursor_position = self.input.len();
                        }
                    },
                    KeyCode::Down => {
                        if self.history_index < self.command_history.len() - 1 {
                            self.history_index += 1;
                            self.input = self.command_history[self.history_index].clone();
                            self.cursor_position = self.input.len();
                        } else {
                            self.history_index = self.command_history.len();
                            self.input.clear();
                            self.cursor_position = 0;
                        }
                    },
                    _ => {}
                }
            },
            AppState::Unlocking(_) => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Main loop
    loop {
        terminal.draw(|f| ui(f, &app))?;

        app.update_animation();

        if crossterm::event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('c') if key.modifiers == crossterm::event::KeyModifiers::CONTROL => {
                        if app.state == AppState::Terminal {
                            break;
                        }
                    },
                    _ => app.handle_input(key.code),
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    match app.state {
        AppState::LockScreen => draw_lock_screen(f, app),
        AppState::Unlocking(progress) => draw_unlock_animation(f, app, progress),
        AppState::Terminal => draw_terminal(f, app),
    }
}

fn draw_lock_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    
    // Background
    let bg = Paragraph::new("")
        .style(Style::default().bg(Color::Black));
    f.render_widget(bg, size);

    // Clock in center
    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();
    let date_str = now.format("%A, %B %d, %Y").to_string();

    let time_spans = vec![
        Span::styled(
            time_str,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        ),
    ];

    let date_spans = vec![
        Span::styled(
            date_str,
            Style::default()
                .fg(Color::Green)
        ),
    ];

    let hint_spans = vec![
        Span::styled(
            "Press any key to unlock...",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::DIM)
        ),
    ];

    // Center vertically and horizontally
    let time_width = 20;
    let time_height = 1;
    let center_col = (size.width.saturating_sub(time_width)) / 2;
    let center_row = size.height / 2;

    let time_area = Rect {
        x: center_col,
        y: center_row.saturating_sub(2),
        width: time_width,
        height: time_height,
    };

    let date_area = Rect {
        x: center_col.saturating_sub(5),
        y: center_row,
        width: 30,
        height: time_height,
    };

    let hint_area = Rect {
        x: center_col.saturating_sub(10),
        y: center_row.saturating_add(3),
        width: 40,
        height: time_height,
    };

    let time_widget = Paragraph::new(Line::from(time_spans))
        .alignment(Alignment::Center);
    
    let date_widget = Paragraph::new(Line::from(date_spans))
        .alignment(Alignment::Center);

    let hint_widget = Paragraph::new(Line::from(hint_spans))
        .alignment(Alignment::Center);

    f.render_widget(time_widget, time_area);
    f.render_widget(date_widget, date_area);
    f.render_widget(hint_widget, hint_area);
}

fn draw_unlock_animation<B: Backend>(f: &mut Frame<B>, app: &App, progress: f32) {
    let size = f.size();
    
    // Background
    let bg = Paragraph::new("")
        .style(Style::default().bg(Color::Black));
    f.render_widget(bg, size);

    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();

    // Animate upward and fade
    let y_offset = ((1.0 - progress) * (size.height as f32)) as u16;
    let opacity = (255.0 * (1.0 - progress)) as u8;

    let time_area = Rect {
        x: size.width / 2 - 10,
        y: size.height / 2 + y_offset,
        width: 20,
        height: 1,
    };

    let time_spans = vec![
        Span::styled(
            time_str,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        ),
    ];

    let time_widget = Paragraph::new(Line::from(time_spans))
        .alignment(Alignment::Center);

    f.render_widget(time_widget, time_area);

    // Terminal background starts appearing
    let terminal_opacity = progress;
    if terminal_opacity > 0.0 {
        draw_terminal(f, app);
    }
}

fn draw_terminal<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(3),
        ].as_ref())
        .split(f.size());

    // Output area
    let output_text: Vec<Line> = app.output.iter()
        .map(|line| Line::from(Span::styled(
            line.clone(),
            Style::default().fg(Color::Green)
        )))
        .collect();

    let output = Paragraph::new(output_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Cyber Shell UI ")
            .title_alignment(Alignment::Left))
        .style(Style::default().fg(Color::Green).bg(Color::Black));

    f.render_widget(output, chunks[0]);

    // Input area
    let input_spans: Vec<Span> = if app.cursor_position < app.input.len() {
        let before = &app.input[..app.cursor_position];
        let cursor_char = app.input.chars().nth(app.cursor_position).unwrap_or(' ');
        let after = &app.input[app.cursor_position + 1..];
        
        vec![
            Span::styled("$ ", Style::default().fg(Color::Yellow)),
            Span::raw(before),
            Span::styled(cursor_char.to_string(), Style::default().fg(Color::Black).bg(Color::Green)),
            Span::raw(after),
        ]
    } else {
        vec![
            Span::styled("$ ", Style::default().fg(Color::Yellow)),
            Span::raw(&app.input),
            Span::styled("_", Style::default().fg(Color::Black).bg(Color::Green)),
        ]
    };

    let input = Paragraph::new(Line::from(input_spans))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Input ")
            .title_alignment(Alignment::Left))
        .style(Style::default().fg(Color::Green).bg(Color::Black));

    f.render_widget(input, chunks[1]);
}
