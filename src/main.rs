use std::io;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::{self, CrosstermBackend}, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::Span, widgets::{Block, Borders, ListState, Paragraph}, DefaultTerminal, Frame, Terminal};
use serde::Deserialize;

fn main() -> Result<()> {

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}



struct App {
    exit: bool,
}


impl App {

    const fn new() -> Self {
        Self {
            exit: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        terminal = Terminal::new(backend)?;

        while !self.exit{
            terminal.draw(Self::render)?;
            if event::poll(std::time::Duration::from_millis(100))?{
                match event::read()? {
                    Event::Key(key) => self.handle_keyboard(key),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn render(frame: &mut Frame) {
        let size = frame.area();
        let fg = Color::Rgb(157, 146, 170);
        let bg = Color::Rgb(44, 29, 58);

        let sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), 
            Constraint::Length(1),     
            Constraint::Percentage(80), 
        ])
        .split(size);

        // ----  Left Side   ----

        let left_width = (size.width * 20) / 100;
        //let left_area = Rect::new(0, 0, left_width, size.height);

        let left_title = Span::styled(
            " Left Block ",                      
            Style::default()
                .fg(fg)                   
                .bg(bg)                     
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),     
        );

        let left_block = Block::default().title(left_title).borders(Borders::ALL).style(Style::default().fg(fg).bg(bg));

        // ----  Right Side   ----

        let right_width = size.width - left_width;
        //let right_area = Rect::new(left_width, 0, right_width, size.height);

        let right_title = Span::styled(
            " Right Block ",                      
            Style::default()
                .fg(fg)                   
                .bg(bg)                     
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),     
        );

        let right_block = Block::default().title(right_title).borders(Borders::ALL).style(Style::default().fg(fg).bg(bg));

        // frame.render_widget(block, frame.area());

        // let paragraph = Paragraph::new("Hello World!")
        // .style(Style::new().fg(Color::Rgb(0, 0, 0)));

        frame.render_widget(left_block, sections[0]);
        frame.render_widget(right_block, sections[2]);
    }

    fn handle_keyboard(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }
}