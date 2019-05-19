use std::io::{self, BufRead, Write};

//Arquivo para teste e criação de funções auxiliares


//Faze leitura do nome do jogador
pub fn read_name() -> String {
  print!("[ DIGITE SEU NICKNAME ] : ");
  io::stdout().flush().unwrap();
  
  let mut nome = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut nome).expect("Não deu pra ler o que você digitou");
  nome
}

pub fn read_tip() -> String {
  print!("[ INFORME A DICA ] : ");
  io::stdout().flush().unwrap();
  
  let mut tip = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut tip).expect("Não deu pra ler o que você digitou");
  tip
}

pub fn read_answer() -> String {
  print!("[ INFORME A RESPOSTA ] : ");
  io::stdout().flush().unwrap();
  
  let mut answer = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut answer).expect("Não deu pra ler o que você digitou");
  answer
}

pub fn read_ask() -> String {
  print!("[ FAÇA SUA PERGUNTA ] : ");
  io::stdout().flush().unwrap();
  
  let mut ask = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut ask).expect("Não deu pra ler o que você digitou");
  ask
}

pub fn read_sn() -> String {
  print!("[ Resposta <s/n> ] : ");
  io::stdout().flush().unwrap();
  
  let mut sn = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut sn).expect("Não deu pra ler o que você digitou");
  sn
}

pub fn read_palpite() -> String {
  print!("[ DIGITE SEU PALPITE ] : ");
  io::stdout().flush().unwrap();
  
  let mut palpite = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut palpite).expect("Não deu pra ler o que você digitou");
  palpite
}

pub fn read_resposta() -> String {
  print!("[ RESPOSTA <certo/errado> ] : ");
  io::stdout().flush().unwrap();
  
  let mut resposta = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut resposta).expect("Não deu pra ler o que você digitou");
  resposta
}
