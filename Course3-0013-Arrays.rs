fn main() {

  let mut names = ["mustafa", "aygün", "hakan"];

  names[names.len() - 1] = "lorem";

  println!("{:?}", names); // ["mustafa", "aygün", "lorem"]

  let nums: [u8; 5];

  nums = [101, 3, 0, 12, 24];

  let years = [1990, 2000, 2010, 2020];

  let zeros = [0; 10];

  println!("Zeros array: {:?}", zeros) // Zeros array: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

}
