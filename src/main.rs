use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, sync::mpsc, thread, time::Duration};
use sysinfo::System;

fn main() -> io::Result<()> {
    // 1. Setup Terminal (Chuyển terminal sang Raw Mode để tự quản lý input/output)
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // 2. Tạo Channel giao tiếp giữa Worker (Data) và UI
    let (tx, rx) = mpsc::channel();

    // 3. Worker Thread: Chuyên đi lấy dữ liệu hệ thống
    thread::spawn(move || {
        let mut sys = System::new_all();
        loop {
            // Cập nhật thông tin CPU
            sys.refresh_cpu_usage();
            let global_cpu = sys.global_cpu_info().cpu_usage();

            // Gửi dữ liệu qua channel. Nếu UI đã đóng (tx lỗi) thì thoát vòng lặp
            if tx.send(global_cpu).is_err() {
                break;
            }

            // Lấy mẫu mỗi giây một lần
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // 4. UI Thread: Vẽ giao diện và bắt sự kiện bàn phím
    let mut current_cpu = 0.0;
    loop {
        // Nhận dữ liệu mới từ Worker (nếu có) mà không làm nghẽn (non-blocking) luồng UI
        if let Ok(cpu) = rx.try_recv() {
            current_cpu = cpu;
        }

        // Render giao diện
        terminal.draw(|f| {
            let text = format!("\n  CPU Usage: {:.1}% \n\n  Bấm 'q' để thoát.", current_cpu);
            let widget = Paragraph::new(text)
                .block(Block::default().title(" Mini htop ").borders(Borders::ALL));

            f.render_widget(widget, f.size());
        })?;

        // Xử lý phím bấm (timeout 50ms để UI luôn mượt, không bị đơ chờ input)
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break; // Thoát vòng lặp chính
                }
            }
        }
    }

    // 5. Restore Terminal (Trả terminal về trạng thái bình thường sau khi thoát)
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
