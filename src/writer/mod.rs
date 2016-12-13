pub mod sqlite3;

use pb::proddle::{Probe, ProbeResult};

use rusqlite::Error;

pub trait Writer {
    fn write_probe(&self, probe: Probe) -> Result<(), Error>;
    fn write_probe_result(&self, probe_result: ProbeResult) -> Result<(), Error>;
    fn close(self) -> Result<(), Error>;
}
