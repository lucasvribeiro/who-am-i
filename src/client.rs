
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

mod aux_functions;

fn main() {
    match TcpStream::connect("127.0.0.1:3333") {
        Ok(mut stream) => {
            println!("CONECTADO COM SUCESSO AO SERVIDOR");
            println!("SEJA BEM-VINDO AO WHO I AM!");

            //recebe o nome do jogador
            let nome = aux_functions::read_name();
            
            //envia nome do jogador ao servidor
            stream.write(nome.as_bytes()).unwrap();

            //dados a serem recebidos do servidor
            let mut data = [0 as u8; 50];
            while match stream.read(&mut data) {
                Ok(size) => {
                  let novo_jogador = from_utf8(&data).unwrap();
                  println!("[NOVO JOGADOR CONECTADO: ] {}", novo_jogador);
                  true
                },
                Err(_) => {
                  println!("FALHA AO RECEBER MENSAGEM");
                  false
                }
            } {}
        },
        Err(e) => {
            println!("FALHA AO CONECTAR: {}", e);
        }
    }
    println!("Terminated.");
}