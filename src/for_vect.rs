fn main() {
    let mut numbers = vec![1, 2, 3];
    println!("{:?} guncellemeden once", numbers);
    for num in &mut numbers {
        *num += 1;
    }
    println!("{:?} guncellemeden sonra", numbers)
}
