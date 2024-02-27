use std::io;

fn main() {
    let mut matriz = [[0; 3]; 3];
    let mut soma_colunas = [0; 3];

    print!("digite os numeros da matriz 3x3: \n");

    for i in 0..3{
        for j in 0..3{
            let mut elemento = String::new();
            io::stdin().read_line(&mut elemento).unwrap();
            matriz[i][j] = elemento.trim().parse().unwrap();
            soma_colunas[j] += matriz[i][j];
        }
    }  

    println!("Array com a soma das colunas: {:?}", soma_colunas)
}
