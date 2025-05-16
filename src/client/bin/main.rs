use std::net::{TcpStream};
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
    loop {

        let mut buffer = [0 as u8; 2048];
        let recibido = stream.read(&mut buffer).unwrap(); 
        let mensaje = String::from_utf8_lossy(&buffer[..recibido]);
        let mensaje = mensaje.trim_end();
        println!("Server > {mensaje}");
        
        let message = get_message(); 
        let _ = stream.write(&message);
    }
}

fn main(){

    if let Ok(stream) = TcpStream::connect("192.168.1.105:9999") {
        handle_connection(stream);
    } else {
        println!("Couldn't connect to server...");
    }
}
