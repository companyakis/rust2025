// fn main() {
//     let celsius_temp = 23.0;
//     let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

//     assert_eq!(fahrenheit_temp, 73.4);
//     println!("Test passed!");
// }

// /* YOUR CODE GOES HERE */

fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

/*
°F = °C * 9/5 + 32
*/

fn celsius_to_fahrenheit(c_t: f64) -> f64 {

    let f_t = c_t * 1.8 + 32.0;

    f_t

}
