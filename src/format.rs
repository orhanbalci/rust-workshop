fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named argument example
    println!("{namedArgument}", namedArgument = "Orhan");

    // special formatting
    println!("{0} is binary {1:b}", 10, 10);
}
