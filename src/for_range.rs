fn main() {
    for i in 1..15 {
        match i {
            x if x % 2 == 0 => println!("{} cift sayi", x),
            y if y % 2 == 1 => println!("{} tek sayi", y),
            _ => {}
        };
    }
}
