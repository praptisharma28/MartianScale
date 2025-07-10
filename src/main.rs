use std::io;

fn main() {
    let mut input = String::new(); //string is a type of smart pointer in rust
    // let mut s = input;

    some_func(&mut input); // passing a reference to the function, so we don't transfer ownership


    println!("Enter your weight on Earth in kg: ");
    io::stdin().read_line(&mut input);
    let mars_weight = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {} kg", mars_weight);
}
// when the func exits, the drop fn on the string is called, which frees the memory, and is invoked automatically by the compiler
// the pattern of deallocating values at the end of their lifetime is called RAII (Resource Acquisition Is Initialization)

// double free when two owners try to free the same memory, it can lead to memory corruption   

fn calculate_weight_on_mars(weight: f64) -> f64 {
    (weight / 9.81) * 3.711
} 

// to be able to pass varibles to functions without transferring ownership, we can use references
// references allow us to borrow the value without taking ownership, we do that by adding an ampersand (&) before the variable name

fn some_func(s: &mut String) {
    println!("The value of s is: {}", s);
}