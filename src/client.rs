use std::net::{TcpStream};
use std::io::{Write, Read};
use std::str::from_utf8;

mod aux_functions;

fn main() {
    
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("SEJA BEM-VINDO AO WHO AM I!");

            println!("--------------------------");
            println!("Aguarde enquanto o jogo começa...");
            println!("--------------------------");

            let mut data = [0 as u8; 255];
            let mut recebido;                       // Guarda mensagem recebida do servidor
            let mut nome_jogador = String::new();   // Guarda jogador "desse" terminal
            let mut tip = String::new();            // Dica
            let mut answer = String::new();         // Resposta da rodada
            let mut palpite = String::new();        // Palpite
            let mut ask = String::new();            // Pergunta
            let mut resposta = String::new();       // Resposta do palpite
            let mut mestre = String::new();         // Quem é o mestre
            let mut jog_vez = String::new();        // Quem é o jogador da vez
            let mut sn = String::new();             // Resposta do mestre para a pergunta
            let mut continuar = String::new();      // Resposta se jogador deseja continuar jogando
            loop {
              match stream.read(&mut data) {
                Ok(size) => {

                  recebido = from_utf8(&data[0..size]).unwrap().to_string();

                  // Recebe nome do jogador
                  if recebido == "nome_jogador" {
                    nome_jogador.push_str(&aux_functions::read_name().to_string());
                    stream.write(nome_jogador.as_bytes()).unwrap();
                    println!("");
                  }

                  // Recebe informação sobre o mestre da rodada
                  if recebido.starts_with("mestre_rodada"){       // Se o jogador é o mestre da rodada
                    let aux: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    mestre.push_str(&aux.get(0).unwrap());
                    if recebido.ends_with(&nome_jogador){
                      println!("---------------------------");
                      println!("VOCÊ É O MESTRE DA RODADA");
                      println!("---------------------------");

                      // Recebe dica e envia para servidor
                      tip.push_str(&aux_functions::read_tip().to_string());
                      stream.write(tip.as_bytes()).unwrap();

                      // Recebe resposta da rodada e envia para o servidor
                      answer.push_str(&aux_functions::read_answer().to_string());
                      stream.write(answer.as_bytes()).unwrap();
                    
                    } else {                                      // Se o jogador não é o mestre da rodada
                      println!("---------------------------");
                      println!("Aguardando dica e resposta do mestre da rodada...");
                      println!("---------------------------");
                    }
                  }

                  // Se o jogo começou
                  if recebido.starts_with("jogo_iniciado"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    println!("\n---------- JOGO INICIADO ----------");
                    println!("[DICA: ] {}", values.get(1).unwrap());        // Imprime dica
                    if &mestre.trim() == &nome_jogador.trim() {             // Se o jogador é o mestre
                      println!("---------------------------");
                      println!("Aguardando pergunta do jogador da vez...");
                      println!("---------------------------");
                    }
                  }

                  // Se é a vez dos jogadores perguntarem
                  if recebido.starts_with("jogador_vez"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    jog_vez = values.get(0).unwrap().to_string();
                    
                    if &jog_vez.trim() == &nome_jogador.trim() {            // Se é a vez do jogador
                      println!("---------------------------");
                      println!("É Sua vez...");
                      println!("---------------------------");
                      ask.clear();
                      ask.push_str(&aux_functions::read_ask().to_string());
                      println!("---------------------------");
                      println!("Aguarde a resposta do mestre...");
                      println!("---------------------------");
                      stream.write(ask.as_bytes()).unwrap();
                    
                    } else if &mestre.trim() != &nome_jogador.trim(){       // Se não é a vez do jogador
                      println!("---------------------------");
                      println!("Vez do jogador [{}]. Aguarde...", jog_vez.trim());
                      println!("---------------------------");
                    }
                  }

                  // Só para o mestre. Exibe pergunta do jogador da vez e recebe resposta
                  if recebido.starts_with("ask"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    ask = values.get(0).unwrap().to_string();
                    println!("[{}] perguntou: {}", jog_vez.trim(), ask);
                    sn.clear();
                    sn.push_str(&aux_functions::read_sn().to_string());
                    stream.write(sn.as_bytes()).unwrap();
                  }

                  // Só para jogador da vez. Exibe resposta pergunta e recebe palpite
                  if recebido.starts_with("sn"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    sn = values.get(0).unwrap().to_string();
                    println!("[ RESPOSTA ] : {}", sn.trim());
                    palpite.clear();
                    palpite.push_str(&aux_functions::read_palpite().to_string());
                    stream.write(palpite.as_bytes()).unwrap();
                  }

                  // Só para o mestre. Exibe palpite do jogador da vez e recebe resposta
                  if recebido.starts_with("palpite"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    palpite = values.get(0).unwrap().to_string();
                    println!("[ PALPITE ] : {}", palpite.trim());
                    resposta.clear();
                    resposta.push_str(&aux_functions::read_resposta().to_string());
                    stream.write(resposta.as_bytes()).unwrap();
                  }

                  // Exibe para todos se o jogador acerto/errou o palpite
                  if recebido.starts_with("resposta"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    resposta = values.get(0).unwrap().to_string();
                    println!("\n Jogador [{}] tentou o palpite e está : {}", jog_vez.trim(), resposta.trim());
                  }

                  // Pergunta se jogador quer continuar e recebe resposta
                  if recebido.starts_with("continuar"){
                    println!("\n---------------------------");
                    println!("Você deseja continuar jogando?");
                    continuar.clear();
                    continuar.push_str(&aux_functions::read_continuar().to_string());
                    stream.write(continuar.as_bytes()).unwrap();
                    println!("---------------------------");
                    println!("Aguardando resposta dos outros jogadores...");
                    println!("---------------------------");
                  }

                  // Exibe para todos a tabela de highscore
                  if recebido.starts_with("###"){
                    let values: Vec<&str> = recebido.rsplit_terminator(":").collect();
                    resposta = values.get(0).unwrap().to_string();
                    println!("\n\n{}", resposta.trim());
                  }

                  // Exibe final do jogo
                  if recebido.starts_with("fim"){
                    println!("--------------- JOGO FINALIZADO ---------------");
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