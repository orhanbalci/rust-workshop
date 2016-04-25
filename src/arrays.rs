use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of slice is {}", slice[0]);
    println!("length of slice is {}", slice.len());
}

fn update_slice(slice: &mut [i32]) {
    for e in slice {
        *e += 3;
    }
}

fn main() {
    let mut xs: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let ys: [i32; 100] = [0; 100];

    analyze_slice(&ys);
    analyze_slice(&xs[1..4]);

    println!("Guncellemeden once {:?}", xs);
    update_slice(&mut xs[0..3]);
    println!("Guncellemeden sonra {:?}", xs)

}
