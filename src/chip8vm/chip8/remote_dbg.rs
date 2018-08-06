use std::process;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::sync::mpsc;
use std::borrow::Borrow;

use serde_json::from_str;
use serde_json::error::Error;
use bincode::{serialize, deserialize};

#[derive(Debug)]
pub enum DbgMessage {
    CPU,
    START,
    STOP,
    STEP,
    RESTART,
    MEM(u16, usize)
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuSnapshot {
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
}

#[derive(Debug)]
pub enum Chip8Snapshots {
    CPU(CpuSnapshot),
    MEM(Vec<u8>)
}

pub struct RemoteDbg;

impl RemoteDbg {
    pub fn init(sender: mpsc::Sender<DbgMessage>, snapshots: mpsc::Receiver<Chip8Snapshots>) {
        let dbg_thread = thread::spawn(  move|| {
            let listener = TcpListener::bind("0.0.0.0:9876").unwrap();
            for stream in listener.incoming() {
                RemoteDbg::handle_dbg_client(stream.unwrap(), sender.clone(), snapshots.borrow());
            }
        });
    }

    fn handle_dbg_client(mut stream: TcpStream, sender: mpsc::Sender<DbgMessage>, snapshots: &mpsc::Receiver<Chip8Snapshots>) {
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        loop {
            let mut line = String::new();
            let result = reader.read_line(&mut line);

            match result {
                Ok(_) => {
                    let command: DbgCommand = from_str(&line).unwrap();

                    match command {
                        DbgCommand::Start =>  { sender.send(DbgMessage::START); }
                        DbgCommand::Step =>  { sender.send(DbgMessage::STEP); }
                        DbgCommand::Stop => { sender.send(DbgMessage::STOP); }
                        DbgCommand::Cpu => {
                            sender.send(DbgMessage::CPU);
                            // TODO: timeouts
                            let cpu_snapshot = snapshots.recv().unwrap();

                            match cpu_snapshot {
                                Chip8Snapshots::CPU(snapshot) => {
                                    let encoded = serialize(&snapshot).unwrap();
                                    writer.write_all(encoded.as_ref());
                                    writer.write(b"\n").unwrap();
                                }
                                _ => {}
                            };
                        },
                        DbgCommand::Mem(addr, size) => {
                            sender.send(DbgMessage::MEM(addr, size));
                            let mem_dump = snapshots.recv().unwrap();

                            match mem_dump {
                                Chip8Snapshots::CPU(snapshot) => {},
                                Chip8Snapshots::MEM(mem) => {
                                    writer.write_all(mem.as_ref());
                                    writer.write_all(b"\n");
                            }
                            };
                        }
                    }
                    writer.flush();

                }
                Err(_) => { error!("In error!"); }
            }
        }
    }
}
