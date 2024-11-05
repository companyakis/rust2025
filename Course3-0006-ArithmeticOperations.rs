fn main() {

    let x: u8 = 12;

    let y: u16 = 5;

    let sum: u16 = x as u16 + y;

    let difference: u8 = x - y as u8;

    let mult: u16 = x as u16 * y;

    let div1: f32 = x as f32 / y as f32;

    let div2: f64 = x as f64 / y as f64;

    let modulus: u8 = x % y as u8;

    println!("{x} + {y} = {sum}");
    println!("{x} - {y} = {difference}");
    println!("{x} * {y} = {mult}");
    println!("{x} / {y} = {div1}");
    println!("{x} / {y} = {div2}");
    println!("{x} % {y} = {modulus}");

  // 12 + 5 = 17 
  // 12 - 5 = 7  
  // 12 * 5 = 60 
  // 12 / 5 = 2.4
  // 12 / 5 = 2.4
  // 12 % 5 = 2

}

