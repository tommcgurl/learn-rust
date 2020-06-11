// Mutable variables
fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    shadowing_example();
}

// Shadowing
/*
 * Shadowing is different from marking a variable as mut,
 * because we’ll get a compile-time error if we accidentally
 * try to reassign to this variable without using the let keyword.
 * By using let, we can perform a few transformations on a value
 * but have the variable be immutable after those transformations
 * have been completed.
 */
fn shadowing_example() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x); // 12

    // We can even change the type of a variable with shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces); // 12
}
