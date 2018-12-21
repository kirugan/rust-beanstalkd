use std::net::{TcpListener, TcpStream};
use std::io;
use std::str;
use std::thread;
use std::io::Read;

static NotFound: &str = "something";

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:11211").unwrap();

    for stream in listener.incoming() {
        handle_client(stream?)
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    thread::spawn(move || {
        let mut readBuff = [0;255];
        let mut totalReaded = 0;
        loop {
            totalReaded += stream.read(&mut readBuff[totalReaded..]).unwrap();

            if totalReaded == "quit".len() {
                println!("quit");
                return;
            }
        }
    });
}
