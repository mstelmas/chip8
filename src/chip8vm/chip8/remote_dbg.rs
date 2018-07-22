use std::process;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::sync::mpsc;

pub struct RemoteDbg {

}

#[derive(Debug)]
pub enum DbgMessage {
    START,
    STOP
}

impl RemoteDbg {
    pub fn init(sender: mpsc::Sender<DbgMessage>) {
        let dbg_thread = thread::spawn(  move|| {
            let listener = TcpListener::bind("0.0.0.0:9876").unwrap();
            for stream in listener.incoming() {
                RemoteDbg::handle_dbg_client(stream.unwrap(), sender.clone());
            }
        });
    }

    fn handle_dbg_client(mut stream: TcpStream, sender: mpsc::Sender<DbgMessage>) {
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        loop {
            let mut line = String::new();
            let result = reader.read_line(&mut line);

            match result {
                Ok(_) => {

                    let command = line.trim().to_uppercase();

                    if command == "DISCONNECT" {
                        stream.shutdown(Shutdown::Both);
                        break;
                    } else if command == "STOP" {
                        sender.send(DbgMessage::STOP);
                    } else if command == "START" {
                        sender.send(DbgMessage::START);
                    }

                    writer.flush();

                }
                Err(_) => break
            }
        }
    }
}
