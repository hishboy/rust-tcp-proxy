use std::env;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

mod macros;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return println!("Example usage: cargo run 127.0.0.1:8000 127.0.0.1:3434");
    }

    let (proxy_addr, to_addr) = (&args[1], &args[2]);
    let listener = TcpListener::bind(proxy_addr).expect("Unable to bind proxy addr");

    println!("Proxing TCP packets from {} to {}", proxy_addr, to_addr);

    for incoming_stream in listener.incoming() {
        let proxy_stream = try_or_continue!(incoming_stream);
        let conn_thread = TcpStream::connect(to_addr)
            .map(|to_stream| thread::spawn(move || handle_conn(proxy_stream, to_stream)));

        match conn_thread {
            Ok(_) => { println!("Successfully established a connection with client"); }
            Err(err) => { println!("Unable to establish a connection with client {}", err); }
        }
    }
}

fn handle_conn(lhs_stream: TcpStream, rhs_stream: TcpStream) {
    let lhs_arc = Arc::new(lhs_stream);
    let rhs_arc = Arc::new(rhs_stream);

    let (mut lhs_tx, mut lhs_rx) = (lhs_arc.try_clone().unwrap(), lhs_arc.try_clone().unwrap());
    let (mut rhs_tx, mut rhs_rx) = (rhs_arc.try_clone().unwrap(), rhs_arc.try_clone().unwrap());

    let connections = vec![
        thread::spawn(move || io::copy(&mut lhs_tx, &mut rhs_rx).unwrap()),
        thread::spawn(move || io::copy(&mut rhs_tx, &mut lhs_rx).unwrap()),
    ];

    for t in connections {
        t.join().unwrap();
    }
}
