fn main() {

  some_calculations(5, -3, 2);

  // Sum = 4
  // Multiplication = -30
  // Squared sum = 38      

}

fn some_calculations(a: i64, b: i64, c: i64) {

  let sum = a + b + c;

  let mult = a * b * c;

  let squared_sum = a * a + b * b + c * c;

  println!("Sum = {sum}");

  println!("Multiplication = {mult}");

  println!("Squared sum = {squared_sum}")
}


