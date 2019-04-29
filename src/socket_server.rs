use std::net::{TcpListener, TcpStream};
use std::io;


fn handle_client(mut stream: TcpStream) {
  println!("Jogador que entrou...");
}

fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1555")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn main () {
  start_server();
}