use std::io;
fn main(){
    let mut reader = io::stdin();
    let mut control : String = String::new();
    loop {
        reader.read_line(&mut control);
        control.trim();
        if control.eq("quit\r\n") {
            println!("adios!");
            break;
        }else{
            println!("{}",control);
        }

        control.clear();
    }
}
