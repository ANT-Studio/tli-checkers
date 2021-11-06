use tui::backend::{Backend};
use tui::Frame;
use tui::widgets::{Block, Borders};
use tui::layout::{Layout, Constraint, Rect};

pub fn build<B>(f: &mut Frame<B>, chunk: Rect) where B: Backend {
    let score_area_layout = Layout::default()
        .constraints([
            Constraint::Percentage(100),
        ].as_ref())
        .split(chunk);

    let score_area = Block::default().title("Score").borders(Borders::ALL);

    f.render_widget(score_area, chunk);
}