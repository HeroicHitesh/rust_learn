fn main() {
    // Numeric Operations
    let sum = 5 + 10;               // addition
    let difference = 95.5 - 4.3;    // subtraction
    let product = 4 * 30;           // multiplication
    let quotient = 56.7 / 32.2;     // division
    let floored = 2 / 3;            // division
    let remainder = 43 % 5;         // remainder
    println!("Sum = {}, Difference = {}, Product = {}, Quotient = {}, Floored = {}, Remainder = {}", sum, difference, product, quotient, floored, remainder);

    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;            // Supports destructuring
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    let five_hundred = tup.0;       // Access tuple elements using period (.) followed by the index
    println!("The value of five_hundred is: {}", five_hundred);
    let _unit_tuples = ();           // The type is called the unit type and the value is called the unit value
    // Error: Cannot print the value of a unit type
    // println!("The value of unit_tuples is: {}", unit_tuples);

    // Arrays
    let _a = [3; 5];                     // An array of 5 elements all initialized to 3
    let a: [i32; 5] = [1, 2, 3, 4, 5];  // An array of 5 elements of type i32
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a_slice = &a[1..4];         // Supports slicing
    println!("The value of a_slice is: {:?}", a_slice);
    let first_month = months[0];    // Supports indexing
    let last_month = months[11];
    println!("The value of first_month and last_month are: {} and {} respectively", first_month, last_month);
}
