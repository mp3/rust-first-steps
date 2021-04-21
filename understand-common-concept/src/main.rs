fn print_greeting(message: &String) {
    println!("{}", message);
}

fn main() {
    let greeting = String::from("hello");
    print_greeting(&greeting)
}
