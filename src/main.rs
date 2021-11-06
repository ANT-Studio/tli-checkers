use std::io;
use std::io::stdin;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::{IntoRawMode};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{TableState, Cell};
use termion::{
    screen::{
        AlternateScreen
    },
    event::{
        Key
    }
};
use termion::event::Event;
use termion::input::TermRead;

mod areas {
    pub mod main_area;
    pub mod score_area;
}

#[derive(PartialEq)]
pub enum Player {
    Red,
    Blue,
    None
}

impl Player {
    pub fn value(&self) -> Cell {
       match self {
           Player::Red => Cell::from(Text::from("R")),
           Player::Blue => Cell::from(Text::from("B")),
           Player::None => Cell::from(Text::from(""))
       }
    }
}

pub struct App {
    state: TableState,
    items: Vec<Vec<Player>>,
}

impl App {
    fn new() -> App {
        let mut checkerboard: Vec<Vec<Player>> = vec![];

        for x in 0..8 {
            checkerboard.push(vec![]);

            for y in 0..8 {
                if (x % 2 == 0 && y % 2 == 0) || (x % 2 == 1 && y % 2 == 1) {
                    if x <= 1 {
                        checkerboard[x].push(Player::Red);
                    }
                    else if x <= 5 {
                        checkerboard[x].push(Player::None);
                    }
                    else {
                        checkerboard[x].push(Player::Blue);
                    }
                }
                else {
                    checkerboard[x].push(Player::None);
                }
            }
        }

        App {
            state: TableState::default(),
            items: checkerboard,
        }
    }

    pub fn set_new(&mut self) {
       self.items[3][4] = Player::Red;
    }
}

fn main() -> Result<(), io::Error> {
    let mut app = App::new();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    terminal.show_cursor()?;

    loop {
        terminal.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(60),
                    Constraint::Percentage(40)
                ].as_ref())
                .split(f.size());

            areas::main_area::build(f, layout[0], &mut app);
            areas::score_area::build(f, layout[1]);
        })?;

       for c in stdin().keys() {
           match c.unwrap() {
               Key::Char('p') => {
                   println!("Clicked p!");
                   app.set_new();
               },
               Key::Char('q') => return Ok(()),
               _ => {}
           }
       }
    }
}
