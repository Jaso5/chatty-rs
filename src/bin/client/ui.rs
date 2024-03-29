pub mod choice;
mod message;

use std::fmt::Display;
use std::io;
use std::io::Stdout;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Style;
use tui::widgets::{Block, Borders};
use tui::{backend::CrosstermBackend, Frame, Terminal};
use tui_textarea::TextArea;

use crate::ui::message::draw_messages;
use crate::Choice;
use common::message::Message;

pub fn init() -> Terminal<CrosstermBackend<Stdout>> {
    let mut stdout = io::stdout();
    // Clear screen
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend).expect("Could not create terminal")
}

pub fn close(mut term: Terminal<CrosstermBackend<Stdout>>) {
    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    term.show_cursor().unwrap();
}

pub struct UIStorage<'a> {
    pub text_area: TextArea<'a>,
    pub choice: Choice<'a>,
}

impl<'a> UIStorage<'a> {
    pub fn new() -> Self {
        let mut text_area = TextArea::default();
        text_area.set_cursor_line_style(Style::default());

        let choice = Choice::new(
            "Hey, just wondering if you got your photos printed?",
            vec![
                "Wha..?".to_string(),
                "Bogos binted.".to_string(),
                "This joke isn't funny".to_string(),
            ],
        );

        Self { text_area, choice }
    }

    pub fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        message_list: &Vec<Message>,
        scroll: u16,
        state: bool,
    ) {
        let whole = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        let chat = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Min(5), Constraint::Length(2)].as_ref())
            .split(whole[1]);

        let channels = Block::default().title(" Channels ").borders(Borders::ALL);
        f.render_widget(channels, whole[0]);

        if self.choice.focused() {
            self.choice.draw(f, chat[0])
        } else {
            draw_messages(f, message_list, chat[0], scroll, state);
        }

        f.render_widget(self.text_area.widget(), chat[1]);
    }
}

pub fn crash<T: Display>(term: Option<Terminal<CrosstermBackend<Stdout>>>, error: T) {
    if let Some(term) = term {
        close(term)
    }
    println!("{error}")
}
