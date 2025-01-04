use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, layout::Rect, style::{Color, Style}, widgets::{Block, Borders, Paragraph}, DefaultTerminal, Frame};

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
        let bg = Style::new().bg(Color::Rgb(94, 177, 191));

        // ----  Left Side   ----

        let left_width = (size.width * 30) / 100;
        let left_area = Rect::new(0, 0, left_width, size.height);

        let left_block = Block::default().title("left block").borders(Borders::ALL).style(Style::default().fg(Color::Yellow).bg(Color::Black));

        // ----  Right Side   ----

        let right_width = size.width - left_width;
        let right_area = Rect::new(left_width, 0, right_width, size.height);

        let right_block = Block::default().title("right block").borders(Borders::ALL).style(Style::default().fg(Color::Green).bg(Color::Black));

        // frame.render_widget(block, frame.area());

        // let paragraph = Paragraph::new("Hello World!")
        // .style(Style::new().fg(Color::Rgb(0, 0, 0)));

        frame.render_widget(left_block, left_area);
        frame.render_widget(right_block, right_area);
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