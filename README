# 🚀 Vitop (Mini htop for Windows)

A blazing-fast, lightweight, and interactive terminal-based system monitor specifically optimized for Windows 11. Written entirely in **Rust**.

Inspired by Linux's legendary `htop`, this tool brings a clean, multi-threaded Terminal User Interface (TUI) to your Windows command line, allowing you to monitor system resources and manage processes without the overhead of the heavy GUI Task Manager.

## ✨ Features

* **Real-time Resource Monitoring:** Live CPU and Memory usage visualized with responsive, color-coded gauges.
* **Interactive Process Table:** View running processes sorted by memory consumption, detailing PID, Name, CPU%, and RAM usage.
* **Fuzzy Search & Filtering:** Instantly find specific applications hidden in hundreds of background processes using the built-in search bar.
* **Safe Process Termination:** Kill unresponsive processes directly from the terminal with a built-in safety confirmation popup.
* **Highly Performant:** Built with a clean architecture separating the data collection thread from the UI rendering thread, ensuring zero UI blocking and minimal footprint.
* **Memory Safe:** Powered by Rust's strict memory management guarantees.

## 🛠️ Tech Stack

* **[Rust](https://www.rust-lang.org/):** The core language.
* **[Ratatui](https://github.com/ratatui/ratatui):** For rendering the rich, immediate-mode Terminal User Interface.
* **[Crossterm](https://github.com/crossterm-rs/crossterm):** For cross-platform terminal manipulation and event handling.
* **[Sysinfo](https://github.com/GuillaumeGomez/sysinfo):** For low-level system hardware and process information retrieval.

## ⌨️ Keybindings

Navigation and control are designed to be intuitive and keyboard-centric:

| Key | Action |
| :--- | :--- |
| `Up` / `Down` | Navigate through the process list |
| `/` | Enter Search/Filter mode |
| `Enter` / `Esc` | Exit Search mode / Clear search query |
| `k` | Attempt to Kill the highlighted process |
| `y` / `n` | Confirm (`y`) or Cancel (`n`) process termination |
| `q` | Quit the application gracefully |

## 🚀 Installation & Build

Ensure you have [Rust and Cargo](https://rustup.rs/) installed on your Windows machine.

1. Clone the repository:
   ```bash
   git clone [https://github.com/LQCpaka/mini_htop.git](https://github.com/LQCpaka/mini_htop.git)
   cd vitop
