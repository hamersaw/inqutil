use std::io::Error;

use writer::Writer;

use pb::proddle::{Probe, ProbeResult};

pub struct PostgresqlWriter {
}

impl PostgresqlWriter {
    pub fn open() -> Result<PostgresqlWriter, Error> {
        Ok(PostgresqlWriter {
        })
    }
}

impl Writer for PostgresqlWriter {
    fn write_probe(&self, probe: Probe) -> Result<(), String> {
        unimplemented!();
    }

    fn write_probe_result(&self, probe_result: ProbeResult) -> Result<(), String> {
        unimplemented!();
    }

    fn close(&self) -> Result<(), String> {
        unimplemented!();
    }
}
