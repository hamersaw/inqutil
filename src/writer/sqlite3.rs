use std::path::Path;

use pb::proddle::{Probe, ProbeResult};
use writer::Writer;

use rusqlite::Error;
use rusqlite::Connection;

const CREATE_HTTP_PROBES_TABLE_STMT: &'static str = "
CREATE TABLE http_probes (
    probe_id                            INT NOT NULL,
    probe_interval_seconds              INT NOT NULL,
    timeout_seconds                     INT NOT NULL,
    attempts_to_declare_failure         INT NULL,
    domain                              TEXT NOT NULL,
    port                                INT NULL,
    url_suffix                          TEXT NULL,
    follow_redirect                     BOOL NULL,
    PRIMARY KEY(probe_id)
)";

const INSERT_HTTP_PROBES_STMT: &'static str = "
INSERT INTO http_probes (
    probe_id,
    probe_interval_seconds,
    timeout_seconds,
    attempts_to_declare_failure,
    domain,
    port,
    url_suffix,
    follow_redirect
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

const CREATE_HTTP_PROBE_RESULTS_TABLE_STMT: &'static str = "
CREATE TABLE http_probe_results (
    probe_id                            INT NOT NULL,
    prober_hostname                     TEXT NOT NULL,
    timestamp_sec                       INT NOT NULL,
    success                             BOOLEAN NOT NULL,
    error_message                       TEXT NULL,
    application_layer_latency_nanosec   INT NULL,
    http_status_code                    INT NULL,
    http_status_message                 TEXT NULL,
    application_bytes_received          INT NULL,
    PRIMARY KEY(probe_id, prober_hostname, timestamp_sec)
)";

const INSERT_HTTP_PROBE_RESULTS_STMT: &'static str = "
INSERT INTO http_probe_results (
    probe_id,
    prober_hostname,
    timestamp_sec,
    success,
    error_message,
    application_layer_latency_nanosec,
    http_status_code,
    http_status_message,
    application_bytes_received
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";

pub struct Sqlite3Writer {
    conn: Connection,
}

impl Sqlite3Writer {
    pub fn open(path: &Path) -> Result<Sqlite3Writer, Error> {
        Ok (
            Sqlite3Writer {
                conn: try!(Connection::open(path)),
            }
        )
    }
}

impl Writer for Sqlite3Writer {
    fn init(&self) -> Result<(), String> {
        if let Err(e) = self.conn.execute(CREATE_HTTP_PROBE_RESULTS_TABLE_STMT, &[]) {
            return Err(format!("{}", e));
        }

        if let Err(e) = self.conn.execute(CREATE_HTTP_PROBES_TABLE_STMT, &[]) {
            return Err(format!("{}", e));
        }

        Ok(())
    }

    fn write_probe(&self, probe: Probe) -> Result<(), String> {
        match self.conn.execute(
                INSERT_HTTP_PROBES_STMT, &[
                    &(probe.get_probe_id() as i64),
                    &(probe.get_probe_interval_seconds() as i32),
                    &(probe.get_timeout_seconds() as i32),
                    &(probe.get_attempts_to_declare_failure() as i32),
                    &probe.get_domain(),
                    &(probe.get_port() as i32),
                    &probe.get_url_suffix(),
                    &probe.get_follow_redirect()
                ]
            ) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn write_probe_result(&self, probe_result: ProbeResult) -> Result<(), String> {
        match self.conn.execute(
                INSERT_HTTP_PROBE_RESULTS_STMT, &[
                    &(probe_result.get_probe_id() as i64),
                    &probe_result.get_prober_hostname(),
                    &(probe_result.get_timestamp_sec() as i64),
                    &probe_result.get_success(),
                    &probe_result.get_error_message(),
                    &(probe_result.get_application_layer_latency_nanosec() as i64),
                    &(probe_result.get_http_status_code() as i32),
                    &probe_result.get_http_status_message(),
                    &(probe_result.get_application_bytes_received() as i64),
                ]
            ) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn close(&self) -> Result<(), String> {
        /*match self.conn.close() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }*/

        unimplemented!();
    }
}
