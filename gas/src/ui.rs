use std::io;
use termion::raw::IntoRawMode;
use tui::style::{Color, Style, Modifier};
use tui::{Terminal, symbols};
use tui::backend::TermionBackend;
use tui::widgets::{ Block, Borders, Paragraph, Wrap, Dataset, GraphType, Chart, Axis};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::{Span, Spans};

use crate::app;

const DATA2: [(f64, f64); 7] = [
    (0.0, 0.0),
    (10.0, 1.0),
    (20.0, 0.5),
    (30.0, 1.5),
    (40.0, 1.0),
    (50.0, 2.5),
    (60.0, 3.0),
];

const MEMDATA: [(u64, u64); 7] = [(0, 0); 7];

fn top(gas_info: app::GasInfo) -> Paragraph<'static> {
    let text_top = vec![
        Spans::from(vec![
            Span::from("hostname: "),
            Span::from(gas_info.hostname),
            Span::raw(" "),
        ]),
        Spans::from(vec![
            Span::from("cpu num: "),
            Span::from(gas_info.cpu_num.to_string()),
            Span::raw(" "),
        ]),
    ];
    let block_top = Block::default()
        .title("基本信息")
        .borders(Borders::ALL);
    Paragraph::new(text_top).block(block_top).wrap(Wrap { trim: true })
}

fn center(gas_info: app::GasInfo) -> Paragraph<'static> {
    let text = vec![
        Spans::from(vec![
            Span::from("disk info:"),
        ]),
        Spans::from(vec![
            Span::from("total: "),
            Span::from(gas_info.disk_info.total.to_string()),
        ]),
        Spans::from(vec![
            Span::from("free: "),
            Span::from(gas_info.disk_info.free.to_string()),
        ]),
        Spans::from(vec![
            Span::from("memory info: "),
        ]),
        Spans::from(vec![
            Span::from("total: "),
            Span::from(gas_info.mem_info.total.to_string()),
        ]),
        Spans::from(vec![
            Span::from("free: "),
            Span::from(gas_info.mem_info.free.to_string()),
        ]),
        Spans::from(vec![
            Span::from("avail: "),
            Span::from(gas_info.mem_info.avail.to_string()),
        ]),
    ];

    let block = Block::default()
        .title("电脑运行状况")
        .borders(Borders::ALL);
    Paragraph::new(text).block(block).wrap(Wrap { trim: true })
}

fn bottom(gas_info: app::GasInfo) -> Chart<'static> {

    let mem_free = gas_info.mem_info.free;

    MEMDATA

    let datasets = vec![Dataset::default()
        .name("data")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&DATA2)];
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
    chart
}

pub fn run() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let gas_info = app::show_info();
    // let gas_info2 = app::show_info();

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                ].as_ref()
            )
            .split(f.size());

        // f.render_widget(top(gas_info), chunks[0]);
        // f.render_widget(center(gas_info2), chunks[1]);
        f.render_widget(bottom(gas_info), chunks[2]);
    })?;
    Ok(())
}