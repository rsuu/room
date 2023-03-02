use std::io::BufReader;
use std::io::Read;
use std::net::UdpSocket;
use std::str;
use std::thread;
use std::thread::sleep_ms;
use std::time::Duration;

pub struct Udp {}

impl Udp {
    pub fn client(addr: &str, text: &str) {
        let me = "0.0.0.0:0";
        let client = UdpSocket::bind(me).unwrap();

        client.send_to(text.as_bytes(), addr).unwrap();

        client.set_read_timeout(Some(Duration::from_millis(1000)));
        let mut res = vec![0; 508];
        if let Ok((len, addr)) = client.recv_from(&mut res) {
            println!("res: {}", std::str::from_utf8(&res[0..len]).unwrap());
        }
    }

    pub fn server(addr: &str) -> anyhow::Result<()> {
        let udp_socket = UdpSocket::bind(addr).unwrap();

        loop {
            // safe packet size of 508 = 576 - 60 (IP header) - 8 (udp header) is reasonable.
            let mut buf = vec![0; 508];
            let socket = udp_socket.try_clone().unwrap();
            let res = socket.recv_from(&mut buf);

            if let Ok((len, peer)) = res {
                thread::spawn(move || {
                    let input = str::from_utf8(&buf[0..len]).unwrap_or("").trim_end();

                    match input {
                        "hi" => {
                            if socket.send_to(b"Hi", peer).is_ok() {
                                println!("peer: {peer}");
                            } else {
                                println!("err");
                            }
                        }
                        "exit" => {
                            println!("exit");
                            std::process::exit(0);
                        }

                        _ => {}
                    }
                });
            } else if let Err(ref e) = res {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                } else {
                    panic!("");
                }
            }

            sleep_ms(1);
        }

        Ok(())
    }
}
