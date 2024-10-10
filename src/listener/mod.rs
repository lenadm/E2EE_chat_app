use std::net::TcpListener;
use std::io::{ BufReader, BufRead };

pub struct Listener {
    listener: TcpListener,
}

impl Listener {
    pub fn new(addr: &String) -> Listener {

        let tcp_listener = match TcpListener::bind(addr) {
            Err(msg) => panic!("Critical Error: {}", msg),
            Ok(listener) => listener,
        };

        let listener = Listener {
            listener: tcp_listener,
        };

        return listener
    }

    pub fn start_listening(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("new connection established on {}", stream.peer_addr()
                        .expect("Critical Error: unable to confirm peer ip address"));

                    let mut reader = BufReader::new(stream);
                    let mut msg = String::new();
                    reader.read_line(&mut msg).expect("err");
                    println!("{}", msg);
                }
                Err(msg) => panic!("Critical Error: unable to establish connection to client {}", msg),
            };
        }
    }
}
