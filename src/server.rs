use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use tokio::prelude::*;
use std::collections::HashMap;

use std::time::{Instant, Duration};

static mut GAME: GameStruct = GameStruct {
    jogadores: HashMap::new(),
    mensagem: String::new(),
};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    // let mut jogadores = Vec::new();
    while match stream.read(&mut data) {
        Ok(size) => {
            println!("Dados do cliente: {}", from_utf8(&data[0..size]).unwrap());
            stream.write(&data[0..size]).unwrap();
            // jogadores.push(nome_recebido);
            // // for x in &jogadores {
            // //   println!("{}", x);
            // // }
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn start_game() {
    let start_time = Instant::now();
    // println!("Ligou: {:?} ", when);

    // let mut jogadores = HashMap::new();

    unsafe {GAME.jogadores.insert("Jogador".to_string(), (0, "Mensagem".to_string()));}
    // jogadores.insert("Jogador".to_string(), (0, "Mensagem".to_string()));
    // let jogador = ("", 0, ""); // (nome, pontos, mensagem)
    
    start_server(start_time);
}

fn start_server(start_time: Instant) {
    let mut current_time = Instant::now();                              // usado para controlar tempo pre partida
    let duracao = Duration::from_millis(10000);                         // tempo para jogadores se conectarem
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();        // defina porta para escuta

    unsafe {let pontoxio = GAME.jogadores.get("Jogador");
    println!("AQEUII: {:?}", pontoxio);}


    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                current_time = Instant::now();
                // contrala tempo para jogadores se conectarem antes da partida
                if current_time-start_time < duracao {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move|| {
                        // connection succeeded
                        handle_client(stream)
                    });
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

    // close the socket server
    drop(listener);
}

struct GameStruct {
    jogadores: HashMap <String, (u32, String)>,
    mensagem: String,
}