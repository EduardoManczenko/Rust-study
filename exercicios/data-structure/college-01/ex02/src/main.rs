fn main() {
    let mut vetor: Vec<i32> = vec![1, 0, 3, 0, 5, 6, 0, 8, 9, 10, 0, 12, 13, 0, 15];
    println!("Vetor original: {:?}", vetor);

    vetor.retain(|&x| x != 0);
    println!("Vetor compactado: {:?}", vetor);
}
