use room::tcp::Tcp;
use room::udp::Udp;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let cmd = args[1].as_str();
    let ip = args[2].as_str();
    let port = args[3].as_str();
    let addr = &format!("{ip}:{port}");

    println!("bind: {addr}");

    if cmd == "udp" {
        Udp::server(addr).unwrap();
    } else if cmd == "u" {
        let text = args[4].as_str();
        Udp::client(addr, text);
    } else if cmd == "tcp" {
        Tcp::server(addr);
    } else if cmd == "t" {
        let ip = args[2].as_str();
        let port = args[3].as_str();
        let text = args[4].as_str();
        Tcp::client(ip, port, text);
    }
}
