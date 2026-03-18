use crate::collector::{memory::Memory, process::ProcessInfo};
use ratatui::widgets::TableState;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

pub struct App {
    pub should_quit: bool,
    pub sys: System,
    pub cpu_usage: f32,
    pub ram_used: u64,
    pub ram_total: u64,
    pub processes: Vec<ProcessInfo>,
    pub table_state: TableState,
}

impl App {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::new().with_ram())
                .with_processes(ProcessRefreshKind::new().with_cpu().with_memory()),
        );
        let mut table_state = TableState::default();
        table_state.select(Some(0)); // select first row by default

        // Must refresh cpu 2 times to get acurate spec for CPU
        sys.refresh_cpu_usage();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_usage();

        Self {
            should_quit: false,
            sys,
            cpu_usage: 0.0,
            ram_total: 0,
            ram_used: 0,
            processes: Vec::new(),
            table_state,
        }
    }

    pub fn update_data(&mut self) {
        // CPU
        self.sys
            .refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
        self.cpu_usage = self.sys.global_cpu_info().cpu_usage();

        // RAM
        self.sys
            .refresh_memory_specifics(MemoryRefreshKind::new().with_ram());
        self.ram_used = self.sys.used_memory();
        self.ram_total = self.sys.total_memory();

        // Processes — only cpu + memory
        self.sys
            .refresh_processes_specifics(ProcessRefreshKind::new().with_cpu().with_memory());

        let mut procs: Vec<ProcessInfo> = self
            .sys
            .processes()
            .iter()
            .map(|(pid, p)| ProcessInfo {
                pid: pid.to_string(),
                name: p.name().to_string(),
                cpu: p.cpu_usage(),
                mem: p.memory(),
            })
            .collect();

        procs.sort_by(|a, b| {
            b.cpu
                .partial_cmp(&a.cpu)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        self.processes = procs;
    }

    pub fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.processes.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.processes.len().saturating_sub(1) {
                    self.processes.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
    pub fn memory(&self) -> Memory {
        Memory::from_bytes(self.ram_used, self.ram_total)
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
