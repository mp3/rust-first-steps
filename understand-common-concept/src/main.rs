fn main() {
    let greeting = String::from("hello");
    let greeting_reference = &greeting;
    println!("{}", greeting);
    println!("{}", greeting_reference);
}
