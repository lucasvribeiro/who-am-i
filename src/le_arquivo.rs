use std::fs;
use std::collections::HashMap;

fn main() {
    // 
    let mut score = HashMap::new();

    // Preenche score estaticamente
    // score.insert("Daniel".to_string(), "4".to_string());
    // score.insert("Ashley".to_string(), "8".to_string());
    // score.insert("Katie".to_string(), "2".to_string());
    // score.insert("Robert".to_string(), "5".to_string());

    // Lê arquivo
    let contents = fs::read_to_string("highscore.txt")
        .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    // ---------------- PREENCHE HASH -----------------------
    let mut part = 1; // 1: jogador; 2: pontuação
    let mut jogador = String::new();
    let mut pontuacao = String::new();
    for c in contents.chars() {
        if c != ' ' {
            if c == '-' {
                part = 2;
            }

            if part == 1 {
                jogador.push(c);
            } else if c != '-' {
                pontuacao.push(c);
            }
        } else {
            part = 1;
            // println!("{} - {}", jogador, pontuacao);
            score.insert(jogador.clone(), pontuacao.clone());
            jogador = "".to_string();
            pontuacao = "".to_string();
        }
        // println!("{}", c);
    }

    let team_name = String::from("Carlos");
    let pontoxio = score.get(&team_name);
    println!("{:?}", pontoxio);
   
}