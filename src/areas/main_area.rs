use std::cmp::min;
use tui::backend::{Backend};
use tui::Frame;
use tui::widgets::{Block, Borders, Table, Row};
use tui::layout::{Layout, Constraint, Rect};
use tui::style::{Color, Style};
use crate::{App, Player};

pub fn build<B>(f: &mut Frame<B>, chunk: Rect, app: &mut App) where B: Backend {
    let main_area_layout = Layout::default()
        .margin(1)
        .constraints([
            Constraint::Percentage(100),
        ].as_ref())
        .split(chunk);

    let main_area = Block::default().title("Checkers").borders(Borders::ALL);

    let min_size = min(chunk.width, chunk.height) - 2;

    f.render_widget(main_area, chunk);
    println!("We are in the yellow submarine!");
    let mut j = 0;
    let rows = app.items.iter().map(|item| {
        j += 1;
        let cells = item.iter().map(|c| {
            j += 1;
            c.value().style(Style::default().bg(if j % 2 == 0 {Color::Black} else {Color::White}).fg(if *c == Player::Red {Color::Red} else {Color::Blue}))
        });
        Row::new(cells).height(min_size / 8)
    });

    let widths: [Constraint; 8] = [Constraint::Length(min_size / 4); 8];

    let table = Table::new(rows)
        .widths(&widths)
        .column_spacing(1)
        .highlight_style(Style::default().bg(Color::Green))
        .highlight_symbol(">>");

    f.render_stateful_widget(table, main_area_layout[0], &mut app.state);
}