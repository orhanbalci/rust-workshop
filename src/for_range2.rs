#![feature(step_by)]
fn main(){
    for i in (0..20).step_by(2) {
        match i{
            x if x % 2 == 0 => {println!("{} cift sayi",x)},
            y if y % 2 == 1 => {println!("{} tek sayi",y)},
            _ => {}
        };
    }
}
