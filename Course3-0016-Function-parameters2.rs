fn main() {

  greeting("Ayhan");

  greeting("Bengü");

  // Hi Ayhan!      
  // Ceteris paribus
  // Hi Bengü!      
  // Ceteris paribus

}

fn greeting(name: &str) {

  println!("Hi {name}!");

  say_ceteris_paribus();
}

fn say_ceteris_paribus() {

  println!("Ceteris paribus")
}

