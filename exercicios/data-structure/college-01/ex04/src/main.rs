fn main() {
    let mut matriz = [[0; 5]; 5];

    for i in 0..5{
        matriz[i][i] = 1;
    }

    for linha in matriz.iter(){
        for &valor in linha.iter(){
            print!("{}", valor);
        }
        println!();
    }
}
