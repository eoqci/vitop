<div align="center">
<pre>
██╗   ██╗██╗████████╗ ██████╗ ██████╗ 
██║   ██║██║╚══██╔══╝██╔═══██╗██╔══██╗
██║   ██║██║   ██║   ██║   ██║██████╔╝
╚██╗ ██╔╝██║   ██║   ██║   ██║██╔═══╝ 
 ╚████╔╝ ██║   ██║   ╚██████╔╝██║     
  ╚═══╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝     
</pre>
</div>

<div align="center">
 
[![Rust](https://img.shields.io/badge/built%20with-Rust-f74c00?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://www.microsoft.com/windows/windows)
[![License](https://img.shields.io/badge/license-MIT-22c55e?style=for-the-badge)](./LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-a78bfa?style=for-the-badge)](https://github.com/LQCpaka/mini_htop/releases)

**A super blazing-fast, lightweight terminal system monitor for Windows — inspired by `htop`.**
</div>
<br/>

> *Why open Task Manager when you can look this cool? Imagine Task Manager crashed with other apps, fck the windows, fck the shameless microsoft.*

<br/>

<img src="https://github.com/LQCpaka/vitop/blob/main/images/vitop-app.png" alt="vitop-images" />

</div>

---

##  Why Vitop?

Windows has Task Manager. Windows also has 47 background `svchost.exe` processes you'll never understand. **Vitop** cuts through the noise — a snappy, keyboard-driven TUI that lives in your terminal, monitors your machine in real time, and lets you nuke misbehaving processes without touching a mouse.

No Electron. No .NET. No bloat. Just Rust.


##  Features

<div align="center">
 
| Features |  |
|:---:|:---|
|  **Live Resource Gauges** | Color-coded CPU and Memory bars that update in real time |
|  **Process Table** | Sorted by memory usage, showing PID, Name, CPU%, and RAM |
|  **Fuzzy Search** | Instantly filter through hundreds of processes as you type |
|  **Safe Kill** | Terminate processes with a built-in confirmation prompt — no accidents |
|  **Non-blocking UI** | Separate threads for data collection and rendering; the UI never freezes |
|  **Memory Safe** | Zero undefined behavior, guaranteed by the Rust compiler |

</div>

##  Tech Stack

<div align="center">

| Crate | Role |
|:---:|:---|
| [`ratatui`](https://github.com/ratatui/ratatui) | Immediate-mode TUI rendering |
| [`crossterm`](https://github.com/crossterm-rs/crossterm) | Terminal manipulation & keyboard events |
| [`sysinfo`](https://github.com/GuillaumeGomez/sysinfo) | System hardware & process data |

</div>


##  Keybindings

<div align="center">

| Key | Action |
|:---:|:---|
| `↑` `↓` | Navigate the process list |
| `/` | Enter search / filter mode |
| `Enter` `Esc` | Exit search / clear query |
| `k` | Kill highlighted process |
| `y` / `n` | Confirm / cancel termination |
| `q` | Quit gracefully |

</div>


##  Installation

> **Prerequisite:** [Rust & Cargo](https://rustup.rs/) must be installed.

You could download it from Release tab. And install it.

Only things you need to do is run this cmd on cmd/pwsh/run, whatever allows to excute binary path. It's very simple:

```bash
vitop
```

---



##  Roadmap

- [ ] Network I/O monitoring (upload / download speeds)
- [ ] Disk usage per drive
- [ ] Sortable columns (click or key to sort by CPU, RAM, PID, Name)
- [ ] Process tree view
- [ ] Config file for custom colors / refresh rate
- [ ] Package via `winget` / `scoop`

---

##  License

Released under the [MIT License](./LICENSE). Free to use, modify, and distribute.

---

<div align="center">

Made with 🦀 and a slight hatred for Task Manager.

**[⬆ Back to top](#)**

</div>
