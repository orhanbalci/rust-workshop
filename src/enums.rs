

enum Person{
    Skinny,
    Fat,
    Height(i32),
    Weight(i32),
    Info { name : String, height : i32}
}

fn inspect(person : Person){
    match person {
        Person::Skinny => println!("Is skinny"),
        Person::Fat => println!("Is fat"),
        Person::Height(i) => println!("Has a height of {}",i),
        Person::Weight(i) => println!("Has a weight of {}", i),
        Person::Info{name , height} => {
            println!("{} is {} tall", name, height);
        },
    }
}

fn main(){
    let person = Person::Height(28);
    let dave = Person::Info{ name : "Dave".to_owned(), height : 72};

    inspect(person);
    inspect(dave);
}
