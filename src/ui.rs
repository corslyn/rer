use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

pub fn ui(frame: &mut Frame, app: &App, input_focused: bool) {
    let area = frame.area();
    let [header, timetable, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(5),
        Constraint::Length(2),
    ])
    .areas(area);

    let input_style = if input_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };
    let input = Paragraph::new(app.input.as_str()).style(input_style).block(
        Block::bordered()
            .title(" Code de la gare ")
            .borders(Borders::ALL),
    );
    frame.render_widget(input, header);

    let rows = app.departures.iter().map(|departure| {
        let line = if departure.line == "A" {
            Cell::from(Line::from(Span::styled(
                "A",
                Style::default().fg(Color::White).bg(Color::Red),
            )))
        } else if departure.line == "B" {
            Cell::from(Line::from(Span::styled(
                "B",
                Style::default().fg(Color::White).bg(Color::Blue),
            )))
        } else if departure.line == "C" {
            Cell::from(Line::from(Span::styled(
                "C",
                Style::default().fg(Color::White).bg(Color::Yellow),
            )))
        } else if departure.line == "D" {
            Cell::from(Line::from(Span::styled(
                "D",
                Style::default().fg(Color::White).bg(Color::Rgb(0, 100, 0)),
            )))
        } else if departure.line == "E" {
            Cell::from(Line::from(Span::styled(
                "E",
                Style::default().fg(Color::White).bg(Color::Magenta),
            )))
        } else {
            Cell::from(departure.line.clone())
        };
        Row::new([
            Cell::from(departure.eta.clone()),
            line,
            Cell::from(departure.train.clone()),
            Cell::from(departure.destination.clone()),
        ])
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(24),
            Constraint::Length(8),
            Constraint::Length(14),
            Constraint::Min(20),
        ],
    )
    .header(
        Row::new(["Heure estimée d'arrivée", "Ligne", "Train", "Direction"])
            .style(Style::default().fg(Color::Cyan).bold())
            .bottom_margin(1),
    )
    .block(
        Block::bordered()
            .title(format!(" {} ", app.station))
            .borders(Borders::ALL),
    )
    .row_highlight_style(Style::default().bg(Color::DarkGray));
    frame.render_stateful_widget(table, timetable, &mut TableState::default());

    let footer_text = if app.status.is_empty() {
        "Entree: rafraichir  Esc: quitter".to_owned()
    } else {
        format!("{}    Entree: rafraichir    Esc: quitter", app.status)
    };
    frame.render_widget(Paragraph::new(footer_text), footer);
}
