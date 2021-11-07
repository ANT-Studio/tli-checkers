use std::collections::HashMap;
use std::io;
use std::io::stdin;
use tui::Terminal;
use tui::backend::{CrosstermBackend};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{TableState, Cell};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

mod areas {
    pub mod main_area;
    pub mod score_area;
}

#[derive(Copy, Clone)]
struct Position {
    x: i8,
    y: i8
}

#[derive(PartialEq, Eq, Hash)]
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
    positions: HashMap<Player, Vec<Position>>,
    current_player: Player,
    current_index: usize,
}

impl App {
    fn new() -> App {
        let mut checkerboard: Vec<Vec<Player>> = vec![];
        let mut red_positions: Vec<Position> = vec![];
        let mut blue_positions: Vec<Position> = vec![];

        for x in 0..8 {
            checkerboard.push(vec![]);

            for y in 0..8 {
                if (x % 2 == 0 && y % 2 == 0) || (x % 2 == 1 && y % 2 == 1) {
                    let xi = x as i8;
                    if x <= 1 {
                        checkerboard[x].push(Player::Red);
                        red_positions.push(Position{ x: xi, y });
                    }
                    else if x <= 5 {
                        checkerboard[x].push(Player::None);
                    }
                    else {
                        checkerboard[x].push(Player::Blue);
                        blue_positions.push(Position{ x:xi, y});
                    }
                }
                else {
                    checkerboard[x].push(Player::None);
                }
            }
        }

        let mut map = HashMap::new();
        map.insert(Player::Red, red_positions);
        map.insert(Player::Blue, blue_positions);

        App {
            state: TableState::default(),
            items: checkerboard,
            positions: map,
            current_player: Player::Red,
            current_index: 0,
        }
    }

    fn get_current_list(&self) -> &Vec<Position>{
        self.positions.get(&self.current_player).unwrap()
    }

}

fn main() -> Result<(), io::Error> {
    let mut app = App::new();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear();

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

       if let Event::Key(key) = event::read()? {
           match key.code {
               KeyCode::Char('p') => {
                    app.current_index = (app.current_index + 1) % app.get_current_list().len();
               },
               KeyCode::Char('q') => {
                   terminal.clear();
                   disable_raw_mode()?;
                   return Ok(())
               },
               KeyCode::Enter => {
                   if app.current_player == Player::Red {app.current_player = Player::Blue} else {app.current_player = Player::Red}
                   app.current_index = 0;
               }
               _ => {}
           }
       }
    }
}
