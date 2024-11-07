fn main() {

  // name, id, department, salary

  let mut emp_1: (&str, u8, &str, f32) = ("Ayhan", 108, "HR", 6250.25);

  println!("Salary = {}", emp_1.3);

  emp_1.2 = "Sales";

  println!("Department = {}", emp_1.2);

  // Salary = 6250.25
  // Department = Sales

}

