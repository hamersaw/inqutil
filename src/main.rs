extern crate docopt;
extern crate protobuf;
extern crate rustc_serialize;

mod pb;
mod reader;

use std::fs::File;

use reader::ProtobufReader;

use docopt::Docopt;

const USAGE: &'static str = "
Application to parse probe result protobuf files

Usage:
    inqutil convert <filename> <output> 
    inqutil print <filename>
    inquitl (-h | --help)

Options:
    -h --help               Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_convert: bool,
    cmd_print: bool,
    arg_filename: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());
    if args.cmd_convert {
        let mut file = match File::open(args.arg_filename) {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };

        let protobuf_reader = ProtobufReader::new(&mut file);
        for probe_result in protobuf_reader {
            //TODO 
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
