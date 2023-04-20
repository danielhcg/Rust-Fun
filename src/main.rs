pub mod loop_module {// module to store all looping functions
  pub fn loop_cons() {
      // function to implment a 'loop' looping struture
      print!("\nPrinting 3 iterations using a 'loop' looping structure.\n"); // print tile message
      let mut incrementor = 0; // declaring mut to specify that value is mutable
      loop {
          // declaring a loop, loop body below
          println!("iteration {}", incrementor + 1); // printing the iteration number
          incrementor += 1; // increment the incrementor
          if incrementor == 3 {
              // stop condition for loop
              break; // breaking out of loop once if condition is met
          }
      }
  }

  pub fn while_cons() {
      // function to implement a while loop
      print!("\nPrinting 3 iterations using a 'while' looping structure.\n"); // printing title message
      let mut incrementor = 0; // declaring a incrementor to keep track of iterations
      while incrementor < 3 {
          // while incrementor is less than 4
          println!("iteration {}", incrementor + 1); // keep printing this
          incrementor += 1; // increment the incrementor each
      }
  }

  pub fn for_cons() {
      // function to implement a for loop
      print!("\nPrinting 3 iterations using a 'for' looping structure.\n"); // printing title message
      let iterations = [1, 2, 3]; // creating an array of numbers to iterate over
      for iteration in iterations.iter() {
          // for loop using .iter() function to iterate
          println!("iteration {}", iteration); // printing display message at each iteration
      }
  }

  pub fn for_in_cons() {
      // defining a function to implement a for in loop
      println!("\nPrinting 3 iterations usign a 'for in' looping structure."); // printing title message
      for number in 1..4 {
          // creating a for in loop specifying i want to iterate 3 times
          println!("iteration {}", number); // loop body to print message and iteration number
      }
  }

  pub fn while_let_cons() {
      // defining a function to implement a while let loop
      print!("\nPrinting 3 iterations using a 'while let' looping structure\n"); // printint title message
      let mut iterations = vec![1, 2, 3].into_iter(); // creating an iterator to iterate over a mutable vector
      while let Some(iteration) = iterations.next() {
          // creating the while let to iterate over and extract from iterations
          println!("iteration {}", iteration); // displaying each extracted iteration
      }
  }
}

pub mod int_module { // creating a module to hold all int data type functions
  pub fn signed_int_data_types() {
    // creating function that declares, assignes, and prints all signed int data types
    print!("\nPrinting the minimum values of all SIGNED integer data types\n"); // title message
    let (a, b, c, d, e): (i8, i16, i32, i64, i128); // 8, 16, 32, 64, 128 bit signed integer declarations
    (a, b, c, d, e) = (
        -128,
        -32768,
        -2147483648,
        -9223372036854775808,
        -170141183460469231731687303715884105728,
    ); // assignments
    println!(
        "8-bit   = {}\n16-bit  = {}\n32-bit  = {}\n64-bit  = {}\n128-bit = {}",
        a, b, c, d, e
    ); // printing
}

  pub fn unsigned_int_data_types() {
    // creating a function that declares, assigns, and prints all unsignged data types
    print!("\nPrinting the maximum values of all UNSIGNED integer data types\n"); // printing title message
    let (f, g, h, i, j): (u8, u16, u32, u64, u128); // 8, 16, 32, 64, 128 bit unsigned integer declarations
    (f, g, h, i, j) = (
        127,
        32767,
        2147483647,
        9223372036854775807,
        170141183460469231731687303715884105727,
    ); // assignments
    println!(
        "8-bit   = {}\n16-bit  = {}\n32-bit  = {}\n64-bit  = {}\n128-bit = {}",
        f, g, h, i, j
    ); // printing
  }
}

pub mod bcfs_module { // module to hold bool_char_float functoin as well as the string funtion
  pub fn bool_char_float() {
    // creating a function to implement booleans, characters, and floats
    print!("\nPrinting a boolean, character, and float data type\n"); // printing title message
    let (k, l, m, n, o): (bool, bool, char, f32, f64); // declarations
    (k, l, m, n, o) = (true, false, 'A', std::f32::MAX, std::f64::MAX); // assignments
    println!("Bool         = {}\nBool         = {}\nchar         = {}\n32-bit float = {}\n64-bit float = {}", k, l, m, n, o);
    //printing everything
  }

  pub fn string_cons() {
    // creating a function to declare a string, assign a string variable a value, and concatenate
    print!("\nPrinting a concatenated string\n"); // title message
    // declarations
    let word_one: &str;
    let word_two: &str;
    let word_three;

    // assignments
    (word_one, word_two) = ("Hello ", "World");
    word_three = word_one.to_string() + word_two;
    println!("{}", word_three); // printing
  }
}

fn resize(len: u8, wid: u8) -> (u8, u8) { // multi param function that resizes a square using unsigned ints because distance can't be negatvie 
  let factor = 3; // factor to multiply by 
  let length = len * factor; // assign new variables to return
  let width = wid * factor;
  return (length, width); // returning calcualted variables
}

use loop_module::*; // To signify that you want to use all functions within the loop_module
use int_module::*; // to use the int module 
use bcfs_module::*; // to use all functions within the bcfs module 

fn main() {// defining the main function
  loop_cons(); // calling function to implement loop construct
  while_cons(); // calling while function to implement a while loop
  for_cons(); // calling for function to implement for loop
  for_in_cons(); // calling the for in function to implment a for in loop
  while_let_cons(); // calling the while let function to implement a while let looping structure
  signed_int_data_types(); // calling the data types functions to print all data types
  unsigned_int_data_types(); // calling the unsigned data type functoin to print all unsigned int data types
  bool_char_float(); // calling bool_char_float function to print rest of the data types
  string_cons(); // to dispaly a concatenated string
  
  let (length, width) = resize(6, 3); // calling function to assign varibles values 
  print!("The new dimensions of rectangle with length 6 and width 3 after a 3x increase is\nlegth: {}\nwidth: {}", length, width); // printing 
}
