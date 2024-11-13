fn main() {

  // hep data!

  let greeting = String::from("Hi there!");

  let say_hi = greeting;

  //println!("{greeting}"); // Error! => value borrowed here after move

  println!("{say_hi}"); // Hi there!

}

