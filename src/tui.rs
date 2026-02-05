use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::Frame;
use ratatui::{
    buffer::Buffer,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListDirection, ListItem, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    cursor_position: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.cursor_up(),
            KeyCode::Down => self.cursor_down(),
            _ => {}
        }
    }

    fn cursor_up(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }
    fn cursor_down(&mut self) {
        if self.cursor_position < 5 {
            self.cursor_position += 1;
        } else {
            todo!()
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("View Generator".bold());
        let instructions = Line::from(vec![
            " Move up ".into(),
            "<UP>".blue().bold(),
            " Move down ".into(),
            "<DOWN>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
            self.cursor_position.to_string().green(),
            " ".into(),
        ]);

        let list_items = ["wow"];
        let list = List::new(list_items)
            .block(Block::bordered().title("Components"))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">")
            .render(area, buf);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        let paragraph = Paragraph::default()
            .alignment(ratatui::layout::HorizontalAlignment::Center)
            .block(block)
            .render(area, buf);
    }
}
