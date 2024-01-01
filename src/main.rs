#![allow(dead_code)]


//   In Rust,  const  and  static  are both used to define constants but they have some differences in 
//   their usage and behavior:                                                                         
                                                                                                  
//   1.  const : The  const  keyword is used to define a constant value or a constant function.        
//   Constants live for the entire duration of a program. They are evaluated at compile time, hence    
//   they can be used in any context that needs a compile-time constant, such as array lengths and match
//   arms. They don't have a fixed memory location in the program (i.e., they don't have an address).  
//   2.  static : The  static  keyword is used to define a global variable, similar to  static  in C++. 
//   static  variables have a fixed memory location and live for the entire duration of the program.   
//   These can be mutable if declared as  static mut , but accessing mutable static variables is unsafe
//   because it can lead to data races.                                                                
                                                                                                  
//   So the main difference is that  const  is for constant values or functions and  static  is for    
//   global variables. Also,  const  does not actually allocate any memory, but  static  does.         


static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;



fn main() {
    let n = 16;
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);

    println!("{} is {}", n, if n < THRESHOLD {"small"} else {"big"});
}
