fn main() {
    basic();
    number();
    boolean();
    char_and_string();
    tuple();
    instantiate();
}

fn basic() {
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
}

fn number() {
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
}

fn boolean() {
    // Boolean
    let is_bigger = 1 > 4;
    println!("{}", is_bigger);
}

fn char_and_string() {
    // Character and String
    // char
    let c = 'z';
    let z = 'Z';
    let heart_eyes_cat = '😻';
    println!("{} {} {}", c, z, heart_eyes_cat);

    // String
    let mut hello = String::from("Hello, ");
    hello.push('w');
    hello.push_str("orld!");
    println!("{}", hello);
}

fn tuple() {
    // Tuple
    let tuple = ("hello", 5, 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
}

// Struct
struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

struct Point2D(u32, u32);

struct Unit;

fn instantiate() {
    let person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25
    };

    let origin = Point2D(0, 0);

    let unit = Unit;
}


// Enum
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}
