use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn handle_client(conexoes: &Vec<TcpStream>) {
  let mut jogadores = vec![];
  let mut mestre_index = 0;
  let mut jog_vez_index = 1;
  let mut mestre;
  let mut conexaoMestre : &TcpStream;
  let mut conexaoVez : &TcpStream;
  let mut data = [0 as u8; 50];
  let mut tip = String::new();
  let mut answer = String::new();
  let mut resposta = String::new();
  let mut palpite = String::new();
  let mut ask = String::new();
  let mut jog_vez = String::new();
  let mut sn = String::new();
  let mut jogo_ativo = true;
  let mut rodada_ativa = true;

  for mut conexao in conexoes {
    conexao.write(b"nome_jogador").unwrap();
    match conexao.read(&mut data){
      Ok(size) => {
        let mut jogador = String::new();
        jogador = from_utf8(&data[0..size]).unwrap().to_string();
        jogadores.push(jogador); 
      },
      Err(e) => {
        println!("FALHA AO RECEBER MENSAGEM: {}", e);
      }
    }   
  }

  //envia mestre do jogo
  mestre = jogadores.get(mestre_index).unwrap();
  let mut mestre_enviar: String = "mestre_rodada: ".to_owned();
  let aux2: &str = mestre;
  mestre_enviar.push_str(aux2);
  for mut conexao in conexoes {
    conexao.write(mestre_enviar.as_bytes()).unwrap();
  }

  conexaoMestre = conexoes.get(mestre_index).unwrap();

  //pega dica
  match conexaoMestre.read(&mut data){
    Ok(size) => {
      tip = from_utf8(&data[0..size]).unwrap().to_string();
      println!("Dica recebida: {}", tip); 
    },
    Err(e) => {
      println!("FALHA AO RECEBER MENSAGEM: {}", e);
    }
  }

  match conexaoMestre.read(&mut data){
    Ok(size) => {
      answer = from_utf8(&data[0..size]).unwrap().to_string();
      println!("Resposta recebida: {}", answer); 
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
    conexaoVez = conexoes.get(jog_vez_index).unwrap();
    jog_vez = jogadores.get(jog_vez_index).unwrap().to_string();
    for mut conexao in conexoes {
      let mut vez = String::new();
      vez.push_str("jogador_vez:");
      vez.push_str(&jog_vez);
      conexao.write(vez.as_bytes()).unwrap();

    }

    match conexaoVez.read(&mut data){
      Ok(size) => {
        ask.clear();
        ask.push_str("ask:");
        ask.push_str(&from_utf8(&data[0..size]).unwrap().to_string());
        println!("{}", ask); 
      },
      Err(e) => {
        println!("FALHA AO RECEBER MENSAGEM: {}", e);
      }
    }

    conexaoMestre.write(ask.as_bytes()).unwrap();

    match conexaoMestre.read(&mut data){
      Ok(size) => {
        sn.clear();
        sn.push_str("sn:");
        sn.push_str(&from_utf8(&data[0..size]).unwrap().to_string());
        println!("Resposta do mestre {}", sn); 
      },
      Err(e) => {
        println!("FALHA AO RECEBER MENSAGEM: {}", e);
      }
    }
    
    conexaoVez.write(sn.as_bytes()).unwrap();

    match conexaoVez.read(&mut data){
      Ok(size) => {
        palpite.clear();
        palpite.push_str("palpite:");
        palpite.push_str(&from_utf8(&data[0..size]).unwrap().to_string());
        println!("{}", palpite);
      },
      Err(e) => {
        println!("FALHA AO RECEBER MENSAGEM: {}", e);
      }
    }

    conexaoMestre.write(palpite.as_bytes()).unwrap();

    match conexaoMestre.read(&mut data){
      Ok(size) => {
        resposta.clear();
        resposta.push_str("resposta:");
        resposta.push_str(&from_utf8(&data[0..size]).unwrap().to_string());
        println!("{}", resposta);
      },
      Err(e) => {
        println!("FALHA AO RECEBER MENSAGEM: {}", e);
      }
    }

    for mut conexao in conexoes {
      conexao.write(resposta.as_bytes()).unwrap();
    }

    if jog_vez_index+1 > jogadores.len()-1 {
      if mestre_index == 0 {
        jog_vez_index = 1;
        jog_vez = jogadores.get(jog_vez_index).unwrap().to_string();
      } else {
        jog_vez_index = 0;
        jog_vez = jogadores.get(jog_vez_index).unwrap().to_string(); 
      }
    } else {
      if jog_vez_index+1 == mestre_index && !(jog_vez_index+2 > jogadores.len()-1) {
        jog_vez_index += 2;
        jog_vez = jogadores.get(jog_vez_index).unwrap().to_string(); 
      } else  if jog_vez_index+1 != mestre_index {
        jog_vez_index += 1;
        jog_vez = jogadores.get(jog_vez_index).unwrap().to_string(); 
      } else {
        jog_vez_index = 0;
        jog_vez = jogadores.get(jog_vez_index).unwrap().to_string(); 
      }
    }

  }
}


fn main() {
    let mut conexoes = vec![];
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("SERVIDOR DE JOGO ATIVO: PORTA [3333]");
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