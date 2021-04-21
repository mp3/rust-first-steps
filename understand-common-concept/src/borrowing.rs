fn change(text: &mut String) {
  text.push_str("!");
}

fn change(text: &mut String) {
  text.push_str("!");
}
fn main() {
  let greeting = String::from("hello");
  print_greeting(&greeting)

  let mut greeting = String::from("hello");
  change(&mut greeting)
}
