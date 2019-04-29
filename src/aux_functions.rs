use std::io::{self, BufRead, Write};

//Arquivo para teste e criação de funções auxiliares

// Aprender:
//   Retorno de função
//   Lidar com strings
//   Funções callback (Ok e Err)
//   Importar funções de outro arquivo


//Faze leitura do nome do jogador
pub fn read_name() -> String {
  print!("Digite seu Nickname: ");
  io::stdout().flush().unwrap();
  
  let mut nome = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut nome).expect("Não deu pra ler o que você digitou");
  // while nome  {
  //   print!("Nickname inválido... Digite outro: ");
  //   io::stdout().flush().unwrap();
  //   stdin.lock().read_line(&mut nome).expect("Não deu pra ler o que você digitou");
  // }
  nome
}

//Converter buffer em string
