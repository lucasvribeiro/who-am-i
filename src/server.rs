use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::collections::HashMap;

mod le_arquivo;

fn handle_client(conexoes: &Vec<TcpStream>) {
  let mut jogadores = vec![];
  let mut mestre_index = 0;
  let mut jog_vez_index = 1;
  let mut mestre;
  let mut conexao_mestre : &TcpStream;
  let mut conexao_vez : &TcpStream;
  let mut data = [0 as u8; 255];
  let mut tip = String::new();
  let mut answer = String::new();
  let mut resposta = String::new();
  let mut palpite = String::new();
  let mut ask = String::new();
  let mut jog_vez;
  let mut sn = String::new();
  let mut rodada_ativa = true;
  let mut nova_rodada = true;                 // Controla as rodadas
  let mut score: HashMap<String, String> = le_arquivo::le_arquivo();

  for mut conexao in conexoes {       // Para cada conexão realizada, pegue nome do jogador
    conexao.write(b"nome_jogador").unwrap();
    match conexao.read(&mut data){
      Ok(size) => {
        let mut jogador;
        jogador = from_utf8(&data[0..size]).unwrap().to_string();
        let aux_jogador = from_utf8(&data[0..size]).unwrap().to_string();       // Usada apenas para preencher o HashMap
        jogadores.push(jogador);

        // Insere jogador na lista de scores, caso não esteja lá
        if score.get(&aux_jogador.trim().to_string().clone()) == None {
          score.insert(aux_jogador.trim().to_string().clone(), String::from("0"));
        }
      },
      Err(e) => {
        println!("FALHA AO RECEBER MENSAGEM: {}", e);
      }
    }   
  }

  while nova_rodada {
    let mut continuam_str;                                                      // Guarda resposta recebida
    let mut continuam_int = 0;                                                  // Guarda quantidade que desejam continuar
    
    // Envia mestre do jogo
    mestre = jogadores.get(mestre_index).unwrap();
    let mut mestre_enviar: String = "mestre_rodada: ".to_owned();
    let aux2: &str = mestre;
    mestre_enviar.push_str(aux2);
    for mut conexao in conexoes {                                       // Envia para todos
      conexao.write(mestre_enviar.as_bytes()).unwrap();
    }

    conexao_mestre = conexoes.get(mestre_index).unwrap();                // Pega conexão do mestre

    // Pega dica do mestre
    match conexao_mestre.read(&mut data){
      Ok(size) => {
        tip = from_utf8(&data[0..size]).unwrap().to_string();
      },
      Err(e) => {
        println!("FALHA AO RECEBER MENSAGEM: {}", e);
      }
    }

    // Pega resposta do mestre
    match conexao_mestre.read(&mut data){
      Ok(size) => {
        answer = from_utf8(&data[0..size]).unwrap().to_string();
      },
      Err(e) => {
        println!("FALHA AO RECEBER MENSAGEM: {}", e);
      }
    }

    for mut conexao in conexoes {
      let mut jogo_iniciado = String::new();
      jogo_iniciado.push_str("jogo_iniciado:");
      jogo_iniciado.push_str(&tip);
      jogo_iniciado.push(':');
      jogo_iniciado.push_str(&answer);
      conexao.write(jogo_iniciado.as_bytes()).unwrap();

    }

    while rodada_ativa {
      conexao_vez = conexoes.get(jog_vez_index).unwrap();
      jog_vez = jogadores.get(jog_vez_index).unwrap().to_string();
      
      // Envia para todos quem é o jogador da vez
      for mut conexao in conexoes {
        let mut vez = String::new();
        vez.push_str("jogador_vez:");
        vez.push_str(&jog_vez);
        conexao.write(vez.as_bytes()).unwrap();

      }

      // Recebe pergunta do jogador da vez
      match conexao_vez.read(&mut data){
        Ok(size) => {
          ask.clear();
          ask.push_str("ask:");
          ask.push_str(&from_utf8(&data[0..size]).unwrap().to_string());
        },
        Err(e) => {
          println!("FALHA AO RECEBER MENSAGEM: {}", e);
        }
      }

      // Envia pergunta para o mestre
      conexao_mestre.write(ask.as_bytes()).unwrap();

      // Recebe resposta do mestre para a pergunta do jogador da vez
      match conexao_mestre.read(&mut data){
        Ok(size) => {
          sn.clear();
          sn.push_str("sn:");
          sn.push_str(&from_utf8(&data[0..size]).unwrap().to_string()); 
        },
        Err(e) => {
          println!("FALHA AO RECEBER MENSAGEM: {}", e);
        }
      }
      
      // Envia resposta mestre para jogador da vez
      conexao_vez.write(sn.as_bytes()).unwrap();

      // Recebe palpite do jogador da vez
      match conexao_vez.read(&mut data){
        Ok(size) => {
          palpite.clear();
          palpite.push_str("palpite:");
          palpite.push_str(&from_utf8(&data[0..size]).unwrap().to_string());
        },
        Err(e) => {
          println!("FALHA AO RECEBER MENSAGEM: {}", e);
        }
      }

      // Envia palpite para o mestre
      conexao_mestre.write(palpite.as_bytes()).unwrap();

      // Recebe resposta do mestre para o palpite do jogador da vez
      match conexao_mestre.read(&mut data){
        Ok(size) => {
          resposta.clear();
          resposta.push_str("resposta:");
          resposta.push_str(&from_utf8(&data[0..size]).unwrap().to_string());
          if resposta.contains("certo") {
            rodada_ativa = false;

            let aux_jogador2 = jog_vez.trim().to_string().clone();             // Utilizado apenas para atualizar o HashMap

            let mut num : i32 = 0;                          // Utilizado para acessar os pontos do jogador da vez
            // Acessa os pontos do jogador da vez
            match score.get(&aux_jogador2.clone()) {
              Some(ponto1) => {
                  num = ponto1.parse::<i32>().unwrap();     // Pega pontos e converte para int
                  num = num + 1;                            // Some o ponto ganhado na rodada
                  println!("Score atualizado!");
              },
              _ => println!("Falha ao atualizar score!"),
            }
            score.insert(aux_jogador2, num.to_string());    // Atualiza HashMap
          }
        },
        Err(e) => {
          println!("FALHA AO RECEBER MENSAGEM: {}", e);
        }
      }

      // Envia para todos se o jogador acertou ou não o palpite
      for mut conexao in conexoes {
        conexao.write(resposta.as_bytes()).unwrap();
      }

      // Define próximo jogador
      if jog_vez_index+1 > jogadores.len()-1 {                                        // Se a lista de jogadores chegou no fim
        if mestre_index == 0 {                                                          // Se mestre é o primeiro
          jog_vez_index = 1;
        } else {                                                                        // Se mestre não é o primerio da lista
          jog_vez_index = 0;
        }
      } else {                                                                        // Se a lista de jogadores não chegou ao fim
        if jog_vez_index+1 == mestre_index && !(jog_vez_index+2 > jogadores.len()-1) {  // Se próximo é o mestre e tem mais gente depois dele
          jog_vez_index += 2;
        } else  if jog_vez_index+1 != mestre_index {                                    // Se o próximo não é o mestre
          jog_vez_index += 1;
        } else {                                                                        // Se próximo é o mestre e é o último da lista
          jog_vez_index = 0;
        }
      }
    }

    // Atualiza arquivo de pontos
    le_arquivo::escreve_arquivo(score.clone());
    for mut conexao in conexoes {                                                 // Envia para todos o highscore
      conexao.write(le_arquivo::imprime_score(score.clone()).as_bytes()).unwrap();
    }

    // Para cada conexão, pergunta se quer continuar
    for mut conexao in conexoes {
      conexao.write(b"continuar:").unwrap();
      match conexao.read(&mut data){
        Ok(size) => {
          continuam_str = from_utf8(&data[0..size]).unwrap().to_string();
          
          if continuam_str.contains("sim"){                                     // Se sim, incrementa número que desejam continuar
            continuam_int += 1;
          }
        },
        Err(e) => {
          println!("FALHA AO RECEBER MENSAGEM: {}", e);
        }
      }   
    }

    if continuam_int == jogadores.len() {                                         // Se todos desejam continuar, inicia nova partida
      if mestre_index == jogadores.len() - 1 {
        mestre_index = 0;
      } else {
        mestre_index += 1;
      }
      nova_rodada = true;
      rodada_ativa = true;
    } else {                                                                      // Se não, finaliza jogo
      nova_rodada = false;
      rodada_ativa = false;
    }
  }

  // Avisa para todos o término do jogo
  for mut conexao in conexoes {
      conexao.write(b"fim").unwrap();
  }
  println!("--------------- JOGO FINALIZADO ---------------");
}


fn main() {
    let mut conexoes = vec![];
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("SERVIDOR DE JOGO ATIVO: PORTA [3333]");
    println!("Aguardando jogadores...");
    println!("O jogo se inicia quando [3] jogadores estiverem conectados...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("NOVA CONEXÃO: {}", stream.peer_addr().unwrap());
                conexoes.push(stream);
                if conexoes.len() == 3 {
                    handle_client(&conexoes);
                }
            }
            Err(e) => {
                println!("UM ERRO OCORREU: {}", e);
            }
        }
    }
}