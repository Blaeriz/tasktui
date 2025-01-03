use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, style::{Color, Style}, widgets::{Block, Borders, Paragraph}, DefaultTerminal, Frame};

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
        let bg = Style::new().bg(Color::Rgb(94, 177, 191));
        let block = Block::default()
        .title("Blue Canvas")
        .borders(Borders::ALL)
        .style(bg);

        frame.render_widget(block, frame.area());

        let paragraph = Paragraph::new("Hello World!")
        .style(Style::new().fg(Color::Rgb(0, 0, 0)));

        frame.render_widget(paragraph, frame.area());
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