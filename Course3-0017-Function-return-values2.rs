fn main() {

  let result = sum_and_diff(-8988, 2198844);

  println!("Sum = {}", result.0);

  println!("Diff = {}", result.1);

  // Sum = 2189856  
  // Diff = -2207832 

}

fn sum_and_diff(x: i128, y: i128) -> (i128, i128) {

  let sum = x + y;

  let diff = x - y;

  return (sum, diff);
}


