use std::net::{TcpStream};
use std::io::{Write, Read};
use std::str::from_utf8;

mod aux_functions;

fn main() {
    
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("SEJA BEM-VINDO AO WHO I AM!");

            println!("--------------------------");
            println!("Aguarde enquanto o jogo começa...");
            println!("--------------------------");

            let mut data = [0 as u8; 50];
            let mut recebido = String::new();
            let mut nome_jogador = String::new();
            let mut tip = String::new();
            let mut answer = String::new();
            let mut palpite = String::new();
            let mut ask = String::new();
            let mut resposta = String::new();
            let mut mestre = String::new();
            let mut jog_vez = String::new();
            let mut sn = String::new();
            loop {
              match stream.read(&mut data) {
                Ok(size) => {

                  recebido = from_utf8(&data[0..size]).unwrap().to_string();

                  //recebe nome do jogador
                  if recebido == "nome_jogador" {
                    nome_jogador.push_str(&aux_functions::read_name().to_string());
                    stream.write(nome_jogador.as_bytes()).unwrap();
                    println!("");
                  }

                  if recebido.starts_with("mestre_rodada"){
                    let aux: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    mestre.push_str(&aux.get(0).unwrap());
                    if recebido.ends_with(&nome_jogador){
                      println!("---------------------------");
                      println!("VOCÊ É O MESTRE DA RODADA");
                      println!("---------------------------");

                      tip.push_str(&aux_functions::read_tip().to_string());
                      stream.write(tip.as_bytes()).unwrap();

                      answer.push_str(&aux_functions::read_answer().to_string());
                      stream.write(answer.as_bytes()).unwrap();
                    } else {
                      println!("---------------------------");
                      println!("Aguardando dica e resposta do mestre da rodada...");
                      println!("---------------------------");
                    }
                  }

                  if recebido.starts_with("jogo_iniciado"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    println!("---------- JOGO INICIADO ----------");
                    println!("[DICA: ] {}", values.get(1).unwrap());
                    if &mestre.trim() == &nome_jogador.trim() {
                      println!("---------------------------");
                      println!("Aguardando pergunta do jogador da vez...");
                      println!("---------------------------");
                    }
                  }

                  if recebido.starts_with("jogador_vez"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    jog_vez = values.get(0).unwrap().to_string();
                    if &jog_vez.trim() == &nome_jogador.trim() {
                      println!("---------------------------");
                      println!("É Sua vez...");
                      println!("---------------------------");
                      ask.clear();
                      ask.push_str(&aux_functions::read_ask().to_string());
                      println!("---------------------------");
                      println!("Aguarde a resposta do mestre...");
                      println!("---------------------------");
                      stream.write(ask.as_bytes()).unwrap();
                    } else if &mestre.trim() != &nome_jogador.trim(){
                      println!("---------------------------");
                      println!("Vez do jogador [{}]. Aguarde...", jog_vez.trim());
                      println!("---------------------------");
                    }
                  }

                  if recebido.starts_with("ask"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    ask = values.get(0).unwrap().to_string();
                    println!("[{}] perguntou: {}", jog_vez.trim(), ask);
                    sn.clear();
                    sn.push_str(&aux_functions::read_sn().to_string());
                    stream.write(sn.as_bytes()).unwrap();
                  }

                  
                  if recebido.starts_with("sn"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    sn = values.get(0).unwrap().to_string();
                    println!("[ RESPOSTA : ] {}", sn.trim());
                    palpite.clear();
                    palpite.push_str(&aux_functions::read_palpite().to_string());
                    stream.write(palpite.as_bytes()).unwrap();
                  }

                  if recebido.starts_with("palpite"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    palpite = values.get(0).unwrap().to_string();
                    println!("[ PALPITE : ] {}", palpite.trim());
                    resposta.clear();
                    resposta.push_str(&aux_functions::read_resposta().to_string());
                    stream.write(resposta.as_bytes()).unwrap();
                  }

                  if recebido.starts_with("resposta"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    resposta = values.get(0).unwrap().to_string();
                    println!("[ JOGADOR {} TENTOU PALPITE E  ESTÁ: ] {}", jog_vez.trim(), resposta.trim());
                  }

                },
                Err(e) => {
                  println!("FALHA AO RECEBER MENSAGEM: {}", e);
                }
              }
            }
            // } 
        },
        Err(e) => {
            println!("ERRO AO CONECTAR: {}", e);
        }
    }
    println!("JOGO TERMINADO.");
}