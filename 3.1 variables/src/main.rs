fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // Throws error if mut not used
    println!("The value of x is: {}", x);

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing != Mutable
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);
}
