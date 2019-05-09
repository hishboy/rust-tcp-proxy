# rust-tcp-proxy
Forwards TCP packets from `127.0.0.1:<PORT>` to `<ADDRESS>:<PORT>`

### Setup
Install Rust and Cargo 
```
curl https://sh.rustup.rs -sSf | sh
``` 
The command will download a script, and start the installation. If everything goes well, you’ll see this appear:
```
Rust is installed now. Great!
```

### Usage: 
The following command will start a server on port 8000 and forward all tcp packets to 172.217.20.110 port 5000
`cargo run 127.0.0.1:8000 172.217.20.110:5000`
