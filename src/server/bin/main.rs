use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*};


fn get_message() -> Vec<u8> {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .unwrap();

    let bytes: Vec<u8> = message.into_bytes();
    bytes
}

fn handle_connection(mut stream: TcpStream){
    let addr = &stream.peer_addr().expect("Error");
    println!("New connection from {}", addr.ip());
    loop {
        let message = get_message(); 

        let mut buffer = [0 as u8; 2048];
        let _ = stream.write(&message);
        let received = stream.read(&mut buffer).unwrap();
        let received = String::from_utf8_lossy(&buffer[..received]);
        let received = received.trim_end();


        println!("{addr}> {received}");
    }
}

fn main(){

    let listener = TcpListener::bind("192.168.1.105:9999").unwrap();
    println!("Server up and listening");

    for stream in listener.incoming(){

        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("An error has occurred: {e}");
            }
        }
    }
}