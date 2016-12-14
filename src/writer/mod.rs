pub mod postgresql;
pub mod print;
pub mod sqlite3;

use pb::proddle::{Probe, ProbeResult};

pub trait Writer {
    fn init(&self) -> Result<(), String>;
    fn write_probe(&self, probe: Probe) -> Result<(), String>;
    fn write_probe_result(&self, probe_result: ProbeResult) -> Result<(), String>;
    fn close(&self) -> Result<(), String>;
}
