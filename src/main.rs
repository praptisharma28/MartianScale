use std::io;

fn main() {
    println!("Enter your weight on Earth in kg: ");
    let mut input = String::new(); //string is a type of smart pointer in rust
    // let mut s = input;

    //let s1 = &input; // s1 is a reference to input, it borrows the value of input
    //let s2 = &input; // s2 is another reference to input, we can
    //println!("{} and {}", s1, s2); // we can use both references to access the value of input
    // if we have already referenced as mutable, we cannot reference it as immutable again
    // but if we change it to mutable, then alsoit will result in an error, because we can have multipl immuable references, but only one mutable reference at a time
    //but the compiler lets us do that, because it knows that it will be mutated later, so it will not allow us to use the immutable references

    //some_func(&mut input); // passing a reference to the function, so we don't transfer ownership

    io::stdin().read_line(&mut input).unwrap();
    // unwrap() is used to handle the Result type, it will panic if there is an error
    // the read_line function mutably borrows the string and changes its content by filling it by input

    let weight: f64 = input.trim().parse().unwrap();
    // trim removes the whitespace from the beginning and end of the string
    // parse converts the string to a float

    let mars_weight = calculate_weight_on_mars(weight);
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

// fn some_func(s: &mut String) {
//     println!("The value of s is: {}", s);
// }
