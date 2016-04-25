fn main(){
    println!("{}", factorial(5));
}

fn factorial(n : u32) -> f32{
    (1..n+1).fold(1_f32,|accumulator, item|{
        match item{
            0 => 1 as f32,
            x => accumulator * x as f32,
        }
    })
}
