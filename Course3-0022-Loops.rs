fn main() {

  let mut counter: u8 = 0;

  loop {

    counter += 1;

    if counter == 10 {
      continue
    }

    if counter > 15 {
      break
    }

    println!("Counter: {counter}")

  };
  
}

// Counter: 1
// Counter: 2
// Counter: 3
// Counter: 4
// Counter: 5
// Counter: 6
// Counter: 7
// Counter: 8
// Counter: 9
// Counter: 11
// Counter: 12
// Counter: 13
// Counter: 14
// Counter: 15




