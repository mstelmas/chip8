use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::io::{BufReader, BufWriter};

use super::disasm;

use serde_json::{Value, Error};
use serde_json::to_string;

use bincode::{serialize, deserialize};

pub struct Cli {
    tcp_stream: TcpStream
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuSnapshot {
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
}

// TODO: move to shared lib
#[derive(Serialize, Deserialize, Debug)]
enum DbgCommand {
    Cpu,
    Start,
    Step,
    Stop,
    Mem(u16, usize)
}

// TODO: Refactor to make it more sane
impl Cli {
    pub fn new() -> Cli {
        Cli {
            tcp_stream: TcpStream::connect("127.0.0.1:9876").unwrap()
        }
    }

    pub fn start(&mut self) {
        let mut writer = BufWriter::new(&self.tcp_stream);

        let command = to_string(&DbgCommand::Start).unwrap();

        writer.write(format!("{}\n", command).as_bytes());
        writer.flush().unwrap();
    }

    pub fn step(&mut self) {
        let mut writer = BufWriter::new(&self.tcp_stream);

        let command = to_string(&DbgCommand::Step).unwrap();

        writer.write(format!("{}\n", command).as_bytes());
        writer.flush().unwrap();
    }

    pub fn stop(&mut self) {
        let mut writer = BufWriter::new(&self.tcp_stream);

        let command = to_string(&DbgCommand::Stop).unwrap();

        writer.write(format!("{}\n", command).as_bytes());
        writer.flush().unwrap();
    }

    pub fn cpu(&mut self) -> CpuSnapshot {
        let mut writer = BufWriter::new(&self.tcp_stream);
        let mut reader = BufReader::new(&self.tcp_stream);

        let command = to_string(&DbgCommand::Cpu).unwrap();

        writer.write(format!("{}\n", command).as_bytes());
        writer.flush().unwrap();
        let mut buf: Vec<u8> = vec![];
        reader.read_until("\n".as_bytes()[0], &mut buf);

        deserialize(buf.as_ref()).unwrap()
    }

    pub fn mem(&mut self, addr: u16, size: usize) -> Vec<u8> {
        let mut writer = BufWriter::new(&self.tcp_stream);
        let mut reader = BufReader::new(&self.tcp_stream);

        let command = to_string(&DbgCommand::Mem(addr, size)).unwrap();

        writer.write(format!("{}\n", command).as_bytes());
        writer.flush().unwrap();
        let mut buf: Vec<u8> = vec![];
        reader.read_until("\n".as_bytes()[0], &mut buf);

        buf
    }

    fn read_stdin() -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().into()
    }
}
