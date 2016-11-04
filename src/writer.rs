use std::path::Path;

use pb::proddle::ProbeResult;

use rusqlite::Error;
use rusqlite::Connection;

const CREATE_HTTP_PROBE_RESULTS_TABLE_STMT: &'static str = "
CREATE TABLE http_probe_results (
    probe_id                            UNSIGNED BIG INT,
    prober_hostname                     VARCHAR NOT NULL,
    timestamp_sec                       UNSIGNED BIG INT NOT NULL,
    success                             BOOLEAN NOT NULL,
    error_message                       VARCHAR(255) NULL,
    application_layer_latency_nanosec   UNSIGNED BIG INT NULL,
    http_status_code                    MEDIUMINT NULL,
    http_status_message                 VARCHAR(255) NULL,
    application_bytes_received          MEDIUMINT NULL,
    PRIMARY KEY(probe_id, timestamp_sec)
)";

const INSERT_HTTP_PROBE_RESULTS_STMT: &'static str = "
INSERT INTO http_probe_results(
    probe_id,
    prober_hostname,
    timestamp_sec,
    success,
    error_message,
    application_layer_latency_nanosec,
    http_status_code,
    http_status_message,
    application_bytes_received
) 
VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)";

pub struct Sqlite3Writer {
    conn: Connection,
}

impl Sqlite3Writer {
    pub fn open(path: &Path) -> Result<Sqlite3Writer, Error> {
        let create = !path.exists();
        let conn = try!(Connection::open(path));

        if create {
            try!(conn.execute(CREATE_HTTP_PROBE_RESULTS_TABLE_STMT, &[]));
        }

        Ok (
            Sqlite3Writer {
                conn: conn,
            }
        )
    }

    pub fn write_probe_result(&self, probe_result: ProbeResult) -> Result<(), Error> {
        try!(self.conn.execute(
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
        ));

        Ok(())
    }
}
