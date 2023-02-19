const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("* Immutable and Mutable *");
    // immutable
    // let x = 5;
    // mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    // if immutable, then error
    x = 6;
    println!("The value of x is: {x}");

    println!();
    println!("* Constants *");
    println!("The constant is: {}", THREE_HOURS_IN_SECONDS);

    println!();
    println!("* Shadowing *");
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    // This will be error
    // spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
