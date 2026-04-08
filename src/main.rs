use std::io::Read;

fn main() {
    let listener = std::net::TcpListener::bind("192.168.1.66:53").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let mut buf = String::new();
                s.read_to_string(&mut buf).unwrap();
                println!("{buf}");
            }
            Err(e) => println!("{e}"),
        }
    }
}
