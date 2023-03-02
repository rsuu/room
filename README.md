# room

Examples for use TCP/UDP in Rust.

## USAGE

```zsh
# TCP
cargo run -- tcp 0.0.0.0 12345 &
  # server
cargo run -- t 0.0.0.0 12345 "GET /hi HTTP/1.1"
cargo run -- t 0.0.0.0 12345 "GET /exit HTTP/1.1"
curl -v 0.0.0.0:12345/hi

# UDP
cargo run -- udp 0.0.0.0 12345 &
  # server
cargo run -- u 0.0.0.0 12345 "hi"
cargo run -- u 0.0.0.0 12345 "exit"
echo "hi" | netcat -w 1 -u 0.0.0.0 12345
```
