# port-scanner
A concurrent SYN port scanner in Rust.

Usage:

```bash
port_scanner 127.0.0.1
```

TODO: 
- Switch to socket2 to send SYN signals instead of establishing a full connection.
- Establish a way to exit the program gracefully.
