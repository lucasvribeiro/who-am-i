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

//Converter buffer em string
