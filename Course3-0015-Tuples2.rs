fn main() {

  // name, id, department, salary

  let emp_1: (&str, u8, &str, f32) = ("Ayhan", 108, "HR", 6250.25);

  let (name, id, department, salary) = emp_1;

  println!("Name: {}", name);

  println!("ID: {}", id);

  println!("Department: {}", department);

  println!("Salary: {}", salary);

}

// Name: Ayhan
// ID: 108        
// Department: HR 
// Salary: 6250.25

