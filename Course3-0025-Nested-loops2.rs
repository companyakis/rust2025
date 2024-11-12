fn main() {

  let mut births = [[1990, 1980, 2023], [2014, 2017, 2012], [1948, 1956, 1974]];

  // let's calculate current ages

  for row in births.iter_mut() {

    for birth_year in row.iter_mut() {

      *birth_year = 2024 - *birth_year;
      
      print!("{}\t", birth_year)
    }
    println!()
  }
}

/*

34      44      1
10      7       12
76      68      50


*/
