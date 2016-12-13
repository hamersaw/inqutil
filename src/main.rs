extern crate docopt;
extern crate protobuf;
extern crate rustc_serialize;
extern crate rusqlite;

mod pb;
mod reader;
mod writer;

use std::fs::File;
use std::path::Path;

use reader::{ProbeReader, ProbeResultReader};
use writer::Writer;
use writer::postgresql::PostgresqlWriter;
use writer::print::PrintWriter;
use writer::sqlite3::Sqlite3Writer;

use docopt::Docopt;

const USAGE: &'static str = "
Application to parse probe result protobuf files

Usage:
    inqutil postgresql <username> (--probe | --result) <filename>
    inqutil print (--probe | --result) <filename>
    inqutil sqlite3 <db-filename> (--probe | --result) <filename>
    inquitl (-h | --help)

Options:
    -h --help               Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_postgresql: bool,
    cmd_print: bool,
    cmd_sqlite3: bool,
    arg_db_filename: String,
    arg_filename: String,
    arg_username: String,
    flag_probe: bool,
    flag_result: bool,
    flag_sqlite3: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    //open writer
    let writer: Box<Writer> = if args.cmd_postgresql {
        //open postgresql writer
        let writer = match PostgresqlWriter::open() {
            Ok(writer) => writer,
            Err(e) => panic!("{}", e),
        };

        Box::new(writer) as Box<Writer>
    } else if args.cmd_print {
        //open print writer
        Box::new(PrintWriter::new()) as Box<Writer>
    } else if args.cmd_sqlite3 {
        //open sqlite3 writer
        let path = Path::new(&args.arg_db_filename);
        let writer = match Sqlite3Writer::open(path) {
            Ok(writer) => writer,
            Err(e) => panic!("{}", e),
        };

        Box::new(writer) as Box<Writer>
    } else {
        panic!("unknown writer type");
    };

    //open file for reading
    let mut file = match File::open(args.arg_filename) {
        Ok(file) => file,
        Err(e) => panic!("{}", e),
    };

    //iterate over records
    if args.flag_probe {
        let probe_reader = ProbeReader::new(&mut file);
        for probe in probe_reader {
            if let Err(e) = writer.write_probe(probe) {
                println!("{}", e);
            }
        }
    } else if args.flag_result {
        let probe_result_reader = ProbeResultReader::new(&mut file);
        for probe_result in probe_result_reader {
            if let Err(e) = writer.write_probe_result(probe_result) {
                println!("{}", e);
            }
        }
    }

    //close writer
    if let Err(e) = writer.close() {
        panic!("{}", e);
    }
}
