use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{ Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::{Span, Spans};

use crate::app;

// fn show() {
//     let gas_info = app::show_info();

//     println!("hostname: {}", gas_info.hostname);
//     println!("cpu num: {}", gas_info.cpu_num);
//     println!("total: {}; \nfree: {}", gas_info.disk_info.total, gas_info.disk_info.free);
//     println!("****memory info****");
//     println!("total: {}\nfree: {}\navail: {}", gas_info.mem_info.total, gas_info.mem_info.free, gas_info.mem_info.avail);
//     println!("****memory info****");
// }

pub fn run() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let gas_info = app::show_info();

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());

            let text = vec![
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
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[1]);
    })?;
    Ok(())
}