fn main() {
  let value = 0b110011100;

  println!("Value is {}", value) // Value is 412

}

/*
Let's find 412

 1 * 256 
 1 * 128 
 0 * 64 
 0 * 32
 1 * 16
 1 * 8
 1 * 4
 0 * 2
 0 * 1
 -------------
 256 + 128 + 16 + 8 + 4 = 412
*/
