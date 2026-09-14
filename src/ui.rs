use crate::app::App;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

pub fn ui(frame: &mut Frame, app: &App, input_focused: bool) {
    let area = frame.area();
    let [header, suggestions, timetable, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(7),
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
            .title(" Nom de la gare ")
            .borders(Borders::ALL),
    );
    frame.render_widget(input, header);

    let suggestion_lines = app
        .suggestions()
        .into_iter()
        .enumerate()
        .map(|(index, station)| {
            let style = if app.suggestion_is_selected(index) {
                Style::default().fg(Color::Black).bg(Color::Yellow)
            } else {
                Style::default()
            };
            Line::from(format!("  {} ", station.name)).style(style)
        })
        .collect::<Vec<_>>();
    frame.render_widget(Paragraph::new(suggestion_lines), suggestions);

    let rows = app.departures.iter().map(|departure| {
        let line = if departure.line == "A" {
            Cell::from(Line::from(Span::styled(
                "A",
                Style::default().bg(Color::Red),
            )))
        } else if departure.line == "B" {
            Cell::from(Line::from(Span::styled(
                "B",
                Style::default().bg(Color::Blue),
            )))
        } else if departure.line == "C" {
            Cell::from(Line::from(Span::styled(
                "C",
                Style::default().bg(Color::Yellow),
            )))
        } else if departure.line == "D" {
            Cell::from(Line::from(Span::styled(
                "D",
                Style::default().bg(Color::Rgb(0, 100, 0)),
            )))
        } else if departure.line == "E" {
            Cell::from(Line::from(Span::styled(
                "E",
                Style::default().bg(Color::Magenta),
            )))
        } else if departure.line == "H" {
            Cell::from(Line::from(Span::styled(
                "H",
                Style::default().bg(Color::Rgb(165, 42, 42)),
            )))
        } else if departure.line == "J" {
            Cell::from(Line::from(Span::styled(
                "J",
                Style::default().bg(Color::LightGreen),
            )))
        } else if departure.line == "K" {
            Cell::from(Line::from(Span::styled(
                "K",
                Style::default().bg(Color::Rgb(122, 118, 61)),
            )))
        } else if departure.line == "L" {
            Cell::from(Line::from(Span::styled(
                "L",
                Style::default().bg(Color::Rgb(230, 230, 250)),
            )))
        } else if departure.line == "N" {
            Cell::from(Line::from(Span::styled(
                "N",
                Style::default().bg(Color::Cyan),
            )))
        } else if departure.line == "P" {
            Cell::from(Line::from(Span::styled(
                "P",
                Style::default().bg(Color::Rgb(255, 165, 0)),
            )))
        } else if departure.line == "R" {
            Cell::from(Line::from(Span::styled(
                "R",
                Style::default().bg(Color::Rgb(255, 192, 203)),
            )))
        } else if departure.line == "U" {
            Cell::from(Line::from(Span::styled(
                "U",
                Style::default().bg(Color::Rgb(220, 20, 60)),
            )))
        } else if departure.line == "V" {
            Cell::from(Line::from(Span::styled(
                "V",
                Style::default().bg(Color::Rgb(122, 118, 61)),
            )))
        } else {
            Cell::from(departure.line.clone())
        };
        Row::new([
            Cell::from(if departure.eta == 0 {
                "en approche / à quai".to_string()
            } else {
                format!("{} min", departure.eta)
            }),
            line,
            Cell::from(departure.train.clone()),
            Cell::from(departure.destination.clone()),
            Cell::from(departure.platform.clone()),
        ])
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(24),
            Constraint::Length(8),
            Constraint::Length(14),
            Constraint::Min(20),
            Constraint::Length(8),
        ],
    )
    .header(
        Row::new(["ETA", "Ligne", "Train", "Direction", "Voie"])
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
