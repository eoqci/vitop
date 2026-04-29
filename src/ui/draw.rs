use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{
    app::app::App,
    ui::widgets::{proc_table::draw_table, sys_info::draw_header},
};

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    //Vertical
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header = 4 lines
            Constraint::Min(0),    // Main = the rest place
            Constraint::Length(3), // Footer = 3 lines
        ])
        .split(f.size());

    // ================| HEADER |================
    draw_header(f, app, chunks[0]);

    // ================| MAIN |==================
    draw_table(f, app, chunks[1]);

    // ================| FOOTER |================
    let footer_text = if app.is_searching {
        format!("Type App name: {}_ | [Enter]/[Esc] Done", app.search_query)
    } else if !app.search_query.is_empty() {
        format!(
            " Filtering: '{}' | [/] Searching | [k] Kill Task | [Esc] Remove Filter | [q] Quit",
            app.search_query
        )
    } else {
        " [/] Searching | [k] Kill Task | [q] Quit | [Up/Down] Scroll Mouse ".to_string()
    };

    let footer_block = Block::default()
        .title(if app.is_searching {
            " SEARCHING "
        } else {
            " HELP "
        })
        .borders(Borders::ALL)
        .border_style(if app.is_searching {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let footer = Paragraph::new(footer_text).block(footer_block);

    f.render_widget(footer, chunks[2]);

    // ================| Popup Kill Process |================
    if app.show_kill_popup {
        let target_name = app.target_name.as_deref().unwrap_or("Unknown");
        let target_pid = app.target_pid.as_deref().unwrap_or("0");

        let popup_text = format!(
            "\n Are you sure that you want kill this task? \n\n Name: {} \n PID: {} \n\n [Y] Yes   /   [N] No ",
            target_name, target_pid
        );

        let popup = Paragraph::new(popup_text)
            .style(Style::default().bg(Color::Red).fg(Color::White)) // Nền đỏ chữ trắng cho cảnh báo
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .title(" Warning ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            );

        // Tạo một cái khung nhỏ ở chính giữa màn hình (rộng 60%, cao 30%)
        let area = centered_rect(60, 30, f.size());

        // 1. Dùng Clear để xóa sạch cái bảng Process nằm dưới cái khung này
        f.render_widget(Clear, area);
        // 2. Vẽ cái Popup đè lên
        f.render_widget(popup, area);
    }
}

// Helper function: This function calculates how to create a rectangular cell centered on the screen.
fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
