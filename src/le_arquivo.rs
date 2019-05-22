use std::fs;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn le_arquivo() -> HashMap<String, String> {
    // 
    let mut score: HashMap<String, String> = HashMap::new();

    // Lê arquivo
    let contents = fs::read_to_string("highscore.txt")
        .expect("Something went wrong reading the file");

    // ---------------- PREENCHE HASH -----------------------
    let mut part = 1; // 1: jogador; 2: pontuação
    let mut jogador = String::new();
    let mut pontuacao = String::new();

    // Percorre arquivo
    for c in contents.chars() {
        if c != ' ' {
            if c == '-' {
                part = 2;
            }

            if part == 1 {          // Se jogador está sendo preenchido
                jogador.push(c);
            } else if c != '-' {    // Se pontuação está sendo preenchida
                pontuacao.push(c);
            }
        } else {
            part = 1;
            // println!("{} - {}", jogador, pontuacao);
            score.insert(jogador.clone(), pontuacao.clone());       // Insere jogador e pontuação no HashMap
            jogador = "".to_string();
            pontuacao = "".to_string();
        }
        // println!("{}", c);
    }

    score.clone()
}

pub fn imprime_score (score: HashMap<String, String>) -> String {
    let mut highscore: String = "### HighScore ###\nJogador\t\tPontos\n".to_owned();

    println!("### HighScore ###");
    println!("Jogador\t\tPontos");
    for (key, value) in &score {
        highscore.push_str(key);
        highscore.push_str("\t\t");
        highscore.push_str(value);
        highscore.push_str("\n");
        
        println!("{}:\t\t{}", key, value);
    }
    println!("#################");
    // println!("highscore: {}", highscore);

    highscore.push_str(":");
    highscore
}

fn gera_texto_score (score: HashMap<String, String>) -> String {
    let mut highscore: String = String::new();

    for (key, value) in &score {
        highscore.push_str(key.trim());
        highscore.push_str("-");
        highscore.push_str(value);
        highscore.push_str(" ");
    }
    highscore.push_str("fim");

    highscore
}

pub fn escreve_arquivo (score: HashMap<String, String>) {
    let path = Path::new("highscore.txt");
    let display = path.display();
    let highscore = gera_texto_score(score);

    // Abre o arquivo em modo write-only, retorna `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Escreve highscore no file, retorna `io::Result<()>`
    match file.write_all(highscore.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

// Apenas para testes
// fn main () {
//     let mut score: HashMap<String, String> = le_arquivo();
//     escreve_arquivo(score);
    

//     // let name = String::from("Carlos");
//     // let pontoxio = score.get(&name);
//     // println!("Pontos xio:{:#?}", score.get(&name));

//     // let mut tab_highscore: String = imprime_score(score);
// }