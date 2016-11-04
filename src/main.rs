extern crate docopt;
extern crate protobuf;
extern crate rustc_serialize;
extern crate rusqlite;

mod pb;
mod reader;
mod writer;

use std::fs::File;
use std::path::Path;

use reader::ProtobufReader;
use writer::Sqlite3Writer;

use docopt::Docopt;

const USAGE: &'static str = "
Application to parse probe result protobuf files

Usage:
    inqutil print (--probe | --result) <filename>
    inqutil write (--probe | --result) <filename> (--sqlite3 <db-filename>)
    inquitl (-h | --help)

Options:
    --sqlite3                 
    -h --help               Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_print: bool,
    cmd_write: bool,
    arg_db_filename: String,
    arg_filename: String,
    flag_probe: bool,
    flag_result: bool,
    flag_sqlite3: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());
    if args.cmd_print {
        let mut file = match File::open(args.arg_filename) {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };

        let protobuf_reader = if args.flag_result {
            ProtobufReader::new(&mut file)
        } else {
            panic!("no reader type found");
        };

        for protobuf in protobuf_reader {
            println!("protobuf: {:?}", protobuf);
        }
    } else if args.cmd_write {
        //open connection to sqlite3 database
        let writer = if args.flag_sqlite3 {
            let path = Path::new(&args.arg_db_filename);
            match Sqlite3Writer::open(path) {
                Ok(writer) => writer,
                Err(e) => panic!("{}", e),
            }
        } else {
            panic!("no writer type found");
        };

        //open file for reading
        if args.flag_result {
            let mut file = match File::open(args.arg_filename) {
                Ok(file) => file,
                Err(e) => panic!("{}", e),
            };

            let protobuf_reader = ProtobufReader::new(&mut file);
            for probe_result in protobuf_reader {
                match writer.write_probe_result(probe_result) {
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
            }
        } else {
            panic!("no reader type found");
        }
    }
}
