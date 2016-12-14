use writer::Writer;
use pb::proddle::{Probe, ProbeResult};

pub struct PrintWriter {
}

impl PrintWriter {
    pub fn new() -> PrintWriter {
        PrintWriter {
        }
    }
}

impl Writer for PrintWriter {
    fn init(&self) -> Result<(), String> {
        unimplemented!();
    }

    fn write_probe(&self, probe: Probe) -> Result<(), String> {
        println!("probe: {:?}", probe);
        Ok(())
    }

    fn write_probe_result(&self, probe_result: ProbeResult) -> Result<(), String> {
        println!("probe_result: {:?}", probe_result);
        Ok(())
    }

    fn close(&self) -> Result<(), String> {
        Ok(())
    }
}
