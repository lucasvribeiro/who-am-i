mod aux_functions;

fn main() {
  let nome;
  nome = aux_functions::read_name();
  println!("{}, Seja Bem-Vindo ao QUEM SOU EU!", nome);
}
