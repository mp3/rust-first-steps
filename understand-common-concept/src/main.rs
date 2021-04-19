fn main() {
    let mut a_number = 10;
    let a_boolean = true;

    println!("the number is {}", a_number);
    a_number = 15;
    println!("and now the number is {}", a_number);
    let a_number = a_number * 2;
    println!("The number is {}", a_number);

    println!("the boolean is {}", a_boolean);

    let number: u32 = "42".parse().expect("Not a number!");
    println!("u32 number {}", number);

    // Number
    // Addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Substraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Interger Division
    println!("9 / 2 = {}", 9u32 / 2);

    // Float Division
    println!("9 / 2 = {}", 9.0 / 2.0);

    // Multiplication
    println!("3 * 6 = {}", 3 * 6);

    // Boolean
    let is_bigger = 1 > 4;
    println!("{}", is_bigger);
}
