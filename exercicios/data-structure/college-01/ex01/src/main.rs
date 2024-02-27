use std::io;

fn main() {
    let mut vetor_x: Vec<f64> = Vec::new();
    let mut vetor_y: Vec<f64> = Vec::new();
    let mut produto_escalar = 0.0;


    println!("digite 5 numeros para o vetor X: ");
    for _ in 0..5 {
        let mut numero = String::new();
        io::stdin().read_line(&mut numero).unwrap();
        let numero: f64 = numero.trim().parse().unwrap();
        vetor_x.push(numero);
    }

    println!("digite 5 numeros para o vetor Y: ");
    for _ in 0..5 {
        let mut numero = String::new();
        io::stdin().read_line(&mut numero).unwrap();
        let numero: f64 = numero.trim().parse().unwrap();
        vetor_y.push(numero);
    }

    for i in 0..5 {
        produto_escalar += vetor_x[i] * vetor_y[i];
    }

    println!("VetorX: {:?}", vetor_x);
    println!("VetorY: {:?}", vetor_y);
    println!("Produto Escalar: {}", produto_escalar);
}
