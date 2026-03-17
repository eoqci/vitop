#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: String,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
}
