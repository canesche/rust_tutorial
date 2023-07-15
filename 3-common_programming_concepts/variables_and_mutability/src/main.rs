fn main() {
    
    // mut x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; 
    println!("The value of x is: {x}");

    // constantes: work in all program
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Value of contant: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Size of space: {spaces}");
}
