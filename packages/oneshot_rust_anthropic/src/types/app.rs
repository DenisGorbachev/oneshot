use ratatui::layout::{Constraint, Direction};
use ratatui::prelude::{Modifier, Style};
use ratatui::Frame;
use throbber_widgets_tui::{Throbber, ThrobberState, WhichUse, ASCII};

#[allow(dead_code)]
pub struct App {
    throbber_state: ThrobberState,
}

#[allow(dead_code)]
impl App {
    fn on_tick(&mut self) {
        self.throbber_state.calc_next();
    }
}

#[allow(dead_code)]
fn ui(f: &mut Frame, app: &mut App) {
    let chunks = ratatui::layout::Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Simple random step
    let simple = Throbber::default();
    f.render_widget(simple, chunks[0]);

    // Set full with state
    let full = Throbber::default()
        .label("Running...")
        .style(Style::default().fg(ratatui::style::Color::Cyan))
        .throbber_style(
            Style::default()
                .fg(ratatui::style::Color::Red)
                .add_modifier(Modifier::BOLD),
        )
        .throbber_set(ASCII)
        .use_type(WhichUse::Spin);
    f.render_stateful_widget(full, chunks[1], &mut app.throbber_state);
}
