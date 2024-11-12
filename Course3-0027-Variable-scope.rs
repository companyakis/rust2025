fn main() {

  let number = 101;

  println!("Number is {number}");

  {
    println!("Number is really {number}");

    let year: u16 = 2024;

    println!("Year is {year}");
  }

  //println!("Year is really {year}"); // error[E0425]: cannot find value `year` in this scope

}

// Number is 101
// Number is really 101
// Year is 2024      
