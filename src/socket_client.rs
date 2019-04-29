use std::net::{TcpStream};
use std::io::{Write};

mod aux_functions;

fn main() {
  let nome;
    match TcpStream::connect("127.0.0.1:1555") {
        Ok(mut stream) => {
            println!("CONECTADO COM SUCESSO!");
            println!("SEJA BEM-VINDO!");

            nome = aux_functions::read_name();

            stream.write(nome.as_bytes());
        },
        Err(e) => {
            println!("Falha ao conectar. Tente novamente! {}", e);
        }
    }
}