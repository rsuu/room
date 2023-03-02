// USAGE:
//   cargo run - tcp 0.0.0.0 12345 &
//   cargo run - t 0.0.0.0 12345 "hi" &
//   curl -v 0.0.0.0:12345/hi

use native_tls::TlsConnector;
use std::io::{self, prelude::*, BufReader, Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{atomic::AtomicU8, Arc};
use std::thread::{self, sleep, sleep_ms};
use std::time::Duration;

// auto trait
trait ForTcpStream: Read + Write {}
impl<T: Read + Write> ForTcpStream for T {}

pub type Port = u16;

pub struct Tcp {
    ip: String,
    port: Port,

    with_tls: bool,
}

pub enum HttpStatus {
    Ok(u16, String),
    Err(u16, String),
}

impl Tcp {
    pub fn client(ip: &str, port: &str, text: &str) {
        let addr = format!("{ip}:{port}");

        let mut stream: Box<dyn ForTcpStream> = {
            let tmp = TcpStream::connect(addr).unwrap();
            if port == "443" {
                let tls = TlsConnector::new().unwrap();
                let mut tmp = tls.connect(ip, tmp).unwrap();

                Box::new(tmp)
            } else {
                Box::new(tmp)
            }
        };

        stream.write(text.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn server(addr: &str) {
        let bind = TcpListener::bind(addr).unwrap();

        for stream in bind.incoming() {
            let mut stream = stream.unwrap();
            stream.set_write_timeout(Some(Duration::from_millis(10000)));
            stream.set_read_timeout(Some(Duration::from_millis(10000)));

            thread::spawn(move || {
                println!(
                    "peer: {}:{}",
                    stream.peer_addr().unwrap().ip(),
                    stream.peer_addr().unwrap().port()
                );

                let mut buf = vec![0; 2048 + 35];
                let mut stauts = HttpStatus::Ok(200, "OK".to_string());
                let len = stream.read(&mut buf).unwrap();
                let req = std::str::from_utf8(&buf[0..len]).unwrap();

                if req.starts_with("GET /") {
                    dbg!("GET");

                    let con = req.split_once("GET /").unwrap().1;
                    let con = con.split_once(" HTTP/1.1").unwrap().0;

                    match con {
                        "hi" => {
                            println!("HI");

                            stream.write(stauts.res1().as_bytes()).unwrap();
                        }
                        "exit" => {
                            println!("exit");
                            std::process::exit(0);
                        }

                        _ => {}
                    }
                } else if req.starts_with("POST") {
                    dbg!("POST");
                    let json = req;
                } else if req.starts_with("PUT") {
                    dbg!("PUT");
                } else if req.starts_with("DELETE") {
                } else {
                    dbg!(req);
                }
            });

            sleep_ms(1);
        }
    }
}

impl HttpStatus {
    pub fn res1(&self) -> String {
        match self {
            Self::Ok(code, res) => format!("HTTP/1.1 {code} {res}\r\n\r\n"),
            Self::Err(code, res) => format!("HTTP/1.1 {code} {res}\r\n\r\n"),
        }
    }
}

mod test {
    #[test]
    fn name() {
        todo!();
        // test
        tmp.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
        let mut res = vec![0; 2048 + 35];
        tmp.read(&mut res).unwrap();

        println!("res: {} :", std::str::from_utf8(&res).unwrap());
    }
}
