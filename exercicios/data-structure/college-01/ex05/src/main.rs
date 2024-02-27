fn main() {
    let mut matriz = [[0; 10]; 10];

    for i in 0..10 {
        for j in 0..10 {
            if i < j {
                matriz[i][j] = 2*i + 7*j;
            } else if i == j {
                matriz[i][j] = 3*i.pow(2);
            } else {
                matriz[i][j] = 4*i.pow(3) + 5*j.pow(2) + 1;
            }
        }
    }

    for linha in &matriz {
        for &valor in linha {
            print!("{:5} ", valor);
        }
        println!();
    }
}
