use std::fs::File;
use std::iter::Iterator;

use pb::proddle::ProbeResult;

use protobuf::{CodedInputStream, Message};

pub struct ProtobufReader<'a> {
    coded_input_stream: CodedInputStream<'a>,
}

impl<'a> ProtobufReader<'a> {
    pub fn new(file: &'a mut File) -> ProtobufReader<'a> {
        ProtobufReader {
            coded_input_stream: CodedInputStream::new(file),
        }
    }
}

impl<'a> Iterator for ProtobufReader<'a> {
    type Item = ProbeResult;

    fn next(&mut self) -> Option<ProbeResult> {
        //check for end of file
        if self.coded_input_stream.eof().unwrap() {
            return None;
        }

        //read length
        let length = self.coded_input_stream.read_uint32().unwrap();

        //read bytes for messages
        let mut bytes = Vec::new();
        for _ in 0..length {
            let byte = self.coded_input_stream.read_raw_byte().unwrap();
            bytes.push(byte);
        }

        //parse message
        let mut message_input_stream = CodedInputStream::from_bytes(&bytes);
        let mut probe_result = ProbeResult::new();
        let _ = probe_result.merge_from(&mut message_input_stream);
        Some(probe_result)
    }
}
