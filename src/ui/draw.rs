use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table},
};

use crate::app::app::App;

pub fn draw_ui(f: &mut Frame, app: &App) {
    //Vertical
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header = 3 lines
            Constraint::Min(0),    // Main = the rest place
            Constraint::Length(3), // Footer = 3 liens
        ])
        .split(f.size());

    // ================| Header |================
    let header_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // 50% on the left
            Constraint::Percentage(50), // 50% on the right
        ])
        .split(chunks[0]);

    // ram spec
    let mem = app.memory();

    // CPU draw
    let cpu_gauge = Gauge::default()
        .block(Block::default().title(" CPU ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(app.cpu_usage as u16)
        .label(format!("{:.1} %", app.cpu_usage));

    f.render_widget(cpu_gauge, header_chunks[0]); // put into the left side

    // RAM draw
    let ram_ratio = if app.ram_total > 0 {
        app.ram_used as f64 / app.ram_total as f64
    } else {
        0.0
    };

    let ram_gauge = Gauge::default()
        .block(Block::default().title(" RAM ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(ram_ratio)
        .label(format!(
            "{:.2} GB / {:.2} GB ({:.1}%)",
            mem.ram_used_gb, mem.ram_total_gb, mem.ram_used_percent
        ));

    f.render_widget(ram_gauge, header_chunks[1]); // put into the right side

    // ================| Main |================
    // let main_content = Paragraph::new(" Proccess running placehold").block(
    //     Block::default()
    //         .title(" Tiến trình (Processes) ")
    //         .borders(Borders::ALL),
    // );
    // f.render_widget(main_content, chunks[1]);
    let header_cells = ["PID", "Name", "CPU %", "RAM %"].iter().map(|h| {
        Cell::from(*h).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    });

    let table_header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1)
        .bottom_margin(1);

    //Data row
    let rows: Vec<Row> = app
        .processes
        .iter()
        .map(|p| {
            let mem_mb = p.mem as f64 / 1_048_567.0;

            Row::new(vec![
                Cell::from(p.pid.clone()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.1} %", p.cpu)),
                Cell::from(format!("{:.1} MB", mem_mb)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Min(20),
            Constraint::Length(10),
            Constraint::Length(15),
        ],
    )
    .header(table_header)
    .block(Block::default().title(" Processes ").borders(Borders::ALL));

    f.render_widget(table, chunks[1]);

    // Footer
    let footer = Paragraph::new(" [q] Thoát | [Up/Down] Cuộn chuột")
        .block(Block::default().title(" Hướng dẫn ").borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
