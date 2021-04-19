fn main() {
  let some_number: Option<u8> = Some(7);
  match some_number {
      Some(7) => println!("That's my lucky number!"),
      _ => {},
  }

  if let Some(7) = some_number {
      println!("That's my lucky number!");
  }
}
