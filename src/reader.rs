use std::fs::File;
use std::iter::Iterator;

use pb::proddle::{Probe, ProbeResult};

use protobuf::{CodedInputStream, Message, ProtobufError};

/*
 * ProbeReader
 */
pub struct ProbeReader<'a> {
    coded_input_stream: CodedInputStream<'a>,
}

impl<'a> ProbeReader<'a> {
    pub fn new(file: &'a mut File) -> ProbeReader<'a> {
        ProbeReader {
            coded_input_stream: CodedInputStream::new(file),
        }
    }
}

impl<'a> Iterator for ProbeReader<'a> {
    type Item = Probe;

    fn next(&mut self) -> Option<Probe> {
        //check for end of file
        if self.coded_input_stream.eof().unwrap() {
            return None;
        }

        //parse probe
        let mut probe= Probe::new();
        let _ = read_protobuf(&mut self.coded_input_stream, &mut probe);
        Some(probe)
    }
}

/*
 * ProbeResultReader
 */
pub struct ProbeResultReader<'a> {
    coded_input_stream: CodedInputStream<'a>,
}

impl<'a> ProbeResultReader<'a> {
    pub fn new(file: &'a mut File) -> ProbeResultReader<'a> {
        ProbeResultReader {
            coded_input_stream: CodedInputStream::new(file),
        }
    }
}

impl<'a> Iterator for ProbeResultReader<'a> {
    type Item = ProbeResult;

    fn next(&mut self) -> Option<ProbeResult> {
        //check for end of file
        if self.coded_input_stream.eof().unwrap() {
            return None;
        }

        //parse probe result
        let mut probe_result = ProbeResult::new();
        let _ = read_protobuf(&mut self.coded_input_stream, &mut probe_result);
        Some(probe_result)
    }
}

fn read_protobuf(coded_input_stream: &mut CodedInputStream, message: &mut Message) -> Result<(), ProtobufError> {
    //read length
    let length = try!(coded_input_stream.read_uint32());

    //read bytes for messages
    let mut bytes = Vec::new();
    for _ in 0..length {
        let byte = try!(coded_input_stream.read_raw_byte());
        bytes.push(byte);
    }

    //parse message
    let mut message_input_stream = CodedInputStream::from_bytes(&bytes);
    let _ = message.merge_from(&mut message_input_stream);
    Ok(())
}
