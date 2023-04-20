fn loop_cons() { // function to implment a 'loop' looping struture
  print!("\nPrinting 3 iterations using a 'loop' looping structure.\n"); // print tile message
  let mut incrementor = 0; // declaring mut to specify that value is mutable
  loop { // declaring a loop, loop body below
    println!("iteration {}", incrementor +1); // printing the iteration number
    incrementor += 1; // increment the incrementor
    if incrementor == 3 { // stop condition for loop
      break; // breaking out of loop once if condition is met 
    } 
  }
}
fn while_cons() { // function to implement a while loop
  print!("\nPrinting 3 iterations using a 'while' looping structure.\n"); // printing title message
  let mut incrementor = 0;  // declaring a incrementor to keep track of iterations
  while incrementor < 3 { // while incrementor is less than 4
    println!("iteration {}", incrementor + 1); // keep printing this
    incrementor += 1; // increment the incrementor each 
  }
}

fn for_cons() { // function to implement a for loop
  print!("\nPrinting 3 iterations using a 'for' looping structure.\n"); // printing title message
  let iterations = [1, 2, 3]; // creating an array of numbers to iterate over 
  for iteration in iterations.iter() { // for loop using .iter() function to iterate
    println!("iteration {}", iteration); // printing display message at each iteration 
  }
}

fn for_in_cons() { // defining a function to implement a for in loop
  println!("\nPrinting 3 iterations usign a 'for in' looping structure."); // printing title message 
  for number in 1..4 { // creating a for in loop specifying i want to iterate 3 times
    println!("iteration {}", number); // loop body to print message and iteration number 
  }
}

fn while_let_cons() { // defining a function to implement a while let loop 
  print!("\nPrinting 3 iterations using a 'while let' looping structure\n");
  let mut iterations = vec![1, 2, 3].into_iter(); // creating an iterator to iterate over a mutable vector 
  while let Some(iteration) = iterations.next() { // creating the while let to iterate over and extract from iterations
    println!("iteration {}", iteration); // displaying each extracted iteration
  }
}

fn signed_int_data_types() { // creating function that declares, assignes, and prints all signed int data types
  print!("\nPrinting the minimum values of all 5 signed integer data types\n");
  // declarations
  let (a, b, c, d, e): (i8, i16, i32, i64, i128); // 8, 16, 32, 64, 128 bit signed integers
  // assgignments
  (a, b, c, d, e) = (-128, -32768, -2147483648, -9223372036854775808, -170141183460469231731687303715884105728);
  // printing
  println!("8-bit   = {}\n16-bit  = {}\n32-bit  = {}\n64-bit  = {}\n128-bit = {}", a, b, c, d, e);
}

fn unsigned_int_data_types() { // creating a function that declares, assigns, and prints all unsignged data types
  print!("\nPrinting the maximum values of all unsigned integer data types")
}

fn main() { // defining the main function
  loop_cons(); // calling function to implement loop construct
  while_cons(); // calling while function to implement a while loop
  for_cons(); // calling for function to implement for loop
  for_in_cons(); // calling the for in function to implment a for in loop 
  while_let_cons(); // calling the while let function to implement a while let looping structure
  signed_int_data_types(); // calling the data types functions to print all data types 
}



