use std::io;
use std::collections::HashSet;

fn main() {
    let mut numeros = HashSet::new();
    println!("Digite 10 numeros: ");
    while numeros.len() < 10 {
        let mut numero = String::new();
        io::stdin().read_line(&mut numero).unwrap();
        let numero: i32 = match numero.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        if !numeros.insert(numero){
            println!("{} ja foi inserido", numero);
        }
    }

    let vetor: Vec<i32> = numeros.into_iter().collect();
    println!("{:?}", vetor);
}
