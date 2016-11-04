extern crate docopt;
extern crate protobuf;
extern crate rustc_serialize;
extern crate rusqlite;

mod pb;
mod reader;

use std::fs::File;
use std::path::Path;

use reader::ProtobufReader;

use docopt::Docopt;
use rusqlite::Connection;

const CREATE_TABLE: &'static str = "
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

const INSERT: &'static str = "
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

const USAGE: &'static str = "
Application to parse probe result protobuf files

Usage:
    inqutil init-sqlite <filename>
    inqutil insert-sqlite <filename> <db-filename>
    inqutil print <filename>
    inquitl (-h | --help)

Options:
    -h --help               Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_init_sqlite: bool,
    cmd_insert_sqlite: bool,
    cmd_print: bool,
    arg_db_filename: String,
    arg_filename: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());
    if args.cmd_insert_sqlite {
        //open file for reading
        let mut file = match File::open(args.arg_filename) {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };

        let protobuf_reader = ProtobufReader::new(&mut file);

        //open connection to sqlite3 database
        let path = Path::new(&args.arg_db_filename);
        if !path.exists() {
            panic!("db file does not exist");
        }
        
        let conn = match Connection::open(path) {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e),
        };

        //insert records
        for probe_result in protobuf_reader {
            let result = conn.execute(
                INSERT, &[
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
            );

            if result.is_err() {
                panic!("{:?}", result);
            }
        }
    } else if args.cmd_init_sqlite {
        let path = Path::new(&args.arg_filename);
        if path.exists() {
            panic!("file already exists");
        }
        
        let conn = match Connection::open(path) {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e),
        };

        let result = conn.execute(CREATE_TABLE, &[]);
        if result.is_err() {
            panic!("{:?}", result);
        }
    } else if args.cmd_print {
        let mut file = match File::open(args.arg_filename) {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };

        let protobuf_reader = ProtobufReader::new(&mut file);
        for probe_result in protobuf_reader {
            println!("probe_result: {:?}", probe_result);
        }
    }
}
