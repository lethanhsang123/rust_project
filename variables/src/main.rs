fn main() {
    // Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Declaring Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of constant: {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;
    {
        let y = "Inner String Value";
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of x is: {y}");
}
