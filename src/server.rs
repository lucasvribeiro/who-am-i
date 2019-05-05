use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use tokio::prelude::*;
use tokio::timer::Delay;

use std::time::{Instant, Duration};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    // let mut jogadores = Vec::new();
    while match stream.read(&mut data) {
        Ok(size) => {
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
    let teste = Instant::now();
    // println!("Ligou: {:?} ", when);
    
    // start_server();

    // define intervalo para inicio da partida
    let start_time = Instant::now() + Duration::from_millis(10000); // em milisegundos
    tokio::run({
        Delay::new(start_time)
            .map_err(|e| panic!("timer failed; err={:?}", e))
            .and_then(|_| {
                println!("Hello world!");
                Ok(())
            })
    });
    let test2 = Instant::now();
    let tempo = test2 - teste;
    println!("Tempo: {:?}", tempo);
}

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
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