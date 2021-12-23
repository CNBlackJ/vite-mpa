use tui::layout::Rect;
use tui::Frame;
use std::io;
use termion::raw::IntoRawMode;
use tui::style::{Color, Style, Modifier};
use tui::{Terminal, symbols};
use tui::backend::TermionBackend;
use tui::widgets::{ Block, Borders, Paragraph, Wrap, Dataset, GraphType, Chart, Axis};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::{Span, Spans};

use crate::app;

fn top<B: tui::backend::Backend>(f: &mut Frame<B>, app_ins: &app::App, area: Rect) {
    let text_top = vec![
        Spans::from(vec![
            Span::from("hostname: "),
            Span::from(app_ins.gas_info.hostname.to_string()),
            Span::raw(" "),
        ]),
        Spans::from(vec![
            Span::from("cpu num: "),
            Span::from(app_ins.gas_info.cpu_num.to_string()),
            Span::raw(" "),
        ]),
    ];
    let block_top = Block::default()
        .title("基本信息")
        .borders(Borders::ALL);
    let p = Paragraph::new(text_top).block(block_top).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn center<B: tui::backend::Backend>(f: &mut Frame<B>, app_ins: &app::App, area: Rect) {
    let text = vec![
        Spans::from(vec![
            Span::from("disk info:"),
        ]),
        Spans::from(vec![
            Span::from("total: "),
            Span::from(app_ins.gas_info.disk_info.total.to_string()),
        ]),
        Spans::from(vec![
            Span::from("free: "),
            Span::from(app_ins.gas_info.disk_info.free.to_string()),
        ]),
        Spans::from(vec![
            Span::from("memory info: "),
        ]),
        Spans::from(vec![
            Span::from("total: "),
            Span::from(app_ins.gas_info.mem_info.total.to_string()),
        ]),
        Spans::from(vec![
            Span::from("free: "),
            Span::from(app_ins.gas_info.mem_info.free.to_string()),
        ]),
        Spans::from(vec![
            Span::from("avail: "),
            Span::from(app_ins.gas_info.mem_info.avail.to_string()),
        ]),
    ];

    let block = Block::default()
        .title("电脑运行状况")
        .borders(Borders::ALL);
    let p = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn bottom<B: tui::backend::Backend>(f: &mut Frame<B>, app_ins: &app::App, area: Rect) {
    let datasets = vec![Dataset::default()
        .name("data")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&app_ins.signals.mem_signal.points)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Chart 3",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(tui::style::Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 50.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("25"),
                    Span::styled("50", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 5.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("2.5"),
                    Span::styled("5", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, area);
}

pub fn run(app_ins: &app::App) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(35),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .split(f.size());

        top(f, app_ins, chunks[0]);
        center(f, app_ins, chunks[1]);
        bottom(f, app_ins, chunks[2]);
    })?;
    Ok(())
}