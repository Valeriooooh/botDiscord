use std::net::TcpListener;

pub fn server() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        println!("coonection",);
        let _stream = stream.unwrap();
    }
}
