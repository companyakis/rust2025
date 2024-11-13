fn main() {

  // stack data

  let my_name: &str = "Mustafa Büyükdereli"; // immutable

  println!("My name is {my_name}");

  // heap data => can grow or shrink

  let mut what_i_want_to_say: String = String::from("Hi there!");

  println!("What I want to say is => {what_i_want_to_say}");

  what_i_want_to_say = String::from("Hi there! How are you?");

  println!("What I want to say is => {what_i_want_to_say}");

  what_i_want_to_say.push_str(" Take care...");

  println!("What I want to say is => {what_i_want_to_say}");

}

// My name is Mustafa Büyükdereli
// What I want to say is => Hi there!
// What I want to say is => Hi there! How are you?
// What I want to say is => Hi there! How are you? Take care...
