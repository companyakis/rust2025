fn main() {

  let people = ["ayhan", "mustafa", "aygün", "michael jordan", "kağan"];

  for (i, &person) in people.iter().enumerate() {

    if person == "michael jordan" {
      continue
    }

    println!("{person} is not a basketball player and his/her index is: {i}")
  }

}

// ayhan is not a basketball player and his/her index is: 0
// mustafa is not a basketball player and his/her index is: 1
// aygün is not a basketball player and his/her index is: 2  
// kağan is not a basketball player and his/her index is: 4  







