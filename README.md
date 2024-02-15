# port-scanner
A concurrent SYN port scanner in Rust.

Usage:

```bash
port_scanner 127.0.0.1
```

TODO: 
- Add tokio for concurrency/signal handling.
- Switch to socket2 or pnet to send SYN signals instead of establishing a full connection.
