fn main() {

  let number = 101;

  println!("Number is {number}");

  {
    println!("Number is {number}");

    let number: u16 = 2024;

    println!("Number is {number}");
  }

  let number = "Lorem 101";

  println!("Number is {number}");
}

// Number is 101
// Number is 101      
// Number is 2024     
// Number is Lorem 101
