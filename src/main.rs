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

fn main() { // defining the main function
  loop_cons(); // calling function to implement loop construct
  while_cons(); // calling while function to implement a while loop
}



