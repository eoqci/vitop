use sysinfo::System;

use crate::collector::memory::Memory;

pub struct App {
    pub should_quit: bool,

    // System for mornitoring
    pub sys: System,

    // Spec hardware
    pub cpu_usage: f32,
    pub ram_used: u64,
    pub ram_total: u64,
}

impl App {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        Self {
            should_quit: false,
            sys,
            cpu_usage: 0.0,
            ram_total: 0,
            ram_used: 0,
        }
    }

    pub fn update_data(&mut self) {
        // CPU spec update
        self.sys.refresh_cpu_usage();
        self.cpu_usage = self.sys.global_cpu_info().cpu_usage();

        // RAM spec update
        self.sys.refresh_memory();
        self.ram_used = self.sys.used_memory();
        self.ram_total = self.sys.total_memory();
    }

    pub fn memory(&self) -> Memory {
        Memory::from_bytes(self.ram_used, self.ram_total)
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
